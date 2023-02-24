mod engine;
mod error;
mod types;

/// should enable this condition feature:
/// but will casue too many `dead_code` as we don't have standalone test for now
// #[cfg(feature = "build_as_xtop_lib")]
mod c_interface;

pub(crate) use error::{EngineError, EngineErrorEnum};
pub(crate) use types::{CallArgs, ReturnResult, TransactionStatus};
