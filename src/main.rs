use web_backend::{app::create_app, db::connect_db};

#[tokio::main]
async fn main() {
    let db = connect_db().await;

    let app = create_app(db);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
