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


#[actix_web::main]
async fn main() -> Result<(), sqlx::Error> {

    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await?;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/user/{id}", web::get().to(get_user))
            .route("/users", web::get().to(get_users))
            .route("/add_user", web::post().to(add_user))
            .route("/delete/{id}", web::delete().to(delete_user))
        })
        .bind("127.0.0.1:8080")?
        .run()
        .await;

    Ok(())
}


pub async fn get_user(pool: web::Data<PgPool>, id: web::Path<i32>) -> Result<HttpResponse, Error> {

    let user = get_user_db(&pool, id.into_inner()).await;

    let j = serde_json::to_string(&user)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(j))
}


pub async fn get_user_db(pool: &PgPool, id: i32) -> User {
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


pub async fn get_users_db(pool: &PgPool) -> Vec<User> {
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


pub async fn add_user_db(pool: &PgPool, user: web::Json<InputUser>) -> Result<PgQueryResult, sqlx::Error> {

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


pub async fn delete_user_db(pool: &PgPool, id: i32) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await
}
