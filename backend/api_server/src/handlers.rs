use actix_web::{http::header::ContentType, HttpResponse, Error, web, App, HttpServer, Result};
use sqlx::PgPool;
use sqlx::postgres::PgQueryResult;
use serde_json;


#[derive(serde::Serialize, sqlx::FromRow)]
pub struct User {
    id: i32,
    name: String, 
    age: i32,
    address: String, 
    salary: f32
}


#[derive(serde::Serialize, serde::Deserialize)]
pub struct InputUser {
    name: String, 
    age: i32,
    address: String, 
    salary: f32
}


pub async fn get_user(pool: web::Data<PgPool>, id: web::Path<i32>) -> Result<HttpResponse, Error> {

    let user = get_user_db(&pool, id.into_inner()).await;

    let j = serde_json::to_string(&user)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(j))
}


async fn get_user_db(pool: &PgPool, id: i32) -> User {
    sqlx::query_as("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await
        .unwrap()
}


pub async fn get_users(pool: web::Data<PgPool>) -> Result<HttpResponse, Error> {

    let users = get_users_db(&pool).await;

    let j = serde_json::to_string(&users)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(j))
}


async fn get_users_db(pool: &PgPool) -> Vec<User> {
    sqlx::query_as("SELECT * FROM users ORDER BY id")
        .fetch_all(pool)
        .await
        .unwrap()
}


pub async fn add_user(pool: web::Data<PgPool>, user: web::Json<InputUser>) -> Result<HttpResponse, Error> {
    
    add_user_db(&pool, user).await;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .body("user added"))
}


async fn add_user_db(pool: &PgPool, user: web::Json<InputUser>) -> Result<PgQueryResult, sqlx::Error> {

    let u = InputUser {
        name: user.name.clone(),
        age: user.age,
        address: user.address.clone(),
        salary: user.salary,

    };

    sqlx::query("INSERT INTO users (name, age, address, salary) VALUES ($1, $2, $3, $4)")
        .bind(u.name)
        .bind(u.age)
        .bind(u.address)
        .bind(u.salary)
        .execute(pool)
        .await
}


pub async fn delete_user(pool: web::Data<PgPool>, id: web::Path<i32>) -> Result<HttpResponse, Error> {

    delete_user_db(&pool, id.into_inner()).await;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .body("user deleted"))
}


async fn delete_user_db(pool: &PgPool, id: i32) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await
}