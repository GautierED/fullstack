#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub name: String, 
    pub age: i32,
    pub address: String, 
    pub salary: f32
}


#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct InputUser {
    pub name: String, 
    pub age: i32,
    pub address: String, 
    pub salary: f32
}