// src/main.rs
mod app_state;
mod routes;

use std::{collections::HashMap, sync::{Arc, Mutex}};

use actix_web::{web, App, HttpServer};
use app_state::Route;
use crate::app_state::AppState;
use routes::request_handler;
use reqwest::Client;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Create a reqwest client
    let client = Client::new();

    let mut routes = HashMap::new();

    // Create the initial app state with routes and cache
    let app_state = Arc::new(Mutex::new(AppState {
        routes: Mutex::new({
            routes.insert("posts".to_string(), Route {
                url: "https://jsonplaceholder.typicode.com".to_string(),
                route: "posts".to_string(),
            });
            routes.insert("users".to_string(), Route {
                url: "https://jsonplaceholder.typicode.com".to_string(),
                route: "users".to_string(),
            });
            routes.insert("comments".to_string(), Route {
                url: "https://jsonplaceholder.typicode.com".to_string(),
                route: "comments".to_string(),
            });
            routes
        }),
        cache: Mutex::new(HashMap::new()),
    }));

    // Start the HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))  // Pass the reqwest client to handler
            .app_data(web::Data::new(app_state.clone()))  // Pass app_state to handler
            .route("/", web::get().to(|| async { "Server is running!" }))  // Add a simple hello route
            .route("/{path}", web::get().to(request_handler))  // Dynamic route for any path
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
