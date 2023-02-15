pub struct EngineError {
    pub kind: EngineErrorEnum,
    pub gas_used: u64,
}

pub enum EngineErrorEnum {
    EvmError,
    EvmFatal,
}

impl EngineErrorEnum {
    pub fn with_gas_used(self, gas_used: u64) -> EngineError {
        EngineError { kind: self, gas_used }
    }
}
