use crate::{Precompile, TvmPrecompileOutput};
use evm::{Context, ExitError};
use std::borrow::Cow::Borrowed;
use tvm_engine_types::{Address, Gas};
use {zeropool_bn as bn, zeropool_bn::Group};

pub(super) const COST_ADD: Gas = Gas::new(150);
pub(super) const COST_MUL: Gas = Gas::new(6_000);
pub(super) const COST_PAIR_PER_POINT: Gas = Gas::new(34_000);
pub(super) const COST_PAIR_BASE: Gas = Gas::new(45_000);

/// Input length for the add operation.
pub(super) const ADD_INPUT_LEN: usize = 128;
/// Input length for the multiplication operation.
pub(super) const MUL_INPUT_LEN: usize = 128;
/// Pair element length.
pub(super) const PAIR_ELEMENT_LEN: usize = 192;
/// Size of BN points.
pub(super) const POINT_LEN: usize = 64;
/// Output length.
pub(super) const OUTPUT_LEN: usize = 64;

/// Size of BN scalars.
pub(super) const SCALAR_LEN: usize = 32;

pub(super) const ERR_BIG_ENDIAN: ExitError = ExitError::Other(Borrowed("ERR_BIG_ENDIAN"));

/// Reads the `x` and `y` points from an input at a given position.
fn read_point(input: &[u8], pos: usize) -> Result<bn::G1, ExitError> {
    use bn::{AffineG1, Fq, G1};

    let px =
        Fq::from_slice(&input[pos..(pos + SCALAR_LEN)]).map_err(|_e| ExitError::Other(Borrowed("ERR_FQ_INCORRECT")))?;
    let py = Fq::from_slice(&input[(pos + SCALAR_LEN)..(pos + SCALAR_LEN * 2)])
        .map_err(|_e| ExitError::Other(Borrowed("ERR_FQ_INCORRECT")))?;

    Ok(if px == Fq::zero() && py == Fq::zero() {
        G1::zero()
    } else {
        AffineG1::new(px, py)
            .map_err(|_| ExitError::Other(Borrowed("ERR_BN128_INVALID_POINT")))?
            .into()
    })
}

pub mod bn_add {
    use super::*;
    pub struct Bn256Add;

    impl Bn256Add {
        pub const ADDRESS: Address = super::super::make_address(0, 6);

        fn run_inner(input: &[u8], _context: &Context) -> Result<Vec<u8>, ExitError> {
            let mut input = input.to_vec();
            input.resize(ADD_INPUT_LEN, 0);

            let p1 = read_point(&input, 0)?;
            let p2 = read_point(&input, POINT_LEN)?;

            let output = Self::execute(p1, p2)?;
            Ok(output.to_vec())
        }

        fn execute(p1: bn::G1, p2: bn::G1) -> Result<[u8; OUTPUT_LEN], ExitError> {
            let mut output = [0u8; POINT_LEN];
            if let Some(sum) = bn::AffineG1::from_jacobian(p1 + p2) {
                sum.x()
                    .to_big_endian(&mut output[0..SCALAR_LEN])
                    .map_err(|_e| ERR_BIG_ENDIAN)?;
                sum.y()
                    .to_big_endian(&mut output[SCALAR_LEN..SCALAR_LEN * 2])
                    .map_err(|_e| ERR_BIG_ENDIAN)?;
            }
            Ok(output)
        }
    }

    impl Precompile for Bn256Add {
        fn required_gas(_input: &[u8]) -> Result<Gas, ExitError> {
            Ok(COST_ADD)
        }

        fn run(
            &self,
            input: &[u8],
            target_gas: Option<Gas>,
            context: &Context,
            _is_static: bool,
        ) -> Result<TvmPrecompileOutput, ExitError> {
            let cost = Self::required_gas(input)?;
            if let Some(target_gas) = target_gas {
                if cost > target_gas {
                    return Err(ExitError::OutOfGas);
                }
            }
            let output = Self::run_inner(input, context)?;
            Ok(TvmPrecompileOutput::without_logs(cost, output))
        }
    }
}

pub mod bn_mul {
    use super::*;
    pub struct Bn256Mul;

    impl Bn256Mul {
        pub const ADDRESS: Address = super::super::make_address(0, 7);

        fn run_inner(input: &[u8], _context: &Context) -> Result<Vec<u8>, ExitError> {
            let mut input = input.to_vec();
            input.resize(MUL_INPUT_LEN, 0);

            let p = read_point(&input, 0)?;
            let fr = bn::Fr::from_slice(&input[POINT_LEN..POINT_LEN + SCALAR_LEN])
                .map_err(|_e| ExitError::Other(Borrowed("ERR_BN128_INVALID_FR")))?;

            let output = Self::execute(p, fr)?;
            Ok(output.to_vec())
        }

        fn execute(p: bn::G1, fr: bn::Fr) -> Result<[u8; OUTPUT_LEN], ExitError> {
            let mut output = [0u8; OUTPUT_LEN];
            if let Some(mul) = bn::AffineG1::from_jacobian(p * fr) {
                mul.x()
                    .into_u256()
                    .to_big_endian(&mut output[0..SCALAR_LEN])
                    .map_err(|_e| ERR_BIG_ENDIAN)?;
                mul.y()
                    .into_u256()
                    .to_big_endian(&mut output[SCALAR_LEN..SCALAR_LEN * 2])
                    .map_err(|_e| ERR_BIG_ENDIAN)?;
            }
            Ok(output)
        }
    }

    impl Precompile for Bn256Mul {
        fn required_gas(_input: &[u8]) -> Result<Gas, ExitError> {
            Ok(COST_MUL)
        }

        fn run(
            &self,
            input: &[u8],
            target_gas: Option<Gas>,
            context: &Context,
            _is_static: bool,
        ) -> Result<TvmPrecompileOutput, ExitError> {
            let cost = Self::required_gas(input)?;
            if let Some(target_gas) = target_gas {
                if cost > target_gas {
                    return Err(ExitError::OutOfGas);
                }
            }

            let output = Self::run_inner(input, context)?;
            Ok(TvmPrecompileOutput::without_logs(cost, output))
        }
    }
}

pub mod bn_pair {
    use super::*;
    pub struct Bn256Pair;

    impl Bn256Pair {
        pub const ADDRESS: Address = super::super::make_address(0, 8);

        fn run_inner(input: &[u8], _context: &Context) -> Result<Vec<u8>, ExitError> {
            if input.len() % PAIR_ELEMENT_LEN != 0 {
                return Err(ExitError::Other(Borrowed("ERR_BN128_INVALID_LEN")));
            }

            let output = if input.is_empty() {
                bn::arith::U256::one()
            } else {
                let elements = input.len() / PAIR_ELEMENT_LEN;
                let mut vals = Vec::with_capacity(elements);
                for idx in 0..elements {
                    let ax =
                        bn::Fq::from_slice(&input[(idx * PAIR_ELEMENT_LEN)..(idx * PAIR_ELEMENT_LEN + SCALAR_LEN)])
                            .map_err(|_e| ExitError::Other(Borrowed("ERR_BN128_INVALID_AX")))?;
                    let ay = bn::Fq::from_slice(
                        &input[(idx * PAIR_ELEMENT_LEN + SCALAR_LEN)..(idx * PAIR_ELEMENT_LEN + SCALAR_LEN * 2)],
                    )
                    .map_err(|_e| ExitError::Other(Borrowed("ERR_BN128_INVALID_AY")))?;
                    let bay = bn::Fq::from_slice(
                        &input[(idx * PAIR_ELEMENT_LEN + SCALAR_LEN * 2)..(idx * PAIR_ELEMENT_LEN + SCALAR_LEN * 3)],
                    )
                    .map_err(|_e| ExitError::Other(Borrowed("ERR_BN128_INVALID_BAY")))?;
                    let bax = bn::Fq::from_slice(
                        &input[(idx * PAIR_ELEMENT_LEN + SCALAR_LEN * 3)..(idx * PAIR_ELEMENT_LEN + SCALAR_LEN * 4)],
                    )
                    .map_err(|_e| ExitError::Other(Borrowed("ERR_BN128_INVALID_BAX")))?;
                    let bby = bn::Fq::from_slice(
                        &input[(idx * PAIR_ELEMENT_LEN + SCALAR_LEN * 4)..(idx * PAIR_ELEMENT_LEN + SCALAR_LEN * 5)],
                    )
                    .map_err(|_e| ExitError::Other(Borrowed("ERR_BN128_INVALID_BBY")))?;
                    let bbx = bn::Fq::from_slice(
                        &input[(idx * PAIR_ELEMENT_LEN + SCALAR_LEN * 5)..(idx * PAIR_ELEMENT_LEN + SCALAR_LEN * 6)],
                    )
                    .map_err(|_e| ExitError::Other(Borrowed("ERR_BN128_INVALID_BBX")))?;

                    let g1_a = {
                        if ax.is_zero() && ay.is_zero() {
                            bn::G1::zero()
                        } else {
                            bn::AffineG1::new(ax, ay)
                                .map_err(|_e| ExitError::Other(Borrowed("ERR_BN128_INVALID_A")))?
                                .into()
                        }
                    };
                    let g1_b = {
                        let ba = bn::Fq2::new(bax, bay);
                        let bb = bn::Fq2::new(bbx, bby);

                        if ba.is_zero() && bb.is_zero() {
                            bn::G2::zero()
                        } else {
                            bn::AffineG2::new(ba, bb)
                                .map_err(|_e| ExitError::Other(Borrowed("ERR_BN128_INVALID_B")))?
                                .into()
                        }
                    };
                    vals.push((g1_a, g1_b))
                }

                let result = Self::execute(vals);
                if result {
                    bn::arith::U256::one()
                } else {
                    bn::arith::U256::zero()
                }
            };

            let mut res = vec![0u8; 32];
            output.to_big_endian(&mut res[0..32]).map_err(|_e| ERR_BIG_ENDIAN)?;
            Ok(res)
        }

        fn execute(vals: Vec<(bn::G1, bn::G2)>) -> bool {
            bn::pairing_batch(&vals) == bn::Gt::one()
        }
    }

    impl Precompile for Bn256Pair {
        fn required_gas(input: &[u8]) -> Result<Gas, ExitError> {
            let input_len = u64::try_from(input.len()).map_err(|_| ExitError::Other(Borrowed("ERR_USIZE_CONV")))?;
            let pair_element_len =
                u64::try_from(PAIR_ELEMENT_LEN).map_err(|_| ExitError::Other(Borrowed("ERR_USIZE_CONV")))?;
            Ok(COST_PAIR_PER_POINT * input_len / pair_element_len + COST_PAIR_BASE)
        }

        fn run(
            &self,
            input: &[u8],
            target_gas: Option<Gas>,
            context: &Context,
            _is_static: bool,
        ) -> Result<TvmPrecompileOutput, ExitError> {
            let cost = Self::required_gas(input)?;
            if let Some(target_gas) = target_gas {
                if cost > target_gas {
                    return Err(ExitError::OutOfGas);
                }
            }

            let output = Self::run_inner(input, context)?;
            Ok(TvmPrecompileOutput::without_logs(cost, output))
        }
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
    fn test_bn_add() {
        let input_data = "\
            0000000000000000000000000000000000000000000000000000000000000001\
            0000000000000000000000000000000000000000000000000000000000000002\
            0000000000000000000000000000000000000000000000000000000000000001\
            0000000000000000000000000000000000000000000000000000000000000002\
        ";
        let expected_data = "\
            030644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd3\
            15ed738c0e0a7c92e7845f96b2ae9c0a68a6a449e3538fc7ff3ebf7a5a18a2c4\
        ";
        let input = hex::decode(input_data).unwrap();
        let res = bn_add::Bn256Add
            .run(&input, Some(Gas::new(1000)), &new_context(), false)
            .unwrap()
            .output;
        let expected = hex::decode(expected_data).unwrap();
        assert_eq!(res, expected);
    }

    #[test]
    fn test_bn_mul() {
        let input_data = "\
            0000000000000000000000000000000000000000000000000000000000000001\
            0000000000000000000000000000000000000000000000000000000000000002\
            0000000000000000000000000000000000000000000000000000000000000002\
        ";
        let expected_data = "\
            030644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd3\
            15ed738c0e0a7c92e7845f96b2ae9c0a68a6a449e3538fc7ff3ebf7a5a18a2c4\
        ";
        let input = hex::decode(input_data).unwrap();
        let res = bn_mul::Bn256Mul
            .run(&input, Some(Gas::new(6000)), &new_context(), false)
            .unwrap()
            .output;
        let expected = hex::decode(expected_data).unwrap();
        assert_eq!(res, expected);
    }

    #[test]
    fn test_bn_pair() {
        let input_data = "\
            2cf44499d5d27bb186308b7af7af02ac5bc9eeb6a3d147c186b21fb1b76e18da\
            2c0f001f52110ccfe69108924926e45f0b0c868df0e7bde1fe16d3242dc715f6\
            1fb19bb476f6b9e44e2a32234da8212f61cd63919354bc06aef31e3cfaff3ebc\
            22606845ff186793914e03e21df544c34ffe2f2f3504de8a79d9159eca2d98d9\
            2bd368e28381e8eccb5fa81fc26cf3f048eea9abfdd85d7ed3ab3698d63e4f90\
            2fe02e47887507adf0ff1743cbac6ba291e66f59be6bd763950bb16041a0a85e\
            0000000000000000000000000000000000000000000000000000000000000001\
            30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd45\
            1971ff0471b09fa93caaf13cbf443c1aede09cc4328f5a62aad45f40ec133eb4\
            091058a3141822985733cbdddfed0fd8d6c104e9e9eff40bf5abfef9ab163bc7\
            2a23af9a5ce2ba2796c1f4e453a370eb0af8c212d9dc9acd8fc02c2e907baea2\
            23a8eb0b0996252cb548a4487da97b02422ebc0e834613f954de6c7e0afdc1fc\
        ";
        let expected_data = "\
            0000000000000000000000000000000000000000000000000000000000000001\
        ";
        let input = hex::decode(input_data).unwrap();
        let res = bn_pair::Bn256Pair
            .run(&input, Some(Gas::new(113000)), &new_context(), false)
            .unwrap()
            .output;
        let expected = hex::decode(expected_data).unwrap();
        assert_eq!(res, expected);
    }
}
