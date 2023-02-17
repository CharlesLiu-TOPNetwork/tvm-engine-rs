use crate::{Precompile, TvmPrecompileOutput};
use evm::ExitError;
use std::borrow::Cow::Borrowed;
use tvm_engine_types::{Address, Gas};

pub(super) const SHA256_BASE: Gas = Gas::new(60);
pub(super) const SHA256_PER_WORD: Gas = Gas::new(12);
pub(super) const SHA256_WORD_LEN: u64 = 32;

pub(super) struct SHA256;

impl SHA256 {
    pub const ADDRESS: Address = super::make_address(0, 2);
}

impl Precompile for SHA256 {
    fn required_gas(input: &[u8]) -> Result<Gas, ExitError> {
        let input_len = u64::try_from(input.len()).map_err(|_| ExitError::Other(Borrowed("ERR_USIZE_CONV")))?;
        Ok((input_len + SHA256_WORD_LEN - 1) / SHA256_WORD_LEN * SHA256_PER_WORD + SHA256_BASE)
    }

    fn run(
        &self,
        input: &[u8],
        target_gas: Option<Gas>,
        _context: &evm::Context,
        _is_static: bool,
    ) -> Result<TvmPrecompileOutput, ExitError> {
        use sha2::Digest;

        let cost = Self::required_gas(input)?;
        if let Some(target_gas) = target_gas {
            if cost > target_gas {
                return Err(ExitError::OutOfGas);
            }
        }

        let output = sha2::Sha256::digest(input).to_vec();
        Ok(TvmPrecompileOutput::without_logs(cost, output))
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_sha256() {
        use sha2::Digest;
        let input = hex::decode("ff").unwrap();
        assert_eq!(
            sha2::Sha256::digest(&input).to_vec(),
            hex::decode("a8100ae6aa1940d0b663bb31cd466142ebbdbd5187131b92d93818987832eb89").unwrap()
        )
    }
}
