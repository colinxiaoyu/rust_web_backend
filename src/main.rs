use dotenvy::dotenv;
use std::env;
use tokio::net::TcpListener;
use tracing_subscriber;
use web_backend::{
    db::{init_db_pool, init_redis_pool},
    routes::create_router,
    state::AppState,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let pg_pool = init_db_pool().await;

    let redis_pool = init_redis_pool();

    let jwt_secret = env::var("JWT_SECRET")?.into_bytes();

    let state = AppState::new(pg_pool, redis_pool, jwt_secret);

    // Create router
    let app = create_router(state.clone());

    // Create listener
    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    println!("listening on {}", listener.local_addr()?);

    // Start server (Axum 0.7)
    axum::serve(listener, app).await?;

    Ok(())
}
