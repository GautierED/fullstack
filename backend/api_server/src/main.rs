use actix_web::{http::header::ContentType, HttpResponse, Error, web, App, HttpServer, Result};
use sqlx::PgPool;
use serde_json;

#[derive(serde::Serialize, sqlx::FromRow)]
pub struct User {
    id: i32,
    name: String, 
    age: i32,
    address: String, 
    salary: f32
}

#[actix_web::main]
async fn main() -> Result<(), sqlx::Error> {

    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await?;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/user/{id}", web::get().to(get_user_by_id))
        })
        .bind("127.0.0.1:8080")?
        .run()
        .await;

    Ok(())
}

async fn get_user_by_id(pool: web::Data<PgPool>, id: web::Path<i32>) -> Result<HttpResponse, Error> {

    let user = get_user_by_id_db(&pool, id.into_inner())
        .await;

    let j = serde_json::to_string(&user)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(j))
}

pub async fn get_user_by_id_db(pool: &PgPool, id: i32) -> User {
    sqlx::query_as("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await
        .unwrap()
}

pub async fn get_users_db(pool: &PgPool) -> Vec<User> {
    sqlx::query_as("SELECT * FROM users ORDER BY id")
        .fetch_all(pool)
        .await
        .unwrap()
}