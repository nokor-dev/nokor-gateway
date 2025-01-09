// src/main.rs
mod app_state;
mod routes;

use actix_web::{web, App, HttpServer};
use crate::app_state::AppState;
use routes::request_handler;
use reqwest::Client;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the app state (routes)
    let app_state = web::Data::new(AppState::new());

    // Create a reqwest client
    let client = Client::new();

    // Start the HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone()) // Share app state across all workers
            .app_data(web::Data::new(client.clone())) // Pass client to the handler
            .route("/", web::get().to(|| async { "Server is running!" })) // Simple health check route
            .route("/{path}", web::get().to(request_handler)) // Dynamic route based on the path
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}