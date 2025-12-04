use std::sync::Arc;

use deadpool_redis::Pool as RedisPool;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub redis: RedisPool,
    pub jwt_secret: Arc<Vec<u8>>, // keep as bytes
    pub session_ttl_secs: i64,
    pub refresh_ttl_secs: i64,        // refresh token ttl
    pub max_sessions_per_user: usize, // 多端控制
}

impl AppState {
    pub fn new(db: PgPool, redis: RedisPool, secret: Vec<u8>) -> Self {
        Self {
            db,
            redis,
            jwt_secret: Arc::new(secret),
            session_ttl_secs: 60 * 15,
            refresh_ttl_secs: 60 * 60 * 24 * 7, // 7 days
            max_sessions_per_user: 5,
        }
    }
}
