use crate::{Precompile, TvmPrecompileOutput};
use evm::ExitError;
use std::borrow::Cow::Borrowed;
use tvm_engine_types::{Address, Gas};

pub(super) const IDENTITY_BASE: Gas = Gas::new(15);
pub(super) const IDENTITY_PER_WORD: Gas = Gas::new(3);
pub(super) const IDENTITY_WORD_LEN: u64 = 32;

pub struct Identity;

impl Identity {
    pub const ADDRESS: Address = super::make_address(0, 4);
}

impl Precompile for Identity {
    fn required_gas(input: &[u8]) -> Result<Gas, ExitError> {
        let input_len = u64::try_from(input.len()).map_err(|_| ExitError::Other(Borrowed("ERR_USIZE_CONV")))?;
        Ok((input_len + IDENTITY_WORD_LEN - 1) / IDENTITY_WORD_LEN * IDENTITY_PER_WORD + IDENTITY_BASE)
    }

    fn run(
        &self,
        input: &[u8],
        target_gas: Option<Gas>,
        _context: &evm::Context,
        _is_static: bool,
    ) -> Result<TvmPrecompileOutput, ExitError> {
        let cost = Self::required_gas(input)?;
        if let Some(target_gas) = target_gas {
            if cost > target_gas {
                return Err(ExitError::OutOfGas);
            }
        }
        Ok(TvmPrecompileOutput::without_logs(cost, input.to_vec()))
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    fn new_context() -> evm::Context {
        evm::Context {
            address: Default::default(),
            caller: Default::default(),
            apparent_value: Default::default(),
        }
    }

    #[test]
    fn test_identity() {
        let input = [0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let res = Identity
            .run(&input, Some(Gas::new(100)), &new_context(), false)
            .unwrap()
            .output;
        assert_eq!(input.to_vec(), res);
    }
}
