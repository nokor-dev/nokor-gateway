use std::sync::Mutex;
use std::collections::HashMap;
use std::time::SystemTime;

// AppState structure to manage routes and cached data
pub struct AppState {
    pub routes: Mutex<HashMap<String, Route>>,   // Store routes mapping
    pub cache: Mutex<HashMap<String, CacheData>>, // Store cached data
}

// CacheData structure to hold the data and its timestamp
#[derive(Clone)]
pub struct CacheData {
    pub data: String,
    pub timestamp: SystemTime,
}

// Route structure to store the URL and the route
#[derive(Clone)]
pub struct Route {
    pub url: String,  // Base URL (e.g., jsonplaceholder)
    pub route: String, // Specific route (e.g., /posts, /users)
}

// Utility function to check if the cache has expired
pub fn is_cache_expired(timestamp: SystemTime, duration: u64) -> bool {
    let now = SystemTime::now();
    if let Ok(elapsed) = now.duration_since(timestamp) {
        elapsed.as_secs() > duration
    } else {
        true // If the timestamp is in the future, consider it expired
    }
}
