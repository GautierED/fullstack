use scrypt::password_hash::{rand_core::OsRng, SaltString};
use pwhash::bcrypt;


pub fn get_salt() -> String {
    
    let salt = SaltString::generate(&mut OsRng);
    salt.to_string()
}


pub fn get_hashed_password(password: &str, salt: &str) -> String { 
    let c = format!("{} {}", password.to_string(), salt.to_string());
    let h = bcrypt::hash(c).unwrap();
    return h
}


pub fn verify_password(password: &str, hashed_password: &str, salt: &str) -> bool {
    let c = format!("{} {}", password.to_string(), salt.to_string());
    let b = bcrypt::verify(c, hashed_password);
    return b
}