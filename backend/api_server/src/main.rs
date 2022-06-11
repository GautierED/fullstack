use actix_web::{web, App, HttpServer, Result};
use sqlx::PgPool;

mod handlers;

#[actix_web::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv::dotenv().ok();
    println!("Starting server...");

    let port = std::env::var("port").expect("PORT must be set");

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await?;
    println!("Connected to database");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/user/{id}", web::get().to(handlers::get_user))
            .route("/users", web::get().to(handlers::get_users))
            .route("/user", web::post().to(handlers::add_user))
            .route("/user/{id}", web::delete().to(handlers::delete_user))
        })
        .bind(&port)?
        .run()
        .await?;

    Ok(())
}