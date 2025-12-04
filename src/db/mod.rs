use std::env;

use deadpool_redis::{Config, Pool, Runtime};
use dotenvy::dotenv;
use sqlx::{PgPool, postgres::PgPoolOptions};

pub type DbPool = PgPool;
type RedisPool = Pool;

pub async fn init_db_pool() -> PgPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    pool
}

pub fn init_redis_pool() -> RedisPool {
    dotenv().ok();
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");
    let cfg = Config::from_url(redis_url);
    cfg.create_pool(Some(Runtime::Tokio1)).unwrap()
}
