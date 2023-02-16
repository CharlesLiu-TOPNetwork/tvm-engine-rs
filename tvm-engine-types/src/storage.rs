use crate::{Address, H256};

enum VersionPrefix {
    V1 = 0x1,
}

#[derive(Clone, Copy)]
pub enum KeyPrefix {
    Nonce = 0x1,
    Balance = 0x2,
    Code = 0x3,
    Storage = 0x4,
}

pub fn address_to_key(prefix: KeyPrefix, address: &Address) -> [u8; 22] {
    let mut r = [0u8; 22];
    r[0] = VersionPrefix::V1 as u8;
    r[1] = prefix as u8;
    r[2..22].copy_from_slice(address.as_slice());
    r
}

pub fn storage_to_key(address: &Address, key: &H256) -> [u8; 54] {
    let mut r = [0u8; 54];
    r[0] = VersionPrefix::V1 as u8;
    r[1] = KeyPrefix::Storage as u8;
    r[2..22].copy_from_slice(address.as_slice());
    r[22..54].copy_from_slice(&key.0);
    r
}
