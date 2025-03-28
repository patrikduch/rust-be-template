use sqlx::{Pool, Postgres};
use crate::commands::users::create_user_command::CreateUserCommand;
use crate::models::user::User;

pub async fn handle(
    db: &Pool<Postgres>,
    command: CreateUserCommand,
) -> Result<User, String> {
    sqlx::query_as!(
        User,
        "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id, name, email",
        command.name,
        command.email
    )
    .fetch_one(db)
    .await
    .map_err(|_| "Failed to create user".to_string())
}
