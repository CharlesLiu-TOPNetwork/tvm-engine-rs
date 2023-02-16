use evm::{ExitError, ExitFatal};

pub struct EngineError {
    pub kind: EngineErrorEnum,
    pub gas_used: u64,
}

pub enum EngineErrorEnum {
    EvmError(ExitError),
    EvmFatal(ExitFatal),
}

impl EngineErrorEnum {
    pub fn with_gas_used(self, gas_used: u64) -> EngineError {
        EngineError { kind: self, gas_used }
    }

    pub fn as_bytes(&self) -> &[u8] {
        use EngineErrorEnum::*;
        match self {
            EvmError(ExitError::StackUnderflow) => b"ERR_STACK_UNDERFLOW",
            EvmError(ExitError::StackOverflow) => b"ERR_STACK_OVERFLOW",
            EvmError(ExitError::InvalidJump) => b"ERR_INVALID_JUMP",
            EvmError(ExitError::InvalidRange) => b"ERR_INVALID_RANGE",
            EvmError(ExitError::DesignatedInvalid) => b"ERR_DESIGNATED_INVALID",
            EvmError(ExitError::CallTooDeep) => b"ERR_CALL_TOO_DEEP",
            EvmError(ExitError::CreateCollision) => b"ERR_CREATE_COLLISION",
            EvmError(ExitError::CreateContractLimit) => b"ERR_CREATE_CONTRACT_LIMIT",
            EvmError(ExitError::OutOfOffset) => b"ERR_OUT_OF_OFFSET",
            EvmError(ExitError::OutOfGas) => b"ERR_OUT_OF_GAS",
            EvmError(ExitError::OutOfFund) => b"ERR_OUT_OF_FUND",
            EvmError(ExitError::Other(m)) => m.as_bytes(),
            EvmError(_) => unreachable!(), // unused misc
            EvmFatal(ExitFatal::NotSupported) => b"ERR_NOT_SUPPORTED",
            EvmFatal(ExitFatal::UnhandledInterrupt) => b"ERR_UNHANDLED_INTERRUPT",
            EvmFatal(ExitFatal::Other(m)) => m.as_bytes(),
            EvmFatal(_) => unreachable!(), // unused misc
        }
    }
}

impl From<ExitError> for EngineErrorEnum {
    fn from(value: ExitError) -> Self {
        Self::EvmError(value)
    }
}

impl From<ExitFatal> for EngineErrorEnum {
    fn from(value: ExitFatal) -> Self {
        Self::EvmFatal(value)
    }
}
