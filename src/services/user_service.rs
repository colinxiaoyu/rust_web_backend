use crate::{
    models::user::{LoginInput, User, UserResponse},
    repositories::user_repo::get_user_by_username,
    utils::hash::verify_password,
};

#[derive(Debug)]
pub enum LoginError {
    InvalidCredentials,
    ServerError,
}
pub async fn login(payload: &LoginInput) -> Result<UserResponse, LoginError> {
    let record = get_user_by_username(&payload.username)
        .await
        .map_err(|_| LoginError::ServerError)?;

    let Some(record) = record else {
        return Err(LoginError::InvalidCredentials);
    };
    let vaild = verify_password(&payload.password, &record.password_hash);
    if !vaild {
        return Err(LoginError::InvalidCredentials);
    }
    Ok(UserResponse::from(&record))
}
