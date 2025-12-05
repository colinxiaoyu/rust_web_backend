pub struct RequiredPermission(pub &'static str);

use crate::services::auth_service::{login, logout_all, refresh_tokens, register};
use crate::state::AppState;
use axum::Json;
use axum::extract::State;
use axum::response::IntoResponse;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct LoginInput {
    pub username: String,
    pub password: String,
}

pub async fn login_handler(
    State(state): State<AppState>,
    Json(payload): Json<LoginInput>,
) -> impl IntoResponse {
    match login(&payload.username, &payload.password, &state).await {
        Ok(r) => Json(json!({
            "access_token": r.access_token,
            "refresh_token": r.refresh_token,
            "user": r.user
        }))
        .into_response(),
        Err(e) => Json(json!({ "error": e.to_string() })).into_response(),
    }
}

#[derive(Deserialize)]
pub struct RefreshInput {
    pub refresh_token: String,
}

pub async fn refresh_handler(
    State(state): State<AppState>,
    Json(payload): Json<RefreshInput>,
) -> impl IntoResponse {
    match refresh_tokens(&payload.refresh_token, &state).await {
        Ok(r) => Json(json!({
            "access_token": r.access_token,
            "refresh_token": r.refresh_token,
            "user": r.user
        }))
        .into_response(),
        Err(e) => Json(json!({ "error": e.to_string() })).into_response(),
    }
}

#[derive(Deserialize)]
pub struct LogoutInput {
    pub user_id: i64,
}

pub async fn logout_handler(
    State(state): State<AppState>,
    Json(payload): Json<LogoutInput>,
) -> impl IntoResponse {
    match logout_all(payload.user_id, &state).await {
        Ok(_) => Json(json!({"ok": true})).into_response(),
        Err(e) => Json(json!({"error": format!("{}", e)})).into_response(),
    }
}

#[derive(Deserialize)]
pub struct RegisterInput {
    pub username: String,
    pub password: String,
}

pub async fn register_handler(
    State(state): State<AppState>,
    Json(payload): Json<RegisterInput>,
) -> impl IntoResponse {
    match register(&payload.username, &payload.password, &state).await {
        Ok(id) => Json(json!({"ok": true,"id":id})).into_response(),
        Err(e) => Json(json!({"error": format!("{}", e)})).into_response(),
    }
}
