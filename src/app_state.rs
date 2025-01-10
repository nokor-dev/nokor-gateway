use std::sync::Mutex;
use std::collections::HashMap;
use std::time::SystemTime;

use crate::api::api::Route;

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

// Utility function to check if the cache has expired
pub fn is_cache_expired(timestamp: SystemTime, duration: u64) -> bool {
    let now = SystemTime::now();
    if let Ok(elapsed) = now.duration_since(timestamp) {
        elapsed.as_secs() > duration
    } else {
        true // If the timestamp is in the future, consider it expired
    }
}
