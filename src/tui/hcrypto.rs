use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn hash(password: &str) -> String {
    let input = password.to_string();
    let mut hasher = DefaultHasher::new();
    input.hash(&mut hasher);
    let hash = hasher.finish();
    return hash.to_string();
}
