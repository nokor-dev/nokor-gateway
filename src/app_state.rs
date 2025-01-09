// src/app_state.rs
use std::collections::HashMap;
use std::sync::Mutex;
use crate::routes::Route;

pub struct AppState {
    pub routes: Mutex<HashMap<String, Route>>,
}

impl AppState {
    pub fn new() -> Self {
        let mut routes = HashMap::new();

        // Defining some example routes (this can be expanded)
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

        AppState {
            routes: Mutex::new(routes),
        }
    }
}