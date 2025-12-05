use std::env;

use axum::{
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use serde_json::json;
use tower::ServiceExt; // for `oneshot`
use web_backend::db::{init_db_pool, init_redis_pool};
use web_backend::routes::create_router;
use web_backend::state::AppState;

#[tokio::test]
async fn test_login_http() {
    let pg_pool = init_db_pool().await;
    let redis_pool = init_redis_pool();
    let jwt_secret = env::var("JWT_SECRET").unwrap().into_bytes();

    let state = AppState::new(pg_pool, redis_pool, jwt_secret);
    // Create router
    let app = create_router(state.clone());

    let payload = json!({
        "username": "http_test1",
        "password": "123456"
    });

    let request = Request::post("/api/login")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body_bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();

    let body_text = String::from_utf8(body_bytes.to_vec()).unwrap();

    println!("Response body: {}", body_text);
}

#[tokio::test]
async fn test_register() {
    let pg_pool = init_db_pool().await;
    let redis_pool = init_redis_pool();
    let jwt_secret = env::var("JWT_SECRET").unwrap().into_bytes();

    let state = AppState::new(pg_pool, redis_pool, jwt_secret);
    // Create router
    let app = create_router(state.clone());

    let payload = json!({
        "username": "http_test1",
        "password": "123456"
    });

    let request = Request::post("/api/register") // 注意路径！
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body_bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();

    let body_text = String::from_utf8(body_bytes.to_vec()).unwrap();

    println!("Response body: {}", body_text);
}
