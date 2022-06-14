use pwhash::bcrypt;


pub fn get_hashed_password(password: &str) -> String { 
    let hash = bcrypt::hash(password).unwrap();
    return hash
}


pub fn verify_password(password: &str, hashed_password: &str) -> bool {
    let b = bcrypt::verify(password, hashed_password);
    return b
}