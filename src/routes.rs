use crate::auth::{
    handlers::{login_handler, refresh_handler},
    middleware::AuthLayer,
};
use crate::state::AppState;
use axum::{Router, routing::post};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/api/login", post(login_handler))
        .route("/api/refresh", post(refresh_handler))
        .with_state(state)
        .layer(AuthLayer)
}
