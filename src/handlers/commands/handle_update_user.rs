use sqlx::{Pool, Postgres};
use crate::commands::users::update_user_command::UpdateUserCommand;

pub async fn handle(
    db: &Pool<Postgres>,
    command: UpdateUserCommand,
) -> Result<(), String> {
    sqlx::query!(
        "UPDATE users SET name = $1, email = $2 WHERE id = $3",
        command.name,
        command.email,
        command.user_id
    )
    .execute(db)
    .await
    .map(|_| ())
    .map_err(|_| "Failed to update user".to_string())
}
