use crate::models::user::User;
use sqlx::PgPool;

pub async fn get_user_by_username(pool: &PgPool, username: &str) -> sqlx::Result<Option<User>> {
    sqlx::query_as!(
        User,
        r#"SELECT id, username, password_hash, disabled FROM users WHERE username = $1"#,
        username
    )
    .fetch_optional(pool)
    .await
}

pub async fn get_user_by_id(pool: &PgPool, id: i64) -> sqlx::Result<Option<User>> {
    sqlx::query_as!(
        User,
        r#"SELECT id, username, password_hash, disabled FROM users WHERE id = $1"#,
        id
    )
    .fetch_optional(pool)
    .await
}
