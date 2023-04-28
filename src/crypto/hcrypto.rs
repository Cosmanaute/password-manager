use sha2::{Sha256, Digest};

pub fn hash(password: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    let hashed_password = hasher.finalize();
    format!("{:x}", hashed_password)
}
