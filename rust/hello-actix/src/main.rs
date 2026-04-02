use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;
use std::env;

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello from Actix Web on Partiri!")
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().json(HealthResponse { status: "ok" })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a valid number");

    println!("Listening on 0.0.0.0:{port}");

    HttpServer::new(|| App::new().service(index).service(health))
        .bind(("0.0.0.0", port))?
        .run()
        .await
}
