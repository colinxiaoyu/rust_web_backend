use axum::extract::State;
use sqlx::PgPool;

use crate::models::user::User;

pub async fn get_user_by_username(
    username: &str,
    pool: &PgPool,
) -> Result<Option<User>, sqlx::Error> {
    let row = sqlx::query_as!(
        User,
        "SELECT id,username, password_hash FROM users WHERE username = $1",
        username
    )
    .fetch_optional(pool)
    .await?;
    Ok(row)
}
