use std::ops::{Add, AddAssign, Div, Mul};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Gas(u64);

impl Gas {
    pub const fn new(gas: u64) -> Gas {
        Self(gas)
    }
    pub fn as_u64(self) -> u64 {
        self.0
    }
}

impl Add<Gas> for Gas {
    type Output = Gas;

    fn add(self, rhs: Gas) -> Self::Output {
        Gas(self.0 + rhs.0)
    }
}

impl AddAssign<Gas> for Gas {
    fn add_assign(&mut self, rhs: Gas) {
        self.0 += rhs.0
    }
}

impl Div<u64> for Gas {
    type Output = Gas;

    fn div(self, rhs: u64) -> Self::Output {
        Gas(self.0 / rhs)
    }
}

impl Mul<Gas> for u32 {
    type Output = Gas;

    fn mul(self, rhs: Gas) -> Self::Output {
        Gas(u64::from(self) * rhs.0)
    }
}

impl Mul<u32> for Gas {
    type Output = Gas;

    fn mul(self, rhs: u32) -> Self::Output {
        Gas(self.0 * u64::from(rhs))
    }
}

impl Mul<u64> for Gas {
    type Output = Gas;

    fn mul(self, rhs: u64) -> Self::Output {
        Gas(self.0 * rhs)
    }
}

impl Mul<Gas> for u64 {
    type Output = Gas;

    fn mul(self, rhs: Gas) -> Self::Output {
        Gas(self * rhs.0)
    }
}
