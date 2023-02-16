use evm::backend::Log;
use tvm_engine_runtime::utils::panic_utf8;
use tvm_engine_types::{uTop, Address, PU256};
use tvm_engine_types::{PCallArgs, PLog, PReturnResult};

/// Args from outside call.
pub(crate) struct CallArgs {
    pub sender_addr: Address,
    pub recver_addr: Address,
    pub value: uTop,
    pub input: Vec<u8>,
    pub gas_limit: u64,
}

/// Result that return back.
pub(crate) struct ReturnResult {
    status: u32,
    status_data: Vec<u8>,
    gas_used: u64,
    logs: Vec<Log>,
}

/// Execute result if no eninge error occur.
/// Returned by evm, than convert into return_result.status.
pub(crate) enum TransactionStatus {
    Succeed(Vec<u8>),
    Revert(Vec<u8>),
    OutOfGas,
    OutOfFund,
    OutOfOffset,
}

impl ReturnResult {
    pub(crate) fn new(tx_status: TransactionStatus, gas_used: u64, logs: Vec<Log>) -> Self {
        Self {
            status: tx_status.as_u32(),
            status_data: {
                if let TransactionStatus::Succeed(data) | TransactionStatus::Revert(data) = tx_status {
                    data
                } else {
                    Vec::new()
                }
            },
            gas_used,
            logs,
        }
    }
}

impl TransactionStatus {}

impl From<ReturnResult> for PReturnResult {
    fn from(value: ReturnResult) -> Self {
        Self {
            status: value.status,
            status_data: value.status_data,
            gas_used: value.gas_used,
            // thank god this conversion code is not that difficult to write :)
            logs: value
                .logs
                .into_iter()
                .map(|l| PLog {
                    address: Some(l.address.into()).into(),
                    topics: l
                        .topics
                        .into_iter()
                        .map(|t| PU256 {
                            data: t.as_bytes().to_vec(),
                            ..Default::default()
                        })
                        .collect(),
                    data: l.data,
                    ..Default::default()
                })
                .collect(),
            ..Default::default()
        }
    }
}

impl From<PCallArgs> for CallArgs {
    fn from(value: PCallArgs) -> Self {
        Self {
            sender_addr: value.sender_address.get_or_default().into(),
            recver_addr: value.recver_address.get_or_default().into(),
            value: value.value.into(),
            input: value.input,
            gas_limit: value.gas_limit,
        }
    }
}

impl TransactionStatus {
    pub(crate) fn as_u32(&self) -> u32 {
        match self {
            TransactionStatus::Succeed(_) => 0,
            TransactionStatus::Revert(_) => 1,
            TransactionStatus::OutOfGas => 2,
            TransactionStatus::OutOfFund => 3,
            TransactionStatus::OutOfOffset => 4,
        }
    }
}

pub trait EngineInterfaceExpect<T> {
    fn engine_interface_expect(self, msg: &str) -> T;
}
impl<T> EngineInterfaceExpect<T> for Option<T> {
    fn engine_interface_expect(self, msg: &str) -> T {
        match self {
            Some(t) => t,
            None => {
                panic_utf8(msg.as_ref());
            }
        }
    }
}
impl<T, E> EngineInterfaceExpect<T> for core::result::Result<T, E> {
    fn engine_interface_expect(self, msg: &str) -> T {
        match self {
            Ok(t) => t,
            Err(_) => panic_utf8(msg.as_ref()),
        }
    }
}
