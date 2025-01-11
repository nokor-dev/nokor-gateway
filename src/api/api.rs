use serde::Serialize;

// Define a structure to represent the new route details
#[derive(serde::Deserialize)]
pub struct NewApi {
    pub path: String, // Route key
    pub url: String,  // Base URL for the route
    pub route: String, // Path segment for the route
}

// Define a structure to represent the api details
#[derive(sqlx::FromRow, Serialize, Clone)]
pub struct Api {
    pub path: String, // Route key
    pub url: String,  // Base URL for the route
    pub route: String, // Path segment for the route
}

// Route structure to store the URL and the route
#[derive(sqlx::FromRow, Serialize, Clone)]
pub struct Route {
    pub url: String,  // Base URL (e.g., jsonplaceholder)
    pub route: String, // Specific route (e.g., /posts, /users)
    pub path: String, // Route key
}

