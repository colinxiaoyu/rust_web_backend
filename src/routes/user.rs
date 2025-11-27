use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use sqlx::PgPool;

use crate::{
    models::user::{LoginInput, RegisterInput},
    services::user_service::{self, login_service},
    utils::hash::hash_password,
};

pub async fn register(db: PgPool, Json(payload): Json<RegisterInput>) -> impl IntoResponse {
    let password_hash: String = hash_password(&payload.password);
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

pub async fn login(
    State(pool): State<PgPool>,
    Json(payload): Json<LoginInput>,
) -> impl IntoResponse {
    match login_service(&payload, &pool).await {
        Ok(record) => (StatusCode::OK, Json(record)).into_response(),
        Err(err) => match err {
            user_service::LoginError::InvalidCredentials => {
                (StatusCode::UNAUTHORIZED, "invalid username or password").into_response()
            }
            user_service::LoginError::ServerError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "server error").into_response()
            }
        },
    }
}
