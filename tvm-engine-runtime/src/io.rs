/// The purpose of this trait is to represent a reference to a value that
/// could be obtained by IO, but without eagerly loading it into memory.
pub trait StorageIntermediate: Sized {
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn copy_to_slice(&self, buffer: &mut [u8]);

    fn to_vec(&self) -> Vec<u8> {
        let size = self.len();
        let mut buf = vec![0u8; size];
        self.copy_to_slice(&mut buf);
        buf
    }
}

pub trait IO {
    /// A type giving a reference to a value obtained by IO without loading it into memory.
    type StorageValue: StorageIntermediate;

    /// all bytes that decode into call args.
    fn get_input(&self) -> Self::StorageValue;

    /// all bytes that encode as return result.
    fn set_output(&self, value: &[u8]);

    /// check if a key exist in storage
    // fn storage_has_key(&self, key: &[u8]) -> bool;

    /// write the given value into storage key, return old reference(if any)
    fn write_storage(&mut self, key: &[u8], value: &[u8]) -> Option<Self::StorageValue>;

    /// read the value in storage of given key.
    fn read_storage(&self, key: &[u8]) -> Option<Self::StorageValue>;

    /// remove the value in storage of given key, and present them(if any)
    fn remove_storage(&mut self, key: &[u8]) -> Option<Self::StorageValue>;

    /// read length of bytes in storage without actually loading them into engine
    fn read_storage_len(&self, key: &[u8]) -> Option<usize> {
        self.read_storage(key).map(|s| s.len())
    }

    // convenience function to read a u64 (big-endian encoding)
    fn read_u64(&self, key: &[u8]) -> Result<u64, error::ReadError> {
        let value = self.read_storage(key).ok_or(error::ReadError::MissingValue)?;
        if value.len() != 8 {
            return Err(error::ReadError::InvalidU64);
        }
        let mut result = [0u8; 8];
        value.copy_to_slice(&mut result);
        Ok(u64::from_be_bytes(result))
    }
}

pub mod methods {

    use super::{StorageIntermediate, IO};
    use tvm_engine_types::{address_to_key, storage_to_key, uTop, Address, KeyPrefix, H256, U256};

    // balance
    pub fn get_balance<I: IO>(io: &I, address: &Address) -> uTop {
        let raw = io
            .read_u64(&address_to_key(KeyPrefix::Balance, address))
            .unwrap_or_else(|_| 0);
        uTop::new(raw)
    }
    pub fn set_balance<I: IO>(io: &mut I, address: &Address, amount: &uTop) {
        io.write_storage(&address_to_key(KeyPrefix::Balance, address), &amount.to_be_bytes());
    }
    // pub fn add_balance<I: IO>(io: &mut I, address: &Address, amount: &uTop) {
    //     let current_balance = get_balance(io, address);
    //     let new_balance = current_balance.check_add(amount).ok_or();
    //     todo!()
    // }
    pub fn remove_balance<I: IO>(io: &mut I, address: &Address) {
        io.remove_storage(&address_to_key(KeyPrefix::Balance, address));
    }

    // code
    pub fn get_code<I: IO>(io: &I, address: &Address) -> Vec<u8> {
        io.read_storage(&address_to_key(KeyPrefix::Code, address))
            .map(|s| s.to_vec())
            .unwrap_or_default()
    }
    pub fn set_code<I: IO>(io: &mut I, address: &Address, code: &[u8]) {
        io.write_storage(&address_to_key(KeyPrefix::Code, address), code);
    }
    pub fn get_code_size<I: IO>(io: &I, address: &Address) -> usize {
        io.read_storage_len(&address_to_key(KeyPrefix::Code, address))
            .unwrap_or(0)
    }
    pub fn remove_code<I: IO>(io: &mut I, address: &Address) {
        io.remove_storage(&address_to_key(KeyPrefix::Code, address));
    }

    // nonce
    pub fn get_nonce<I: IO>(io: &I, address: &Address) -> U256 {
        let raw = io
            .read_u64(&address_to_key(KeyPrefix::Balance, address))
            .unwrap_or_else(|_| 0);
        U256::from(raw)
    }
    pub fn set_nonce<I: IO>(io: &mut I, address: &Address, nonce: &U256) {
        io.write_storage(
            &address_to_key(KeyPrefix::Nonce, address),
            &nonce.as_u64().to_be_bytes(),
        );
    }
    pub fn increment_nonce<I: IO>(io: &mut I, address: &Address) {
        let current_nonce = get_nonce(io, address);
        set_nonce(io, address, &current_nonce.saturating_add(U256::one()))
    }
    pub fn remove_nonce<I: IO>(io: &mut I, address: &Address) {
        io.remove_storage(&address_to_key(KeyPrefix::Nonce, address));
    }

    // storage
    pub fn get_storage<I: IO>(io: &I, address: &Address, key: &H256) -> H256 {
        io.read_storage(&storage_to_key(address, key))
            .and_then(|s| {
                if s.len() == 32 {
                    let mut buf = [0u8; 32];
                    s.copy_to_slice(&mut buf);
                    Some(H256(buf))
                } else {
                    None
                }
            })
            .unwrap_or_default()
    }
    pub fn set_storage<I: IO>(io: &mut I, address: &Address, key: &H256, value: &H256) {
        io.write_storage(&storage_to_key(address, key), value.as_bytes());
    }
    pub fn remove_storage<I: IO>(io: &mut I, address: &Address, key: &H256) {
        io.remove_storage(&storage_to_key(address, key));
    }
    pub fn remove_all_storage<I: IO>(io: &mut I, address: &Address) {
        io.remove_storage(&address_to_key(KeyPrefix::Storage, address));
    }

    pub fn is_account_empty<I: IO>(io: &I, address: &Address) -> bool {
        get_balance(io, address).is_zero() && get_nonce(io, address).is_zero() && get_code_size(io, address) == 0
    }

    pub fn remove_account<I: IO>(io: &mut I, address: &Address) {
        remove_nonce(io, address);
        remove_balance(io, address);
        remove_code(io, address);
        remove_all_storage(io, address);
    }
}

mod error {
    #[derive(Debug)]
    pub enum ReadError {
        InvalidU64,
        InvalidU256,
        MissingValue,
    }
}
