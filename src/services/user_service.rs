use sqlx::{Pool, Postgres};
use crate::models::user::{User, CreateUserRequest};

pub struct UserService;

impl UserService {
    // Get all users
    pub async fn get_all_users(db_pool: &Pool<Postgres>) -> Result<Vec<User>, String> {
        let result = sqlx::query_as!(
            User,
            "SELECT id, name, email FROM users"
        )
        .fetch_all(db_pool)
        .await;

        result.map_err(|_| "Failed to fetch users".to_string())
    }

    // Get user by ID
    pub async fn get_user_by_id(db_pool: &Pool<Postgres>, user_id: i32) -> Result<User, String> {
        let result = sqlx::query_as!(
            User,
            "SELECT id, name, email FROM users WHERE id = $1",
            user_id
        )
        .fetch_one(db_pool)
        .await;

        result.map_err(|_| "User not found".to_string())
    }

    // Create new user
    pub async fn create_user(db_pool: &Pool<Postgres>, user_data: CreateUserRequest) -> Result<User, String> {
        let result = sqlx::query_as!(
            User,
            "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id, name, email",
            user_data.name,
            user_data.email
        )
        .fetch_one(db_pool)
        .await;

        result.map_err(|_| "Failed to create user".to_string())
    }

    // Update user
    pub async fn update_user(db_pool: &Pool<Postgres>, user_id: i32, user_data: CreateUserRequest) -> Result<(), String> {
        let result = sqlx::query!(
            "UPDATE users SET name = $1, email = $2 WHERE id = $3",
            user_data.name,
            user_data.email,
            user_id
        )
        .execute(db_pool)
        .await;

        result.map(|_| ()).map_err(|_| "Failed to update user".to_string())
    }

    // Delete user
    pub async fn delete_user(db_pool: &Pool<Postgres>, user_id: i32) -> Result<(), String> {
        let result = sqlx::query!("DELETE FROM users WHERE id = $1", user_id)
            .execute(db_pool)
            .await;

        result.map(|_| ()).map_err(|_| "Failed to delete user".to_string())
    }
}
