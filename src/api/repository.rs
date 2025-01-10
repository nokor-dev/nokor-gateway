use std::{ops::Deref, sync::Arc};
use actix_web::{web::Data, FromRequest};
use sqlx::{Pool, Postgres};
use futures::future::{ready, Ready};
use crate::errors::AppError; // Add this line to import AppError

use super::api::Route;

pub struct ApiRepository {
    pool: Arc<Pool<Postgres>>
}

impl ApiRepository {
    pub fn new(pool: Arc<Pool<Postgres>>) -> Self {
        Self { pool }
    }

    pub async fn list(&self) -> Result<Vec<Route>, sqlx::Error> {
        let users = sqlx::query_as::<_, Route>("SELECT * FROM apis")
            .fetch_all(&*self.pool)
            .await?;

        Ok(users)
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
            Ok(pool) => ready(Ok(ApiRepository::new(pool.deref().clone()))),
            _ => ready(Err(AppError::NOT_AUTHORIZED.default())),
        }
    }
}