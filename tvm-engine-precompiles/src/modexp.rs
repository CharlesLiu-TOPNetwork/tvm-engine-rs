use crate::{Precompile, TvmPrecompileOutput};
use evm::ExitError;
use num::{BigUint, Integer};
use std::borrow::Cow::Borrowed;
use tvm_engine_types::{Address, Gas, U256};

pub struct ModExp;

impl ModExp {
    pub const ADDRESS: Address = super::make_address(0, 5);

    fn calc_iter_count(exp_len: u64, base_len: u64, bytes: &[u8]) -> Result<U256, ExitError> {
        let start = usize::try_from(base_len).map_err(|_| ExitError::Other(Borrowed("ERR_USIZE_CONV")))?;
        let exp_len = usize::try_from(exp_len).map_err(|_| ExitError::Other(Borrowed("ERR_USIZE_CONV")))?;
        let exp = parse_bytes(bytes, start.saturating_add(96), core::cmp::min(32, exp_len), |x| {
            U256::from(x)
        });

        if exp_len <= 32 && exp.is_zero() {
            Ok(U256::zero())
        } else if exp_len <= 32 {
            Ok(U256::from(exp.bits()) - U256::from(1))
        } else {
            // else > 32
            Ok(U256::from(8) * U256::from(exp_len - 32) + U256::from(exp.bits()) - U256::from(1))
        }
    }
    fn run_inner(input: &[u8]) -> Result<Vec<u8>, ExitError> {
        let (base_len, exp_len, mod_len) = parse_lengths(input);
        let base_len = usize::try_from(base_len).map_err(|_| ExitError::Other(Borrowed("ERR_USIZE_CONV")))?;
        let exp_len = usize::try_from(exp_len).map_err(|_| ExitError::Other(Borrowed("ERR_USIZE_CONV")))?;
        let mod_len = usize::try_from(mod_len).map_err(|_| ExitError::Other(Borrowed("ERR_USIZE_CONV")))?;

        let base_start = 96;
        let base_end = base_len.saturating_add(base_start);

        let exp_start = base_end;
        let exp_end = exp_len.saturating_add(exp_start);

        let mod_start = exp_end;

        let base = parse_bytes(input, base_start, base_len, BigUint::from_bytes_be);
        let exponent = parse_bytes(input, exp_start, exp_len, BigUint::from_bytes_be);
        let modulus = parse_bytes(input, mod_start, mod_len, BigUint::from_bytes_be);

        let output = {
            let computed_result = if modulus == BigUint::from(0u32) {
                Vec::new()
            } else {
                base.modpow(&exponent, &modulus).to_bytes_be()
            };
            // The result must be the same length as the input modulus.
            // To ensure this we pad on the left with zeros.
            if mod_len > computed_result.len() {
                let diff = mod_len - computed_result.len();
                let mut padded_result = Vec::with_capacity(mod_len);
                padded_result.extend(core::iter::repeat(0).take(diff));
                padded_result.extend_from_slice(&computed_result);
                padded_result
            } else {
                computed_result
            }
        };

        Ok(output)
    }
    // output bounded by 2^122
    fn mul_complexity(base_len: u64, mod_len: u64) -> U256 {
        let max_len = core::cmp::max(mod_len, base_len);
        let words = U256::from(Integer::div_ceil(&max_len, &8));
        words * words
    }
}

impl Precompile for ModExp {
    fn required_gas(input: &[u8]) -> Result<Gas, ExitError> {
        let (base_len, exp_len, mod_len) = parse_lengths(input);

        let mul = Self::mul_complexity(base_len, mod_len);
        let iter_count = Self::calc_iter_count(exp_len, base_len, input)?;
        // mul * iter_count bounded by 2^189 (so no overflow)
        let gas = mul * iter_count.max(U256::one()) / U256::from(3);

        Ok(Gas::new(core::cmp::max(200, saturating_round(gas))))
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

        let output = Self::run_inner(input)?;
        Ok(TvmPrecompileOutput::without_logs(cost, output))
    }
}

fn parse_bytes<T, F: FnOnce(&[u8]) -> T>(input: &[u8], start: usize, size: usize, f: F) -> T {
    let len = input.len();
    if start >= len {
        return f(&[]);
    }
    let end = start + size;
    if end > len {
        // Pad on the right with zeros if input is too short
        let bytes: Vec<u8> = input[start..]
            .iter()
            .copied()
            .chain(core::iter::repeat(0u8))
            .take(size)
            .collect();
        f(&bytes)
    } else {
        f(&input[start..end])
    }
}

fn saturating_round(x: U256) -> u64 {
    if x.bits() > 64 {
        u64::MAX
    } else {
        x.as_u64()
    }
}

fn parse_lengths(input: &[u8]) -> (u64, u64, u64) {
    let parse = |start: usize| -> u64 { saturating_round(parse_bytes(input, start, 32, |x| U256::from(x))) };
    let base_len = parse(0);
    let exp_len = parse(32);
    let mod_len = parse(64);

    (base_len, exp_len, mod_len)
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

    struct Test {
        input: &'static str,
        expected: &'static str,
        name: &'static str,
    }

    const TESTS: [Test; 2] = [
        Test {
            input: "\
            0000000000000000000000000000000000000000000000000000000000000001\
            0000000000000000000000000000000000000000000000000000000000000001\
            0000000000000000000000000000000000000000000000000000000000000001\
            08\
            09\
            0a",
            expected: "08",
            name: "test_0",
        },
        Test {
            input: "\
            0000000000000000000000000000000000000000000000000000000000000001\
            0000000000000000000000000000000000000000000000000000000000000020\
            0000000000000000000000000000000000000000000000000000000000000020\
            03\
            fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2e\
            ffffffffffffffffffffffffffffffffffffffffff2f",
            expected: "162ead82cadefaeaf6e9283248fdf2f2845f6396f6f17c4d5a39f820b6f6b5f9",
            name: "test_1",
        },
    ];

    const GAS: [Gas; 2] = [Gas::new(200), Gas::new(1360)];

    #[test]
    fn test_modexp() {
        for (test, test_gas) in TESTS.iter().zip(GAS.iter()) {
            let input = hex::decode(test.input).unwrap();

            let res = ModExp
                .run(&input, Some(*test_gas), &new_context(), false)
                .unwrap()
                .output;
            let expected = hex::decode(test.expected).unwrap();
            assert_eq!(res, expected, "{}", test.name);
        }
    }
}
