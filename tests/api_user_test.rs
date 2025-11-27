use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use serde_json::json;
use tower::ServiceExt;
use web_backend::{app::create_app, db::init_db_pool};

#[tokio::test]
async fn test_register_http() {
    let _db = init_db_pool().await;

    let app = create_app();

    let payload = json!({
      "username": "http_test1",
      "password": "123456"
    });

    let request = Request::post("/register")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::CREATED)
}

#[tokio::test]
async fn test_login_http() {
    let _db = init_db_pool().await;

    let app = create_app();

    let payload = json!({
      "username": "http_test1",
      "password": "123456"
    });

    let request = Request::post("/login")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    println!("{:#?}", response);

    assert_eq!(response.status(), StatusCode::OK)
}
