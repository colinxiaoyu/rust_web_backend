use crate::{
    models::user::{LoginInput, User, UserResponse},
    repositories::user_repo::get_user_by_username,
    utils::hash::verify_password,
};
use sqlx::PgPool;

#[derive(Debug)]
pub enum LoginError {
    InvalidCredentials,
    ServerError,
}
pub async fn login_service(
    payload: &LoginInput,
    pool: &PgPool,
) -> Result<UserResponse, LoginError> {
    let record = get_user_by_username(&payload.username, pool)
        .await
        .map_err(|_| LoginError::ServerError)?;

    let Some(record) = record else {
        return Err(LoginError::InvalidCredentials);
    };
    let valid = verify_password(&payload.password, &record.password_hash);
    if !valid {
        return Err(LoginError::InvalidCredentials);
    }
    Ok(UserResponse::from(&record))
}
