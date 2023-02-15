use evm::backend::Log;
use tvm_engine_types::{uTop, Address};

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
