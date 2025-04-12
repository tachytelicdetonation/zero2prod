use axum::extract::{Path, Query};
use axum::serve::Serve;
use axum::{Router, http::StatusCode, routing::get, routing::post};
use tokio::net::TcpListener;

#[derive(serde::Deserialize)]
struct FormData {
    name: String,
    email: String,
}

async fn greet(Path(name): Path<String>) -> String {
    format!("Hello {}!", name)
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}

async fn subscribe(_form: Query<FormData>) -> StatusCode {
    StatusCode::OK
}

pub async fn run(
    listener: TcpListener,
) -> Result<Serve<TcpListener, Router, Router>, std::io::Error> {
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/{name}", get(greet))
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe));
    Ok(axum::serve(listener, app))
}
