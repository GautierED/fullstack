use actix_web::{web, Result};
use sqlx::PgPool;
use sqlx::postgres::PgQueryResult;


use crate::models::*;
use crate::security;


pub async fn get_user_db(pool: &PgPool, id: &i32) -> Result<User, sqlx::Error> {
    sqlx::query_as("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await
}


pub async fn get_users_db(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as("SELECT * FROM users ORDER BY id")
        .fetch_all(pool)
        .await
}


pub async fn add_user_db(pool: &PgPool, user: &web::Json<InputUser>) -> Result<PgQueryResult, sqlx::Error> {
    let hash = security::get_hashed_password(&user.password);

    sqlx::query("INSERT INTO users (email, password) VALUES ($1, $2)")
        .bind(user.email.clone())
        .bind(hash)
        .execute(pool)
        .await
}


pub async fn delete_user_db(pool: &PgPool, id: &i32) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await
}