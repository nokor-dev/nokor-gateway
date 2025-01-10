mod app_state;
mod config;
mod api;
mod admin;
mod errors;

use std::{collections::HashMap, sync::{Arc, Mutex}};

use actix_web::{web, App, HttpServer};
use actix_web::middleware::Logger;
use api::api::Route;
use config::Configurations;
use reqwest::Client;
use crate::app_state::AppState;

use tracing::{info, instrument};

#[actix_web::main]
#[instrument]
async fn main() -> std::io::Result<()> {
    // Create a reqwest client
    let client = Client::new();
    let config = Configurations::from_env().expect("Server configuration");
    let pool = config.db_pool().await.expect("Database configuration");

    let mut routes = HashMap::new();

    info!("Starting server at http://{}:{}/", config.host, config.port);

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

    // // Create the initial app state with routes and cache
    let app_server = {
        let client = client.clone();
        let app_state = app_state.clone();
        let pool = pool.clone();

        HttpServer::new(move || {
            App::new()
                .wrap(Logger::default())
                .app_data(web::Data::new(client.clone()))  // Pass the reqwest client to handler
                .app_data(web::Data::new(app_state.clone()))  // Pass app_state to handler
                .app_data(web::Data::new(pool.clone())) // Pass the database pool to handler
                // .route("/", web::get().to(|| async { "Server is running!" }))  // Add a simple hello route
                // .route("/{path}", web::get().to(api_handler))  // Dynamic route for any path
                .configure(api::handler::app_config)  // Configure the app routes
        })
        .bind("0.0.0.0:8080")?
        .run()
    };

    // Start the HTTP server for the admin port
    let admin_server = {
        let client = client.clone();
        let app_state = app_state.clone();
        let pool = pool.clone();

        HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(client.clone()))  // Pass the reqwest client
                .app_data(web::Data::new(app_state.clone())) // Pass app_state
                .app_data(web::Data::new(pool.clone())) // Pass the database pool to handler
                .configure(admin::handler::routes_config)   // Configure the admin routes
        })
        .bind("0.0.0.0:8081")?
        .run()
    };

    // Run both servers concurrently
    futures::try_join!(app_server, admin_server)?;

    Ok(())
}
