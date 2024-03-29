#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub email: String, 
    pub password: String
}


#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct InputUser {
    pub email: String, 
    pub password: String
}


#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Claims {
    pub sub: String,
    pub role: String,
    pub exp: usize
}