use serde::Serialize;

// Define a structure to represent the new route details
#[derive(serde::Deserialize)]
pub struct NewApi {
    pub name: String, // Route key
    pub url: String,  // Base URL for the route
    pub route: String, // Path segment for the route
}

// Route structure to store the URL and the route
#[derive(Clone)]
#[derive(sqlx::FromRow, Serialize)]
pub struct Route {
    pub url: String,  // Base URL (e.g., jsonplaceholder)
    pub route: String, // Specific route (e.g., /posts, /users)
}
