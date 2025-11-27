use crate::{db::get_db_pool, models::user::User};

pub async fn get_user_by_username(username: &str) -> Result<Option<User>, sqlx::Error> {
    let db = get_db_pool();
    let row = sqlx::query_as!(
        User,
        "SELECT id,username, password_hash FROM users WHERE username = $1",
        username
    )
    .fetch_optional(db)
    .await?;
    Ok(row)
}
