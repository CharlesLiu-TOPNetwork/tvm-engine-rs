use crate::{Precompile, TvmPrecompileOutput};
use evm::ExitError;
use tvm_engine_types::{Address, Gas, H256};

pub(super) const ECRECOVER_BASE: Gas = Gas::new(3000);
pub(super) const INPUT_LEN: usize = 128;

pub(super) struct ECRecover;

impl ECRecover {
    pub(super) const ADDRESS: Address = super::make_address(0, 1);
}

impl Precompile for ECRecover {
    fn required_gas(_input: &[u8]) -> Result<Gas, ExitError> {
        Ok(ECRECOVER_BASE)
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

        let mut input = input.to_vec();

        input.resize(INPUT_LEN, 0);

        let mut hash = [0; 32];
        hash.copy_from_slice(&input[0..32]);

        let mut v = [0; 32];
        v.copy_from_slice(&input[32..64]);

        let mut signature = [0; 65]; // signature is (r, s, v), typed (uint256, uint256, uint8)
        signature[0..32].copy_from_slice(&input[64..96]); // r
        signature[32..64].copy_from_slice(&input[96..128]); // s

        let v_bit = match v[31] {
            27 | 28 if v[..31] == [0; 31] => v[31] - 27,
            _ => {
                return Ok(TvmPrecompileOutput::without_logs(cost, vec![255u8; 32]));
            }
        };
        signature[64] = v_bit; // v

        let address_res = ecrevocer(H256::from_slice(&hash), &signature);
        let output = match address_res {
            Ok(a) => {
                let mut output = [0u8; 32];
                output[12..32].copy_from_slice(a.as_slice());
                output.to_vec()
            }
            Err(_) => Vec::new(),
        };
        Ok(TvmPrecompileOutput::without_logs(cost, output))
    }
}

pub fn ecrevocer(hash: H256, signature: &[u8]) -> Result<Address, ExitError> {
    assert_eq!(signature.len(), 65);
    use sha3::Digest;
    use std::borrow::Cow::Borrowed;

    let hash = libsecp256k1::Message::parse_slice(hash.as_bytes()).unwrap();
    let v = signature[64];
    let signature = libsecp256k1::Signature::parse_standard_slice(&signature[0..64])
        .map_err(|_| ExitError::Other(Borrowed("ERR_ECRECOVER")))?;
    let bit = match v {
        0..=26 => v,
        _ => v - 27,
    };

    if let Ok(recovery_id) = libsecp256k1::RecoveryId::parse(bit) {
        if let Ok(public_key) = libsecp256k1::recover(&hash, &signature, &recovery_id) {
            // recover returns a 65-byte key, but addresses come from the raw 64-byte key
            let r = sha3::Keccak256::digest(&public_key.serialize()[1..]);
            return Address::build_from_slice(&r[12..])
                .map_err(|_| ExitError::Other(Borrowed("ERR_INCORRECT_ADDRESS")));
        }
    }

    Err(ExitError::Other(Borrowed("ERR_ECRECOVER")))
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_ecrecover() {
        let hash = hex::decode("456e9aea5e197a1f1af7a3e85a3212fa4049a3ba34c2289b4c860fc0b0c64ef3").unwrap();
        let mut signature = [0; 65];
        signature[0..32].copy_from_slice(
            hex::decode("9242685bf161793cc25603c231bc2f568eb630ea16aa137d2664ac8038825608")
                .unwrap()
                .as_slice(),
        );
        signature[32..64].copy_from_slice(
            hex::decode("4f8ae3bd7535248d0bd448298cc2e2071e56992d0774dc340c368ae950852ada")
                .unwrap()
                .as_slice(),
        );
        let v_bit = 28 - 27;
        signature[64] = v_bit; // v

        let address = ecrevocer(H256::from_slice(&hash), &signature).unwrap();
        assert_eq!(
            address,
            Address::build_from_str("7156526fbd7a3c72969b54f64e42c10fbb768c8a").unwrap()
        );
    }
}
