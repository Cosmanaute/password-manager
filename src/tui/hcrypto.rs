use magic_crypt::*;
use sha3::{Sha3_256, Digest};

pub fn hash(password: &str) -> String {
    let mut hasher = Sha3_256::new();
    hasher.update(password.as_bytes());
    let hashed_password = hasher.finalize();
    format!("{:x}", hashed_password)
}

pub fn encrypt(key: &str, password: &str) -> String {
    let mcrypt = new_magic_crypt!(key, 256);
    let encrypted_password = mcrypt.encrypt_to_base64(password);
    return encrypted_password.to_string()
} 

pub fn decrypt(key: &str, encrypted: &str) -> String {
    let mcrypt = new_magic_crypt!(&key, 256);
    let decrypted_password = mcrypt.decrypt_base64_to_string(&encrypted).unwrap();
    return decrypted_password.to_string()
}
