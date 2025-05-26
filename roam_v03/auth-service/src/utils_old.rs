// utils.rs
use sha2::{Sha256, Digest};
use hex;

pub fn hash_password(password: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    let result = hasher.finalize();
    hex::encode(result)
}

pub fn verify_password(stored_hash: &str, provided_password: &str) -> bool {
    let provided_hash = hash_password(provided_password);
    stored_hash == provided_hash
}

pub fn generate_token() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let token: String = (0..32).map(|_| rng.sample(rand::distributions::Alphanumeric)).collect();
    token
}