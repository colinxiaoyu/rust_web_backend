use axum::{
    Router,
    routing::{get, post},
};
use sqlx::PgPool;

use crate::{db::get_db_pool, routes::user};

pub fn create_app() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/register", post(user::register))
        .route("/login", post(user::login))
}

async fn root() -> &'static str {
    "Hello world"
}
