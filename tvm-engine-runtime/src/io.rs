pub trait IO {}

pub mod methods {
    use super::IO;
    use tvm_engine_types::{uTop, Address, H256, U256};

    pub fn get_balance<I: IO>(io: &mut I, address: &Address) -> uTop {
        todo!()
    }
    pub fn set_balance<I: IO>(io: &mut I, address: &Address, amount: &uTop) {
        todo!()
    }
    pub fn add_balance<I: IO>(io: &mut I, address: &Address, amount: &uTop) {
        todo!()
    }
    // pub fn remove_balance<I: IO>(io: &mut I, address: &Address) {
    //     todo!()
    // }

    pub fn get_code<I: IO>(io: &mut I, address: &Address) -> Vec<u8> {
        todo!()
    }
    pub fn set_code<I: IO>(io: &mut I, address: &Address, code: &[u8]) {
        todo!()
    }
    pub fn get_code_size<I: IO>(io: &mut I, address: &Address) -> usize {
        todo!()
    }
    pub fn remove_code<I: IO>(io: &mut I, address: &Address) {
        todo!()
    }

    pub fn get_nonce<I: IO>(io: &mut I, address: &Address) -> U256 {
        todo!()
    }
    pub fn set_nonce<I: IO>(io: &mut I, address: &Address, nonce: &U256) {
        todo!()
    }
    pub fn increment_nonce<I: IO>(io: &mut I, address: &Address) {
        todo!()
    }
    // pub fn check_nonce<I: IO>(io: &mut I, address: &Address) {
    //     todo!()
    // }

    pub fn get_storage<I: IO>(io: &mut I, address: &Address, key: &H256) -> H256 {
        todo!()
    }
    pub fn set_storage<I: IO>(io: &mut I, address: &Address, key: &H256, value: &H256) {
        todo!()
    }
    pub fn remove_storage<I: IO>(io: &mut I, address: &Address, key: &H256) {
        todo!()
    }
    pub fn remove_all_storage<I: IO>(io: &mut I, address: &Address) {
        todo!()
    }

    pub fn is_account_empty<I: IO>(io: &I, address: &Address) -> bool {
        todo!()
    }

    pub fn remove_account<I: IO>(io: &mut I, address: &Address) {
        todo!()
    }
}
