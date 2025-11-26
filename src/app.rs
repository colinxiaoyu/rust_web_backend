use axum::{
    Router,
    routing::{get, post},
};
use sqlx::PgPool;

use crate::routes::user;

pub fn create_app(db: PgPool) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/register", post(user::register))
        .with_state(db)
}

async fn root() -> &'static str {
    "Hello world"
}
