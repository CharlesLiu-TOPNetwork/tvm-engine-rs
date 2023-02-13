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
