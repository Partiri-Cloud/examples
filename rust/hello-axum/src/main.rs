use axum::{routing::get, Json, Router};
use serde::Serialize;
use std::env;
use tokio::net::TcpListener;

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
}

async fn index() -> &'static str {
    "Hello from Axum on Partiri!"
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}

#[tokio::main]
async fn main() {
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "10000".to_string())
        .parse()
        .expect("PORT must be a valid number");

    let app = Router::new()
        .route("/", get(index))
        .route("/health", get(health));

    let addr = format!("0.0.0.0:{port}");
    let listener = TcpListener::bind(&addr).await.expect("failed to bind");
    println!("Listening on {addr}");

    axum::serve(listener, app).await.expect("server error");
}
