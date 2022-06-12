use scrypt::{password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString}, Scrypt};


pub fn get_salt() -> String {
    
    let salt = SaltString::generate(&mut OsRng);
    salt.to_string()
}

pub fn get_hashed_password(password: &str, salt: &str) -> String {
    
    let password_hash = Scrypt
        .hash_password(password.as_bytes(), salt)
        .unwrap()
        .to_string();

    password_hash
}