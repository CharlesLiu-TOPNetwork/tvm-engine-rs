use std::cell::RefCell;

use evm::backend::{Apply, ApplyBackend, Backend};
use tvm_engine_precompiles::Precompiles;
use tvm_engine_runtime::{env::Env, io::IO, methods::*, utils, DupCache, PairDupCache};
use tvm_engine_types::{uTop, Address, H256, U256};

use crate::{CallArgs, EngineError, EngineErrorEnum, ReturnResult, TransactionStatus};

struct StackExecutorParams {
    precompiles: Precompiles,
    gas_limit: u64,
}

// todo we can edit config
const CONFIG: &evm::Config = &evm::Config::london();

impl StackExecutorParams {
    fn new(gas_limit: u64) -> Self {
        Self {
            precompiles: Precompiles::new(),
            gas_limit,
        }
    }
    fn make_executor<'a, 'env, I, E>(
        &'a self,
        engine: &'a Engine<'env, I, E>,
    ) -> evm::executor::stack::StackExecutor<
        'static,
        'a,
        evm::executor::stack::MemoryStackState<Engine<'env, I, E>>,
        Precompiles,
    >
    where
        I: IO,
        E: Env,
    {
        let metadata = evm::executor::stack::StackSubstateMetadata::new(self.gas_limit, CONFIG);
        let state = evm::executor::stack::MemoryStackState::new(metadata, engine);
        evm::executor::stack::StackExecutor::new_with_precompiles(state, CONFIG, &self.precompiles)
    }
}

pub struct Engine<'env, I, E> {
    io: I,
    env: &'env E,
    account_info_cache: RefCell<DupCache<Address, evm::backend::Basic>>,
    storage_cache: RefCell<PairDupCache<Address, H256, H256>>,
}

/// convert `evm::ExitReason` into `Result<TransactionStatus, EngineErrorEnum>`
///
/// make it easier match `Ok(TransactionStatus) => Ok(ReturnResult)`, and `Err(EngineErrorEnum) => Err(EngineError)`
///
/// which is `Result<ReturnResult, EngineError>` aka `EngineResult`
trait EvmExitIntoResult {
    fn into_result(self, data: Vec<u8>) -> Result<TransactionStatus, EngineErrorEnum>;
}

impl EvmExitIntoResult for evm::ExitReason {
    fn into_result(self, data: Vec<u8>) -> Result<TransactionStatus, EngineErrorEnum> {
        use evm::ExitError;
        use evm::ExitReason::*;
        match self {
            Succeed(_) => Ok(TransactionStatus::Succeed(data)),
            Revert(_) => Ok(TransactionStatus::Revert(data)),
            Error(ExitError::OutOfFund) => Ok(TransactionStatus::OutOfFund),
            Error(ExitError::OutOfGas) => Ok(TransactionStatus::OutOfGas),
            Error(ExitError::OutOfOffset) => Ok(TransactionStatus::OutOfOffset),
            Error(e) => Err(e.into()),
            Fatal(e) => Err(e.into()),
        }
    }
}

pub(crate) type EngineResult = Result<ReturnResult, EngineError>;

impl<'env, I, E> Engine<'env, I, E>
where
    I: IO,
    E: Env,
{
    pub(crate) fn new(io: I, env: &'env E) -> Self {
        Self {
            io,
            env,
            account_info_cache: RefCell::new(DupCache::default()),
            storage_cache: RefCell::new(PairDupCache::default()),
        }
    }

    pub(crate) fn call(&mut self, args: CallArgs) -> EngineResult {
        let caller = args.sender_addr;
        let target = args.recver_addr;
        if target.is_zero() {
            // deploy contract
            self.deploy_code(caller, args.value, args.input, args.gas_limit)
        } else {
            // call contract
            self.call_contract(caller, target, args.value, args.input, args.gas_limit)
        }
    }

    fn deploy_code(&mut self, caller: Address, value: uTop, input: Vec<u8>, gas_limit: u64) -> EngineResult {
        // 1. make evm executor
        let executor_params = StackExecutorParams::new(gas_limit);
        let mut executor = executor_params.make_executor(self);

        // 2. calc contract address
        // 2.1 code hash
        let code_hash: H256 = H256::from_slice(utils::sha256(&input).as_slice());

        // 2.2 nonce hash as salt begin value
        let nonce = self.basic(caller.raw()).nonce;
        let mut temp_bytes = Vec::new();
        nonce.to_big_endian(&mut temp_bytes);
        let mut salt_value = H256::from_slice(utils::sha256(&temp_bytes).as_slice());

        // 2.3 target table id
        let caller_table_id = caller.get_top_address_tableid();

        // 2.4 loop to calc the same table id contract address
        let contract_address = loop {
            let contract_address = executor.create_address(evm::CreateScheme::Create2 {
                caller: caller.raw(),
                code_hash,
                salt: salt_value,
            });
            if Address::build_from_hash160(contract_address).get_top_address_tableid() == caller_table_id {
                utils::log(
                    format!(
                        "generate contract_address at nonce:{}, address:{}",
                        nonce, contract_address
                    )
                    .as_str(),
                );
                break contract_address;
            }
            // salt_hash_value++
            salt_value = H256::from_slice({
                let u256_value = U256::from_big_endian(salt_value.as_bytes())
                    .overflowing_add(U256::from(1))
                    .0;
                u256_value.to_big_endian(&mut temp_bytes);
                temp_bytes.as_slice()
            });
        };

        // 3. execute tx
        let (exit_reason, return_value) =
            executor.transact_create(caller.raw(), value.into_wei_raw(), input, gas_limit, Vec::new());
        let result = if exit_reason.is_succeed() {
            // todo test this branch if is same with `return_value`
            contract_address.0.to_vec()
        } else {
            utils::log(format!("deploy_code failed: {:?}", exit_reason).as_str());
            return_value
        };

        // 4. get tx status or engine error.
        let used_gas = executor.used_gas();
        let status = match exit_reason.into_result(result) {
            Ok(status) => status,
            Err(engine_error) => {
                increment_nonce(&mut self.io, &caller);
                return Err(engine_error.with_gas_used(used_gas));
            }
        };

        // 5. apply changes && return result
        let (values, logs) = executor.into_state().deconstruct();

        self.apply(values, Vec::new(), true);

        Ok(ReturnResult::new(status, used_gas, logs.into_iter().collect()))
    }

    fn call_contract(
        &mut self,
        caller: Address,
        target: Address,
        value: uTop,
        input: Vec<u8>,
        gas_limit: u64,
    ) -> EngineResult {
        // 1. make evm executor
        let executor_params = StackExecutorParams::new(gas_limit);
        let mut executor = executor_params.make_executor(self);

        // 2. execute tx
        let (exit_reason, return_value) = executor.transact_call(
            caller.raw(),
            target.raw(),
            value.into_wei_raw(),
            input,
            gas_limit,
            Vec::new(),
        );

        // 3. get tx status or engine error
        let used_gas = executor.used_gas();
        let status = match exit_reason.into_result(return_value) {
            Ok(status) => status,
            Err(engine_error) => {
                increment_nonce(&mut self.io, &caller);
                return Err(engine_error.with_gas_used(used_gas));
            }
        };

        // 4. apply changes && return result
        let (values, logs) = executor.into_state().deconstruct();

        self.apply(values, Vec::new(), true);

        Ok(ReturnResult::new(status, used_gas, logs.into_iter().collect()))
    }
}

impl<'env, I, E> Backend for Engine<'env, I, E>
where
    I: IO,
    E: Env,
{
    /// current gas_price
    fn gas_price(&self) -> tvm_engine_types::U256 {
        self.env.gas_price()
    }

    /// origin address that create this call
    fn origin(&self) -> tvm_engine_types::H160 {
        self.env.origin().raw()
    }

    /// hash value of some block height
    fn block_hash(&self, _number: tvm_engine_types::U256) -> tvm_engine_types::H256 {
        // query real block hash is expensive, figure out some method to calc a fake but certain hash
        todo!()
    }

    /// current block height
    fn block_number(&self) -> tvm_engine_types::U256 {
        U256::from(self.env.block_height())
    }

    /// current consensus leader
    fn block_coinbase(&self) -> tvm_engine_types::H160 {
        self.env.block_coinbase().raw()
    }

    /// current block timestamp (Unix Timestamp)
    fn block_timestamp(&self) -> tvm_engine_types::U256 {
        U256::from(self.env.block_timestamp().secs())
    }

    fn block_difficulty(&self) -> tvm_engine_types::U256 {
        todo!()
    }

    fn block_gas_limit(&self) -> tvm_engine_types::U256 {
        todo!()
    }

    fn block_base_fee_per_gas(&self) -> tvm_engine_types::U256 {
        todo!()
    }

    /// current chain id
    fn chain_id(&self) -> tvm_engine_types::U256 {
        U256::from(self.env.chain_id())
    }

    /// check if a address exist
    fn exists(&self, address: tvm_engine_types::H160) -> bool {
        !is_account_empty(&self.io, &Address::build_from_hash160(address))
    }

    /// return one address' basic infomation (balance && nonce)
    fn basic(&self, address: tvm_engine_types::H160) -> evm::backend::Basic {
        let address = Address::build_from_hash160(address);
        self.account_info_cache
            .borrow_mut()
            .get_or_insert_with(&address, || evm::backend::Basic {
                nonce: get_nonce(&self.io, &address),
                balance: get_balance(&self.io, &address).into_wei_raw(),
            })
            .clone()
    }

    /// return the code of some address
    fn code(&self, address: tvm_engine_types::H160) -> Vec<u8> {
        get_code(&self.io, &Address::build_from_hash160(address))
    }

    /// get storage of some address at some index
    fn storage(&self, address: tvm_engine_types::H160, index: tvm_engine_types::H256) -> tvm_engine_types::H256 {
        let address = Address::build_from_hash160(address);
        self.storage_cache
            .borrow_mut()
            .get_or_insert_with((&address, &index), || get_storage(&self.io, &address, &index))
            .clone()
    }

    /// Get original storage value of address at index, if available.
    ///
    /// Since SputnikVM collects storage changes in memory until the transaction is over,
    /// the "original storage" will always be the same as the storage because no values
    /// are written to storage until after the transaction is complete.
    fn original_storage(
        &self,
        address: tvm_engine_types::H160,
        index: tvm_engine_types::H256,
    ) -> Option<tvm_engine_types::H256> {
        Some(self.storage(address, index))
    }
}

impl<'env, J, E> ApplyBackend for Engine<'env, J, E>
where
    J: IO,
    E: Env,
{
    fn apply<A, I, L>(&mut self, values: A, _logs: L, delete_empty: bool)
    where
        A: IntoIterator<Item = evm::backend::Apply<I>>,
        I: IntoIterator<Item = (H256, H256)>,
        L: IntoIterator<Item = evm::backend::Log>,
    {
        for apply in values {
            match apply {
                Apply::Modify {
                    address,
                    basic,
                    code,
                    storage,
                    reset_storage,
                } => {
                    let address = Address::build_from_hash160(address);
                    set_nonce(&mut self.io, &address, &basic.nonce);
                    set_balance(
                        &mut self.io,
                        &address,
                        &uTop::from_wei_value(basic.balance).unwrap_or(uTop::zero()),
                    );
                    if let Some(code) = code {
                        set_code(&mut self.io, &address, &code);
                        utils::log(format!("code write at {:?}, size:{}", address, code.len()).as_str());
                    }
                    if reset_storage {
                        remove_all_storage(&mut self.io, &address);
                    }
                    for (index, value) in storage {
                        if value == H256::default() {
                            remove_storage(&mut self.io, &address, &index);
                        } else {
                            // utils::log(format!("set_storage {:?}, {:?}",hex::encode(index.as_bytes()),hex::encode(value.bytes())).as_str());
                            set_storage(&mut self.io, &address, &index, &value);
                        }
                    }
                    if delete_empty && is_account_empty(&self.io, &address) {
                        remove_account(&mut self.io, &address);
                    }
                }
                Apply::Delete { address } => {
                    let address = Address::build_from_hash160(address);
                    remove_account(&mut self.io, &address);
                }
            }
        }
    }
}
