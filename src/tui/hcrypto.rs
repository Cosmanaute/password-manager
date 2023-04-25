use pwhash::bcrypt;

pub fn hash(password: &str) -> String {
    let mut hashed_password = String::new();
    hashed_password = bcrypt::hash(password).unwrap();
    return hashed_password
}
