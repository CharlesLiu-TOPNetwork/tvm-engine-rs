use crate::{Precompile, TvmPrecompileOutput};
use evm::ExitError;
use std::borrow::Cow::Borrowed;
use tvm_engine_types::{Address, Gas};

pub(super) const RIPEMD_BASE: Gas = Gas::new(600);
pub(super) const RIPEMD_PER_WORD: Gas = Gas::new(120);
pub(super) const RIPEMD_WORD_LEN: u64 = 32;

pub(super) struct Ripemd;

impl Ripemd {
    pub const ADDRESS: Address = super::make_address(0, 3);
}

impl Precompile for Ripemd {
    fn required_gas(input: &[u8]) -> Result<Gas, ExitError> {
        let input_len = u64::try_from(input.len()).map_err(|_| ExitError::Other(Borrowed("ERR_USIZE_CONV")))?;
        Ok((input_len + RIPEMD_WORD_LEN - 1) / RIPEMD_WORD_LEN * RIPEMD_PER_WORD + RIPEMD_BASE)
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

        use ripemd::{Digest, Ripemd160};
        let hash = {
            let hash = Ripemd160::digest(input);
            let mut output = [0u8; 20];
            output.copy_from_slice(&hash);
            output
        };
        let mut output = vec![0u8; 32];
        output[12..].copy_from_slice(&hash);
        Ok(TvmPrecompileOutput::without_logs(cost, output))
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_ripemd160() {
        use ripemd::{Digest, Ripemd160};
        let input = hex::decode("ff").unwrap();
        let hash = {
            let hash = Ripemd160::digest(&input);
            let mut output = [0u8; 20];
            output.copy_from_slice(&hash);
            output
        };
        assert_eq!(
            hash.to_vec(),
            hex::decode("2c0c45d3ecab80fe060e5f1d7057cd2f8de5e557").unwrap()
        )
    }
}
