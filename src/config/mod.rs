use std::time::Duration;

use serde::Deserialize;
use dotenv::dotenv;
use color_eyre::Result;
use config::{Config, Environment};
use eyre::WrapErr;
use sqlx::postgres::PgPoolOptions;

#[derive(Debug, Deserialize)]
pub struct Configurations {
    pub host: String,
    pub port: i32,
    pub database_url: String
}

impl Configurations {
    
    pub async fn db_pool(&self) -> Result<sqlx::Pool<sqlx::Postgres>> {
        // Create a connection pool with a timeout of 30 seconds
        let pool = PgPoolOptions::new()
            .acquire_timeout(Duration::from_secs(30))
            .max_connections(5) // djust the number of connections
            .connect(&self.database_url)
            .await
            .wrap_err("Failed to connect to the database")?;

        Ok(pool)
    }

    pub fn from_env() -> Result<Configurations> {
        // Load environment variables from .env file (if it exists)
        dotenv().ok();

        // Create a new Config object using the builder pattern
        let c = Config::builder()
            .add_source(Environment::default()) // Merge environment variables
            .build()?;

        // Attempt to deserialize the configuration directly into the Configurations struct
        c.try_deserialize()
            .context("loading configuration from environment") // Add context for better error handling
    }
}
