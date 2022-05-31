use actix_web::{http::header::ContentType, HttpResponse, Error, web, App, HttpServer, Result, Responder};
use sqlx::PgPool;
use sqlx::postgres::PgQueryResult;
use serde_json;


#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct User {
    id: i32,
    name: String, 
    age: i32,
    address: String, 
    salary: f32
}


#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct InputUser {
    name: String, 
    age: i32,
    address: String, 
    salary: f32
}


pub async fn get_user(pool: web::Data<PgPool>, id: web::Path<i32>) -> impl Responder {

    let user = get_user_db(&pool, id.into_inner()).await;

    match user {
        Err(_) => HttpResponse::NotFound().finish(),
        Ok(user) => HttpResponse::Ok().json(&user),
    }
}


async fn get_user_db(pool: &PgPool, id: i32) -> Result<User, sqlx::Error> {
    sqlx::query_as("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await
}


pub async fn get_users(pool: web::Data<PgPool>) -> impl Responder {

    let users = get_users_db(&pool).await;

    match users {
        Err(_) => HttpResponse::NotFound().finish(),
        Ok(users) => HttpResponse::Ok().json(&users),
    }
}


async fn get_users_db(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as("SELECT * FROM users ORDER BY id")
        .fetch_all(pool)
        .await
}


pub async fn add_user(pool: web::Data<PgPool>, user: web::Json<InputUser>) -> impl Responder {
    
    let insert = add_user_db(&pool, user).await;

    match insert {
        Err(_) => HttpResponse::BadRequest().finish(),
        Ok(insert) => HttpResponse::Ok().finish(),
    }
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


pub async fn delete_user(pool: web::Data<PgPool>, id: web::Path<i32>) -> impl Responder {

    let delete = delete_user_db(&pool, id.into_inner()).await;

    match delete {
        Err(_) => HttpResponse::BadRequest().finish(),
        Ok(delete) => HttpResponse::Ok().finish(),
    }
}


async fn delete_user_db(pool: &PgPool, id: i32) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await
}