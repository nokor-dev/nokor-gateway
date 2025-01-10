// Define a structure to represent the new route details
#[derive(serde::Deserialize)]
pub struct NewApi {
    pub name: String, // Route key
    pub url: String,  // Base URL for the route
    pub route: String, // Path segment for the route
}