use pwhash::bcrypt;
use jsonwebtoken::{encode, EncodingKey, Header};
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

    let p = Payload {
        sub: String::from("test"),
        role: String::from("admin"),
        exp: expiration_time as usize,
    };

    let token = match encode(
        &Header::default(),
        &p,
        &EncodingKey::from_secret(&secret),
    ) {
        Ok(t) => t,
        Err(_) => panic!(),
    };

    token
}