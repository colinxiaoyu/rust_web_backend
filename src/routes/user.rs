use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::Deserialize;

use crate::{
    db::{self, DbPool, connect_db},
    utils::hash::hash_password,
};

#[derive(Deserialize)]
pub struct RegisterInput {
    pub username: String,
    pub password: String,
}

pub async fn register(
    State(db): State<DbPool>,
    Json(payload): Json<RegisterInput>,
) -> impl IntoResponse {
    let password_hash = hash_password(&payload.password);
    let result = sqlx::query!(
        "INSERT INTO users (username, password_hash) VALUES ($1, $2)",
        payload.username,
        password_hash
    )
    .execute(&db)
    .await;

    match result {
        Ok(_) => (StatusCode::CREATED, "user created").into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, format!("Error: {}", e)).into_response(),
    }
}
