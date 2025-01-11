use std::{collections::{HashMap, HashSet}, ops::Deref, sync::Arc};
use actix_web::{web::Data, FromRequest, HttpRequest, HttpResponse};
use sqlx::{Pool, Postgres, Row, Column};
use futures::future::{ready, Ready};
use crate::errors::AppError;

use super::api::Api;

pub struct ApiRepository {
    pool: Arc<Pool<Postgres>>,
    request: HttpRequest,
    table: String,
}

impl ApiRepository {
    /// Constructor with an explicit table name
    pub fn new(pool: Arc<Pool<Postgres>>, request: HttpRequest, table: String) -> Self {
        Self {
            pool,
            request,
            table,
        }
    }
    
    /// Constructor that extracts the table name from the URL path
    // pub fn new(pool: Arc<Pool<Postgres>>, request: HttpRequest) -> Self {
    //     // Extract table name from the URL path
    //     let table = Self::extract_table_name(&request);
    //     Self::table(pool, request, table)
    // }

    /// Helper method to extract the table name from the URL path
    // fn extract_table_name(request: &HttpRequest) -> String {
    //     // Get the path from the request, e.g., "/apis/subpath"
    //     let path = request.path();

    //     // Split the path into segments and get the first one after the root `/`
    //     let segments: Vec<&str> = path.trim_start_matches('/').split('/').collect();
    //     segments.get(0).unwrap_or(&"default_table").to_string()
    // }

    pub async fn list(&self) -> Result<Vec<HttpResponse>, sqlx::Error> {
        let sql: String = format!("SELECT * FROM {}", self.table);
        let routes = sqlx::query_as::<_, Api>(sql.as_str())
            .fetch_all(&*self.pool)
            .await?;

        let responses: Vec<HttpResponse> = routes.into_iter()
            .map(|route| HttpResponse::Ok().json(route))
            .collect();

        Ok(responses)
    }

    pub async fn list_columns(&self) -> Result<Vec<HttpResponse>, sqlx::Error> {
        let query_string = self.request.query_string();
        let columns_param = query_string
            .split('&')
            .find(|param| param.starts_with("select="))
            .map(|param| param.trim_start_matches("select="))
            .unwrap_or("*");

        // Split the column names by comma
        let columns: Vec<&str> = columns_param.split(',').collect();

        if columns_param != "*" {
            // Validate that the columns exist in your table (for security, ensure no SQL injection)
            let valid_columns = Self::get_valid_columns(&self.pool, &self.table).await?;

            // Check for invalid columns
            let invalid_columns: Vec<String> = columns.iter().filter(|&&col| !valid_columns.contains(col)).map(|&col| col.to_string()).collect();
            if !invalid_columns.is_empty() {
                return Err(sqlx::Error::ColumnNotFound(format!(
                    "Invalid columns requested: {:?}",
                    invalid_columns
                )));
            }
        }

        // Build the dynamic query
        let query = self.build_dynamic_query(&columns);

        // Fetch the data from the database
        let results = Self::fetch_data(&self.pool, &query).await?;

        // Return the results as JSON
        Ok(results.into_iter()
            .map(|row| HttpResponse::Ok().json(row))
            .collect())
    }

    async fn get_valid_columns( pool: &Pool<Postgres>, table: &str) -> Result<HashSet<String>, sqlx::Error> {
        // Query the database to get column names for the specified table
        let query = r#"
            SELECT column_name
            FROM information_schema.columns
            WHERE table_name = $1
        "#;
    
        let rows = sqlx::query(query)
            .bind(table) // Bind the table name to the query
            .fetch_all(pool)
            .await?;
    
        // Collect the column names into a HashSet
        let valid_columns = rows
            .into_iter()
            .filter_map(|row| row.try_get::<String, _>("column_name").ok())
            .collect();
    
        Ok(valid_columns)
    }
    
    pub fn build_dynamic_query(&self, columns: &[&str]) -> String {
        // Build the dynamic SELECT query string
        let columns_str = columns.join(", ");
        format!("SELECT {} FROM {}", columns_str, self.table)
    }

    async fn fetch_data(pool: &Pool<Postgres>, query: &str) -> Result<Vec<HashMap<String, String>>, sqlx::Error> {
        // Execute the dynamic query and return the results
        let rows = sqlx::query(query)
            .fetch_all(pool)
            .await?;

        let mut results = Vec::new();
        for row in rows {
            let mut row_data = HashMap::new();
            // Dynamically map the result row to a HashMap where the key is the column name
            for column in row.columns() {
                if let Ok(value) = row.try_get::<String, _>(column.name()) {
                    row_data.insert(column.name().to_string(), value);
                }
            }
            results.push(row_data);
        }

        Ok(results)
    }

}

impl FromRequest for ApiRepository {
    type Error = AppError;
    type Future = Ready<Result<Self, Self::Error>>;
    fn from_request(
        req: &actix_web::HttpRequest,
        payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let pool_result = Data::<Pool<Postgres>>::from_request(req, payload).into_inner();

        match pool_result {
            Ok(pool) => ready(
                // Ok(ApiRepository::new(
                // pool.deref().clone(),
                // req.clone())
                Ok(ApiRepository::new(
                    pool.deref().clone(),
                    req.clone(),
                    "apis".to_string()
                )
            )),
            _ => ready(Err(AppError::NOT_AUTHORIZED.default())),
        }
    }
}