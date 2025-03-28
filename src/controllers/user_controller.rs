use actix_web::{web, HttpResponse, Responder};
use sqlx::{Pool, Postgres};

// Commands
use crate::commands::users::{
    create_user_command::CreateUserCommand,
    update_user_command::UpdateUserCommand,
    delete_user_command::DeleteUserCommand,
};

// Queries
use crate::queries::users::get_user_by_id_query::GetUserByIdQuery;

// Command Handlers
use crate::handlers::commands::{
    handle_create_user,
    handle_update_user,
    handle_delete_user,
};

// Query Handlers
use crate::handlers::queries::{
    handle_get_all_users,
    handle_get_user_by_id,
};

pub struct UserController;

impl UserController {
    pub async fn get_all_users(db_pool: web::Data<Pool<Postgres>>) -> impl Responder {
        match handle_get_all_users::handle(db_pool.get_ref()).await {
            Ok(users) => HttpResponse::Ok().json(users),
            Err(err) => HttpResponse::InternalServerError().body(err),
        }
    }

    pub async fn get_user_by_id(
        db_pool: web::Data<Pool<Postgres>>,
        path: web::Path<i32>,
    ) -> impl Responder {
        let query = GetUserByIdQuery {
            user_id: path.into_inner(),
        };

        match handle_get_user_by_id::handle(db_pool.get_ref(), query).await {
            Ok(user) => HttpResponse::Ok().json(user),
            Err(err) => HttpResponse::NotFound().body(err),
        }
    }

    pub async fn create_user(
        db_pool: web::Data<Pool<Postgres>>,
        user_data: web::Json<CreateUserCommand>,
    ) -> impl Responder {
        match handle_create_user::handle(db_pool.get_ref(), user_data.into_inner()).await {
            Ok(user) => HttpResponse::Created().json(user),
            Err(err) => HttpResponse::InternalServerError().body(err),
        }
    }

    pub async fn update_user(
        db_pool: web::Data<Pool<Postgres>>,
        path: web::Path<i32>,
        user_data: web::Json<UpdateUserCommand>,
    ) -> impl Responder {
        let mut command = user_data.into_inner();
        command.user_id = path.into_inner();

        match handle_update_user::handle(db_pool.get_ref(), command).await {
            Ok(_) => HttpResponse::Ok().body("User updated"),
            Err(err) => HttpResponse::InternalServerError().body(err),
        }
    }

    pub async fn delete_user(
        db_pool: web::Data<Pool<Postgres>>,
        path: web::Path<i32>,
    ) -> impl Responder {
        let command = DeleteUserCommand {
            user_id: path.into_inner(),
        };

        match handle_delete_user::handle(db_pool.get_ref(), command).await {
            Ok(_) => HttpResponse::Ok().body("User deleted"),
            Err(err) => HttpResponse::InternalServerError().body(err),
        }
    }
}
