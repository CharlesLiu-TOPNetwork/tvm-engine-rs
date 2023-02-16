pub fn sha256(input: &Vec<u8>) -> Vec<u8> {
    use sha2::Digest;
    sha2::Sha256::digest(input).to_vec()
}

pub fn panic_utf8(bytes: &[u8]) -> ! {
    println!("panic: {:?}", bytes);
    unsafe {
        crate::runtime::tvm_log_utf8(bytes.len() as u64, bytes.as_ptr() as u64);
    }
    unreachable!()
}

pub fn log_utf8(bytes: &[u8]) {
    println!("log: {:?}", bytes);
    unsafe {
        crate::runtime::tvm_log_utf8(bytes.len() as u64, bytes.as_ptr() as u64);
    }
}

pub fn log(data: &str) {
    log_utf8(data.as_bytes());
}
