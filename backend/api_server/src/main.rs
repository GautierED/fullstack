use actix_web::{web, App, HttpServer, Result};
use sqlx::PgPool;

mod handlers;
mod models;
mod database;
mod security;

#[actix_web::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv::dotenv().ok();
    let port = std::env::var("port").expect("PORT must be set");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await?;
    println!("Starting server, listening to {}", &port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(handlers::init)
        })
        .bind(&port)?
        .run()
        .await?;

    Ok(())
}