use crate::auth::{
    handlers::{login_handler, refresh_handler, register_handler},
    middleware::AuthLayer,
};
use crate::state::AppState;
use axum::{Router, routing::post};

pub fn create_router(state: AppState) -> Router {
    let public_router = Router::new().route("/api/register", post(register_handler));

    let protected_router = Router::new()
        .route("/api/login", post(login_handler))
        .route("/api/refresh", post(refresh_handler))
        .layer(AuthLayer);

    Router::new()
        .merge(public_router)
        .merge(protected_router)
        .with_state(state)
}
