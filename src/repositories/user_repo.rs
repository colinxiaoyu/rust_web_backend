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

pub async fn exist_by_username(pool: &PgPool, username: &str) -> sqlx::Result<Option<bool>> {
    sqlx::query_scalar!(
        r#"SELECT EXISTS(SELECT 1 FROM users WHERE username = $1)"#,
        username
    )
    .fetch_one(pool)
    .await
}

pub async fn register_by_username_password_hash(
    pool: &PgPool,
    username: &str,
    password_hash: &str,
) -> sqlx::Result<Option<i64>> {
    sqlx::query_scalar!(
        r#"
        INSERT INTO users (username, password_hash)
        VALUES ($1, $2)
        RETURNING id
        "#,
        username,
        password_hash
    )
    .fetch_optional(pool)
    .await
}
