use sqlx::{Pool, Postgres};
use crate::queries::users::get_user_by_id_query::GetUserByIdQuery;
use crate::models::user::User;

pub async fn handle(
    db: &Pool<Postgres>,
    query: GetUserByIdQuery,
) -> Result<User, String> {
    sqlx::query_as!(
        User,
        "SELECT id, name, email FROM users WHERE id = $1",
        query.user_id
    )
    .fetch_one(db)
    .await
    .map_err(|_| "User not found".to_string())
}
