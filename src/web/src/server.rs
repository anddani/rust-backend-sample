use axum::http::StatusCode;
use axum::Router;
use axum::routing::get;

pub async fn run() {
    let app = Router::new()
        .route("/health", get(health));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap()
}

async fn health() -> Result<String, StatusCode> {
    Ok("Healthy!".into())
}