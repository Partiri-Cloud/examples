#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;
use serde::Serialize;
use std::env;
use std::net::Ipv4Addr;

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
}

#[get("/")]
fn index() -> &'static str {
    "Hello from Rocket on Partiri!"
}

#[get("/health")]
fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}

#[launch]
fn rocket() -> _ {
    let port: u16 = env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(10000);

    let config = rocket::Config {
        port,
        address: Ipv4Addr::new(0, 0, 0, 0).into(),
        ..Default::default()
    };

    rocket::custom(config).mount("/", routes![index, health])
}
