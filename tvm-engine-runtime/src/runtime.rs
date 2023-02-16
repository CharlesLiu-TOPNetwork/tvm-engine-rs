use tvm_engine_types::{Address, U256};

use crate::env::Env;
use crate::io::{StorageIntermediate, IO};

#[derive(Copy, Clone)]
pub struct Runtime;

pub struct RegisterIndex(u64);

impl StorageIntermediate for RegisterIndex {
    fn len(&self) -> usize {
        unsafe {
            let result = exports::tvm_register_len(self.0);
            if result < u64::MAX {
                result as usize
            } else {
                0
            }
        }
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn copy_to_slice(&self, buffer: &mut [u8]) {
        unsafe { exports::tvm_read_register(self.0, buffer.as_ptr() as u64) }
    }
}

impl Runtime {
    const IO_READ_REGISTER: RegisterIndex = RegisterIndex(0);
    const IO_WRITE_REGISTER: RegisterIndex = RegisterIndex(1);
    const IO_REMOVE_REGISTER: RegisterIndex = RegisterIndex(2);
    const ENV_REGISTER: RegisterIndex = RegisterIndex(3);
    const INPUT_REGISTER: RegisterIndex = RegisterIndex(4);
}

impl IO for Runtime {
    type StorageValue = RegisterIndex;

    fn get_input(&self) -> Self::StorageValue {
        unsafe {
            exports::tvm_input(Self::INPUT_REGISTER.0);
        }
        Self::INPUT_REGISTER
    }

    fn set_output(&self, value: &[u8]) {
        unsafe {
            exports::tvm_result(value.len() as u64, value.as_ptr() as u64);
        }
    }

    // fn storage_has_key(&self, key: &[u8]) -> bool {
    //     todo!()
    // }

    fn write_storage(&mut self, key: &[u8], value: &[u8]) -> Option<Self::StorageValue> {
        unsafe {
            if exports::tvm_storage_write(
                key.len() as u64,
                key.as_ptr() as u64,
                value.len() as u64,
                value.as_ptr() as u64,
                Self::IO_WRITE_REGISTER.0,
            ) == 1
            {
                Some(Self::IO_WRITE_REGISTER)
            } else {
                None
            }
        }
    }

    fn read_storage(&self, key: &[u8]) -> Option<Self::StorageValue> {
        unsafe {
            if exports::tvm_storage_read(key.len() as u64, key.as_ptr() as u64, Self::IO_READ_REGISTER.0) == 1 {
                Some(Self::IO_READ_REGISTER)
            } else {
                None
            }
        }
    }

    fn remove_storage(&mut self, key: &[u8]) -> Option<Self::StorageValue> {
        unsafe {
            if exports::tvm_storage_remove(key.len() as u64, key.as_ptr() as u64, Self::IO_REMOVE_REGISTER.0) == 1 {
                Some(Self::IO_REMOVE_REGISTER)
            } else {
                None
            }
        }
    }
}

impl Env for Runtime {
    fn gas_price(&self) -> U256 {
        U256::from(unsafe { exports::tvm_gas_price() })
    }

    fn origin(&self) -> Address {
        unsafe {
            exports::tvm_origin_address(Self::ENV_REGISTER.0);
        }
        let bytes = Self::ENV_REGISTER.to_vec();
        Address::build_from_slice(&bytes).unwrap()
    }

    fn block_height(&self) -> u64 {
        unsafe { exports::tvm_block_height() }
    }

    fn block_coinbase(&self) -> Address {
        unsafe {
            exports::tvm_block_coinbase(Self::ENV_REGISTER.0);
        }
        let bytes = Self::ENV_REGISTER.to_vec();
        Address::build_from_slice(&bytes).unwrap()
    }

    fn block_timestamp(&self) -> crate::env::Timestamp {
        crate::env::Timestamp::new(unsafe { exports::tvm_block_timestamp() })
    }

    fn chain_id(&self) -> u64 {
        unsafe { exports::tvm_chain_id() }
    }
}

mod exports {
    extern "C" {
        // register common
        pub fn tvm_read_register(register_id: u64, ptr: u64);
        pub fn tvm_register_len(register_id: u64) -> u64;

        // io input && output
        pub fn tvm_input(register_id: u64);
        pub fn tvm_result(value_len: u64, value_ptr: u64);

        // io storage
        pub fn tvm_storage_write(key_len: u64, key_ptr: u64, value_len: u64, value_ptr: u64, register_id: u64) -> u64;
        pub fn tvm_storage_read(key_len: u64, key_ptr: u64, register_id: u64) -> u64;
        pub fn tvm_storage_remove(key_len: u64, key_ptr: u64, register_id: u64) -> u64;

        // env
        pub fn tvm_gas_price() -> u64;
        pub fn tvm_origin_address(register_id: u64);
        pub fn tvm_block_height() -> u64;
        pub fn tvm_block_coinbase(register_id: u64);
        pub fn tvm_block_timestamp() -> u64;
        pub fn tvm_chain_id() -> u64;

        // logs
        pub fn tvm_log_utf8(len: u64, ptr: u64);
    }
}
pub use exports::tvm_log_utf8;
