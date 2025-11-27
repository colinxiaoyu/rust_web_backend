use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct RegisterInput {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginInput {
    pub username: String,
    pub password: String,
}

pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: i32,
    pub username: String,
}

impl From<&User> for UserResponse {
    fn from(user: &User) -> Self {
        UserResponse {
            id: user.id,
            username: user.username.clone(),
        }
    }
}
