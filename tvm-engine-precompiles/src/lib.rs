use std::collections::BTreeMap;

use tvm_engine_types::{Address, Gas, H160};

use evm::{backend::Log, Context, ExitError};

/// [0x01. ecRecover](../README.md#0x01-ecrecover)
mod ecrecover;
use ecrecover::ECRecover;

#[derive(Debug, PartialEq, Eq)]
pub struct TvmPrecompileOutput {
    pub cost: Gas,
    pub output: Vec<u8>,
    pub logs: Vec<Log>,
}

impl TvmPrecompileOutput {
    pub fn without_logs(cost: Gas, output: Vec<u8>) -> Self {
        Self {
            cost,
            output,
            logs: Vec::new(),
        }
    }
}

pub trait Precompile {
    fn required_gas(input: &[u8]) -> Result<Gas, ExitError>
    where
        Self: Sized;

    fn run(
        &self,
        input: &[u8],
        target_gas: Option<Gas>,
        context: &Context,
        is_static: bool,
    ) -> Result<TvmPrecompileOutput, ExitError>;
}

pub struct Precompiles(pub BTreeMap<Address, Box<dyn Precompile>>);

impl evm::executor::stack::PrecompileSet for Precompiles {
    fn execute(
        &self,
        handle: &mut impl evm::executor::stack::PrecompileHandle,
    ) -> Option<Result<evm::executor::stack::PrecompileOutput, evm::executor::stack::PrecompileFailure>> {
        let r = self
            .0
            .get(&Address::build_from_hash160(handle.code_address()))
            .map(|p| {
                p.run(
                    handle.input(),
                    handle.gas_limit().map(Gas::new),
                    handle.context(),
                    handle.is_static(),
                )
                .map_err(|exit_status| evm::executor::stack::PrecompileFailure::Error { exit_status })
            });

        // convert `Result<TvmPrecompileOutput>` to `Result<PrecompileOutput>` for trait `PrecompileSet`
        r.map(|tpr| {
            tpr.and_then(|output| {
                handle.record_cost(output.cost.as_u64())?;
                for log in output.logs {
                    handle.log(log.address, log.topics, log.data)?;
                }
                Ok(evm::executor::stack::PrecompileOutput {
                    exit_status: evm::ExitSucceed::Returned,
                    output: output.output,
                })
            })
        })
    }

    fn is_precompile(&self, address: H160) -> bool {
        self.0.contains_key(&Address::build_from_hash160(address))
    }
}

impl Precompiles {
    // could add some arguments.
    pub fn new() -> Self {
        let addresses = vec![ECRecover::ADDRESS];
        let f: Vec<Box<dyn Precompile>> = vec![Box::new(ECRecover)];
        let map = addresses.into_iter().zip(f).collect();
        Precompiles(map)
    }
}

/// make a const address from `32 + 128 = 160`
pub const fn make_address(x: u32, y: u128) -> Address {
    let x_bytes = x.to_be_bytes();
    let y_bytes = y.to_be_bytes();
    Address::build_from_hash160(H160([
        x_bytes[0],
        x_bytes[1],
        x_bytes[2],
        x_bytes[3],
        y_bytes[0],
        y_bytes[1],
        y_bytes[2],
        y_bytes[3],
        y_bytes[4],
        y_bytes[5],
        y_bytes[6],
        y_bytes[7],
        y_bytes[8],
        y_bytes[9],
        y_bytes[10],
        y_bytes[11],
        y_bytes[12],
        y_bytes[13],
        y_bytes[14],
        y_bytes[15],
    ]))
}
