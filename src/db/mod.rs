use std::env;

use dotenvy::dotenv;
use once_cell::sync::OnceCell;
use sqlx::{PgPool, postgres::PgPoolOptions};

pub type DbPool = PgPool;

static DB_POOL: OnceCell<PgPool> = OnceCell::new();

pub async fn init_db_pool() -> PgPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    DB_POOL.set(pool.clone()).ok();
    pool
}

pub fn get_db_pool() -> &'static PgPool {
    DB_POOL.get().expect("Database pool not initialized")
}
