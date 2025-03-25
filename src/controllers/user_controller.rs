use actix_web::{web, HttpResponse, Responder};
use sqlx::Pool;
use sqlx::Postgres;
use crate::models::user::CreateUserRequest;
use crate::services::user_service::UserService;

pub struct UserController;

impl UserController {
    pub async fn get_all_users(db_pool: web::Data<Pool<Postgres>>) -> impl Responder {
        match UserService::get_all_users(db_pool.get_ref()).await {
            Ok(users) => HttpResponse::Ok().json(users),
            Err(err) => HttpResponse::InternalServerError().body(err),
        }
    }

    pub async fn get_user_by_id(db_pool: web::Data<Pool<Postgres>>, path: web::Path<i32>) -> impl Responder {
        match UserService::get_user_by_id(db_pool.get_ref(), path.into_inner()).await {
            Ok(user) => HttpResponse::Ok().json(user),
            Err(err) => HttpResponse::NotFound().body(err),
        }
    }

    pub async fn create_user(db_pool: web::Data<Pool<Postgres>>, user_data: web::Json<CreateUserRequest>) -> impl Responder {
        match UserService::create_user(db_pool.get_ref(), user_data.into_inner()).await {
            Ok(user) => HttpResponse::Created().json(user),
            Err(err) => HttpResponse::InternalServerError().body(err),
        }
    }

    pub async fn update_user(db_pool: web::Data<Pool<Postgres>>, path: web::Path<i32>, user_data: web::Json<CreateUserRequest>) -> impl Responder {
        match UserService::update_user(db_pool.get_ref(), path.into_inner(), user_data.into_inner()).await {
            Ok(_) => HttpResponse::Ok().body("User updated"),
            Err(err) => HttpResponse::InternalServerError().body(err),
        }
    }

    pub async fn delete_user(db_pool: web::Data<Pool<Postgres>>, path: web::Path<i32>) -> impl Responder {
        match UserService::delete_user(db_pool.get_ref(), path.into_inner()).await {
            Ok(_) => HttpResponse::Ok().body("User deleted"),
            Err(err) => HttpResponse::InternalServerError().body(err),
        }
    }
}
