use axum::serve::Serve;
use axum::{Router, extract::Path, http::StatusCode, response::IntoResponse, routing::get};
use tokio::net::TcpListener;

async fn greet(Path(name): Path<String>) -> String {
    format!("Hello {}!", &name)
}

async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}

pub async fn run(
    listener: TcpListener,
) -> Result<Serve<TcpListener, Router, Router>, std::io::Error> {
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/{name}", get(greet))
        .route("/health_check", get(health_check));
    Ok(axum::serve(listener, app))
}
