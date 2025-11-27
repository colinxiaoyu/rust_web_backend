use axum::{
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use serde_json::json;
use tower::ServiceExt;
use web_backend::{app::create_app, db::init_db_pool};

#[tokio::test]
async fn test_login_http() {
    let db = init_db_pool().await;

    let app = create_app(db);

    let payload = json!({
      "username": "http_test1",
      "password": "123456"
    });

    let request = Request::post("/login")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body_bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body_text = String::from_utf8(body_bytes.to_vec()).unwrap();
    println!("Response body: {}", body_text);
}
