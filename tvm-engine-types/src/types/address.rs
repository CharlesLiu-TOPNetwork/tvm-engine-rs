use primitive_types::H160;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Address(H160);

/// account address, should be compatible with both `0x` and `T80000` prefix
impl Address {
    /// Build from H160
    pub const fn build_from_hash160(val: H160) -> Self {
        Self(val)
    }

    /// Build(decode) from str
    pub fn build_from_str(val: &str) -> Result<Self, error::AddressError> {
        if val.len() != 40 {
            return Err(error::AddressError::IncorrectLength);
        }

        let mut res = [0u8; 20];
        hex::decode_to_slice(val, &mut res).map_err(|_| error::AddressError::DecodeFailure)?;
        Ok(Self::build_from_hash160(H160(res)))
    }

    /// Build from slice
    pub fn build_from_slice(val: &[u8]) -> Result<Self, error::AddressError> {
        if val.len() != 20 {
            return Err(error::AddressError::IncorrectLength);
        }

        Ok(Self::build_from_hash160(H160::from_slice(val)))
    }

    /// Zero Address
    pub const fn zero() -> Self {
        Self::build_from_hash160(H160([0u8; 20]))
    }

    pub fn as_slice(&self) -> &[u8] {
        self.0.as_bytes()
    }

    pub fn encode(&self) -> String {
        hex::encode(self.as_slice())
    }

    pub fn to_top_address(&self) -> String {
        // might be T8Address struct
        todo!()
    }
}

pub mod error {

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum AddressError {
        IncorrectLength,
        DecodeFailure,
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_wrong_length_addres() {
        todo!()
    }

    #[test]
    fn test_from_top_address() {
        todo!()
    }
}
