use axum::{
    Router,
    routing::{get, post},
};
use sqlx::PgPool;

use crate::routes::user;

pub fn create_app(pool: PgPool) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/login", post(user::login))
        .with_state(pool)
}

async fn root() -> &'static str {
    "Hello world"
}
