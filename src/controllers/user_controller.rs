use actix_web::{web, HttpResponse, Responder};
use sqlx::Pool;
use sqlx::Postgres;
use crate::models::user::{User, CreateUserRequest};

pub struct UserController;

impl UserController {
    pub async fn get_all_users(db_pool: web::Data<Pool<Postgres>>) -> impl Responder {
        let result = sqlx::query_as!(
            User,
            "SELECT id, name, email FROM users"
        )
        .fetch_all(db_pool.get_ref())
        .await;

        match result {
            Ok(users) => HttpResponse::Ok().json(users),
            Err(_) => HttpResponse::InternalServerError().body("Failed to fetch users"),
        }
    }

    pub async fn create_user(
        db_pool: web::Data<Pool<Postgres>>,
        user_data: web::Json<CreateUserRequest>,
    ) -> impl Responder {
        let result = sqlx::query_as!(
            User,
            "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id, name, email",
            user_data.name,
            user_data.email
        )
        .fetch_one(db_pool.get_ref())
        .await;

        match result {
            Ok(user) => HttpResponse::Created().json(user),
            Err(_) => HttpResponse::InternalServerError().body("Failed to create user"),
        }
    }

    pub async fn get_user_by_id(
        db_pool: web::Data<Pool<Postgres>>,
        path: web::Path<i32>,
    ) -> impl Responder {
        let user_id = path.into_inner();
        let result = sqlx::query_as!(
            User,
            "SELECT id, name, email FROM users WHERE id = $1",
            user_id
        )
        .fetch_one(db_pool.get_ref())
        .await;

        match result {
            Ok(user) => HttpResponse::Ok().json(user),
            Err(_) => HttpResponse::NotFound().body("User not found"),
        }
    }

    pub async fn update_user(
        db_pool: web::Data<Pool<Postgres>>,
        path: web::Path<i32>,
        user_data: web::Json<CreateUserRequest>,
    ) -> impl Responder {
        let user_id = path.into_inner();
        let result = sqlx::query!(
            "UPDATE users SET name = $1, email = $2 WHERE id = $3",
            user_data.name,
            user_data.email,
            user_id
        )
        .execute(db_pool.get_ref())
        .await;

        match result {
            Ok(_) => HttpResponse::Ok().body("User updated"),
            Err(_) => HttpResponse::InternalServerError().body("Failed to update user"),
        }
    }

    pub async fn delete_user(db_pool: web::Data<Pool<Postgres>>, path: web::Path<i32>) -> impl Responder {
        let user_id = path.into_inner();
        let result = sqlx::query!("DELETE FROM users WHERE id = $1", user_id)
            .execute(db_pool.get_ref())
            .await;

        match result {
            Ok(_) => HttpResponse::Ok().body("User deleted"),
            Err(_) => HttpResponse::InternalServerError().body("Failed to delete user"),
        }
    }
}
