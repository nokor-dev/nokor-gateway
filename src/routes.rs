// src/routes.rs
use actix_web::{web, HttpResponse, error};
use reqwest::Client;
use crate::app_state::AppState;

#[derive(Clone)]
pub struct Route {
    pub url: String,   // Base URL (e.g., jsonplaceholder)
    pub route: String, // Path (e.g., posts, users, comments)
}

pub async fn request_handler(
    client: web::Data<Client>,
    app_state: web::Data<AppState>, // Access to the in-memory routes
    path: web::Path<String>,       // Capture dynamic path from request
) -> Result<HttpResponse, actix_web::Error> {
    let path_str = path.into_inner();

    // Lock the mutex to safely access the route mapping
    let routes = app_state.routes.lock().unwrap();

    // Check if the path exists in the route mapping
    if let Some(route) = routes.get(&path_str) {
        println!("Request for /{} matched with origin: {}", path_str, route.url);

        // Build the full URL dynamically based on the mapped route
        let full_url = format!("{}/{}", route.url, route.route);

        // Send GET request to the origin
        let response = client.get(&full_url).send().await;

        match response {
            Ok(resp) => {
                if resp.status().is_success() {
                    // Return the raw JSON response
                    let body = resp.text().await.map_err(|e| error::ErrorInternalServerError(e))?;
                    Ok(HttpResponse::Ok().content_type("application/json").body(body))
                } else {
                    // Handle unsuccessful responses from the origin
                    Ok(HttpResponse::InternalServerError().body("Failed to fetch data"))
                }
            }
            Err(e) => {
                // Convert `reqwest::Error` into an internal server error response
                Err(error::ErrorInternalServerError(e))
            }
        }
    } else {
        // Return 404 if no match is found in the routes
        Ok(HttpResponse::NotFound().body("Not Found"))
    }
}
