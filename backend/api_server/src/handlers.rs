use actix_web::{HttpResponse, web, Responder};
use sqlx::PgPool;


use crate::models::*;
use crate::database;
use crate::security;


pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.route("/user/{id}", web::get().to(get_user));
    cfg.route("/users", web::get().to(get_users));
    cfg.route("/user", web::post().to(add_user));
    cfg.route("/user/{id}", web::delete().to(delete_user));
    cfg.route("/login", web::post().to(login));
}


pub async fn get_user(pool: web::Data<PgPool>, id: web::Path<i32>) -> impl Responder {
    let user = database::get_user_by_id_db(&pool, &id.into_inner()).await;

    match user {
        Err(_) => HttpResponse::NotFound().finish(),
        Ok(user) => HttpResponse::Ok().json(&user),
    }
}


pub async fn get_users(pool: web::Data<PgPool>) -> impl Responder {
    let users = database::get_users_db(&pool).await;

    match users {
        Err(_) => HttpResponse::NotFound().finish(),
        Ok(users) => HttpResponse::Ok().json(&users),
    }
}


pub async fn add_user(pool: web::Data<PgPool>, user: web::Json<InputUser>) -> impl Responder {
    let insert = database::add_user_db(&pool, &user).await;

    match insert {
        Err(_) => HttpResponse::BadRequest().finish(),
        Ok(_insert) => HttpResponse::Ok().finish(),
    }
}


pub async fn delete_user(pool: web::Data<PgPool>, id: web::Path<i32>) -> impl Responder {
    let delete = database::delete_user_db(&pool, &id.into_inner()).await;

    match delete {
        Err(_) => HttpResponse::NotFound().finish(),
        Ok(_delete) =>  HttpResponse::Ok().finish(),
    }
}


pub async fn login(pool: web::Data<PgPool>, form: web::Json<LoginUser>) -> impl Responder {
    let user = database::get_user_by_email_db(&pool, &form.email).await;

    match user {
        Err(_) => HttpResponse::NotFound().finish(),
        Ok(_user) => {
            let is_same_password = security::verify_password(&form.password, &_user.password);

            if is_same_password {
                let token = security::get_jwt();
                return HttpResponse::Ok().finish();
            }
            else {
                return HttpResponse::Unauthorized().finish();
            }
        }
    }
}