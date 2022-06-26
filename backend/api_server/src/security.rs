use pwhash::bcrypt;
use jsonwebtoken::{encode, decode, Algorithm, EncodingKey, DecodingKey, Header, Validation};
use chrono::{Duration, Utc};


use crate::models::*;


pub fn get_hashed_password(password: &str) -> String { 
    let hash = bcrypt::hash(password).unwrap();
    return hash
}


pub fn verify_password(password: &str, hashed_password: &str) -> bool {
    let b = bcrypt::verify(password, hashed_password);
    return b
}


pub fn get_jwt() -> String {
    let secret = std::env::var("JWT_SECRET").unwrap().into_bytes();
    let expiration_time = Utc::now()
        .checked_add_signed(Duration::seconds(60))
        .expect("invalid timestamp")
        .timestamp();

    let c = Claims {
        sub: String::from("test"),
        role: String::from("admin"),
        exp: expiration_time as usize,
    };

    let token = match encode(&Header::default(), &c, &EncodingKey::from_secret(&secret)) {
        Ok(t) => t,
        Err(_) => panic!(),
    };

    token
}


pub fn verify_jwt(token: &str) -> bool {
    let secret = std::env::var("JWT_SECRET").unwrap().into_bytes();
    let _is_jwt_valid = match decode::<Claims>(&token, &DecodingKey::from_secret(&secret), &Validation::new(Algorithm::HS256)) {
        Ok(_is_jwt_valid) => {
            return true
        }
        Err(_) => {
            return false
        }
    };
}