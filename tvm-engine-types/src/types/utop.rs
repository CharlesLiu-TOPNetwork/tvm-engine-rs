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
    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }

    pub fn raw(&self) -> u64 {
        self.0
    }

    pub fn to_be_bytes(&self) -> [u8; 8] {
        self.0.to_be_bytes()
    }

    // pub fn check_add(self, rhs: Self) -> Option<Self> {
    //     self.0.checked_add(rhs.0).map(Self)
    // }

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

impl From<u64> for uTop {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}
