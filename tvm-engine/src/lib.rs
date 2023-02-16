mod engine;
mod error;
mod types;

// todo add compile macro control
mod c_interface;

pub(crate) use error::{EngineError, EngineErrorEnum};
pub(crate) use types::{CallArgs, ReturnResult, TransactionStatus};
