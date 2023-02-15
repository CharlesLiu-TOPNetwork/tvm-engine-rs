pub fn sha256(input: &Vec<u8>) -> Vec<u8> {
    use sha2::Digest;
    sha2::Sha256::digest(input).to_vec()
}

pub fn log_utf8(bytes: &[u8]) {
    // todo export logs
    println!("log: {:?}", bytes);
}

pub fn log(data: &str) {
    log_utf8(data.as_bytes());
}
