use sqlx::{Pool, Postgres};
use crate::commands::users::delete_user_command::DeleteUserCommand;

pub async fn handle(
    db: &Pool<Postgres>,
    command: DeleteUserCommand,
) -> Result<(), String> {
    sqlx::query!(
        "DELETE FROM users WHERE id = $1",
        command.user_id
    )
    .execute(db)
    .await
    .map(|_| ())
    .map_err(|_| "Failed to delete user".to_string())
}
