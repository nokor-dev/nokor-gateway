// src/routes.rs

use crate::app_state::{is_cache_expired, AppState, CacheData};
use actix_web::{Error, web, HttpRequest, HttpResponse};
use std::sync::{Arc, Mutex};
use std::time::SystemTime;
use reqwest::Client;

pub async fn request_handler(
    client: web::Data<Client>,
    app_state: web::Data<Arc<Mutex<AppState>>>,
    path: web::Path<String>,
    req: HttpRequest,  // Accept HttpRequest instead of HeaderMap
) -> Result<HttpResponse, Error> {
    let path_str = path.into_inner();

    // Access headers from the HttpRequest
    let headers = req.headers();

    // Get cache duration from the "X-Cache-Duration" header
    let cache_duration = headers
        .get("X-Cache-Duration")
        .and_then(|val| val.to_str().ok())
        .and_then(|val| val.parse::<u64>().ok())
        .unwrap_or(3); // Default to 3 seconds if not provided

    // Lock the AppState and check for cached data
    let app_state = app_state.lock().unwrap();

    let routes = app_state.routes.lock().unwrap();
    // Check if the path exists in the route mapping
    if let Some(route) = routes.get(&path_str) {
        println!("Request for /{} matched with origin: {}", path_str, route.url);

        if let Some(cached_data) = app_state.cache.lock().unwrap().get(path_str.as_str()) {
            if !is_cache_expired(cached_data.timestamp, cache_duration) {
                println!("Cache found for /{}", path_str);
                // Return cached data if it is not expired
                return Ok(HttpResponse::Ok()
                    .content_type("application/json")
                    .body(cached_data.data.clone()));
            }
        }

        // Build the full URL dynamically based on the mapped route
        let full_url = format!("{}/{}", route.url, route.route);
        // Send GET request to the origin
        let response = client.get(&full_url).send().await;

        match response {
            Ok(resp) => {
                if resp.status().is_success() {
                    let body = resp.text().await.map_err(actix_web::error::ErrorInternalServerError)?;

                    // Cache the response data if necessary
                    let cache_data = CacheData {
                        data: body.clone(),
                        timestamp: SystemTime::now(),
                    };
                    app_state.cache.lock().unwrap().insert(path_str.clone(), cache_data);

                    Ok(HttpResponse::Ok().content_type("application/json").body(body))
                } else {
                    Ok(HttpResponse::InternalServerError().body("Failed to fetch data"))
                }
            }
            Err(e) => {
                Err(actix_web::error::ErrorInternalServerError(e))
            }
        }
    }else {
        // Return 404 if no match is found in the routes
        Ok(HttpResponse::NotFound().body("Not Found"))
    }

}
