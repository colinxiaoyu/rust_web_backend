use axum::{Json, http::StatusCode, response::IntoResponse};

use crate::{
    db::get_db_pool,
    models::user::{LoginInput, RegisterInput},
    utils::hash::{hash_password, verify_password},
};

pub async fn register(Json(payload): Json<RegisterInput>) -> impl IntoResponse {
    let db = get_db_pool();
    let password_hash: String = hash_password(&payload.password);
    let result = sqlx::query!(
        "INSERT INTO users (username, password_hash) VALUES ($1, $2)",
        payload.username,
        password_hash
    )
    .execute(db)
    .await;

    match result {
        Ok(_) => (StatusCode::CREATED, "user created").into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, format!("Error: {}", e)).into_response(),
    }
}

pub async fn login(Json(payload): Json<LoginInput>) -> impl IntoResponse {
    let db = get_db_pool();

    let result = sqlx::query!(
        "SELECT password_hash FROM users WHERE username = $1",
        payload.username
    )
    .fetch_one(db)
    .await;

    match result {
        Ok(record) => {
            if verify_password(&payload.password, &record.password_hash) {
                (StatusCode::OK, "login successful").into_response()
            } else {
                (StatusCode::UNAUTHORIZED, "invalid credentials").into_response()
            }
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Error: {}", e)).into_response(),
    }
}
