use web_backend::{app::create_app, db::init_db_pool};

#[tokio::main]
async fn main() {
    let _db = init_db_pool().await;

    let app = create_app();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
