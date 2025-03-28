use sqlx::{Pool, Postgres};
use crate::models::user::User;

pub async fn handle(
    db: &Pool<Postgres>,
) -> Result<Vec<User>, String> {
    sqlx::query_as!(
        User,
        "SELECT id, name, email FROM users"
    )
    .fetch_all(db)
    .await
    .map_err(|_| "Failed to fetch users".to_string())
}
