use std::ops::{Div, Mul};

use primitive_types::U256;

/// Newtype of TOP's balance: uTOP. 1 TOP = 1 * 10^6 uTOP
#[allow(non_camel_case_types)]
pub struct uTop(u64);

impl uTop {
    const UTOP_TO_WEI: U256 = U256([1_000_000_000_000, 0, 0, 0]);
    const UTOP_MAX: U256 = U256([u64::MAX, 0, 0, 0]);

    pub const fn zero() -> Self {
        Self(0)
    }
    pub const fn new(amount: u64) -> Self {
        Self(amount)
    }

    pub fn into_wei_raw(self) -> U256 {
        U256([self.0, 0, 0, 0]).mul(Self::UTOP_TO_WEI)
    }

    pub fn from_wei_value(value: U256) -> Option<Self> {
        let r = value.div(Self::UTOP_TO_WEI);
        // should never be bigger than u64::MAX
        if r > Self::UTOP_MAX {
            None
        } else {
            Some(Self::new(r.as_u64()))
        }
    }
}
