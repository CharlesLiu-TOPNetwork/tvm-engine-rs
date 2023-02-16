use crate::PAddress;

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

    pub fn raw(&self) -> H160 {
        self.0
    }

    /// Zero Address
    pub const fn zero() -> Self {
        Self::build_from_hash160(H160([0u8; 20]))
    }

    pub fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    pub fn as_slice(&self) -> &[u8] {
        self.0.as_bytes()
    }

    pub fn encode(&self) -> String {
        hex::encode(self.as_slice())
    }

    /// Top T8 address, use T80000 prefix than do xxhash64 (NOT xxh3) on hex-string address.
    /// Get a u64 value x, tableid = x & 63
    pub fn get_top_address_tableid(&self) -> u8 {
        // let top_address = format!("T80000{}", self.encode());
        (xxhash_rust::const_xxh64::xxh64([b"T80000", self.encode().as_bytes()].concat().as_slice(), 0) & 63) as u8
    }
}

impl Default for Address {
    fn default() -> Self {
        Self::zero()
    }
}

pub mod error {

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum AddressError {
        IncorrectLength,
        DecodeFailure,
    }
}

impl From<&PAddress> for Address {
    fn from(value: &PAddress) -> Self {
        Address::build_from_slice(&value.value).expect("Incorrect Address Length from ProtoAddress")
    }
}
impl From<PAddress> for Address {
    fn from(value: PAddress) -> Self {
        (&value).into()
    }
}

impl From<&Address> for PAddress {
    fn from(value: &Address) -> Self {
        PAddress {
            value: value.as_slice().to_vec(),
            ..Default::default()
        }
    }
}
impl From<Address> for PAddress {
    fn from(value: Address) -> Self {
        (&value).into()
    }
}

impl From<H160> for PAddress {
    fn from(value: H160) -> Self {
        PAddress {
            value: value.as_bytes().to_vec(),
            ..Default::default()
        }
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
    fn test_proto_address() {
        let addr = Address::build_from_str("7156526fbd7a3c72969b54f64e42c10fbb768c8a").unwrap();
        let paddr: PAddress = addr.into();
        let apaddr: Address = paddr.into();
        assert_eq!(apaddr, addr);
    }

    #[test]
    fn test_top_address_tableid() {
        // let r = xxhash_rust::const_xxh3::xxh3_64(b"T8000056d9407e0ae1246a2aafcfa57f3fc1bd7023df81");
        // let l = xxhash_rust::const_xxh64::xxh64(b"T8000056d9407e0ae1246a2aafcfa57f3fc1bd7023df81", 0);
        // println!("{}\n{}", r, l);
        assert_eq!(
            xxhash_rust::const_xxh64::xxh64(b"T8000056d9407e0ae1246a2aafcfa57f3fc1bd7023df81", 0),
            13715836294981773700
        );
        // tableid 4
        assert_eq!(
            Address::build_from_str("56d9407e0ae1246a2aafcfa57f3fc1bd7023df81")
                .unwrap()
                .get_top_address_tableid(),
            4
        );
    }
}
