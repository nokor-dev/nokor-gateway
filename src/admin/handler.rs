use crate::api::handler::{create_route_handler, list, list_routes_from_db, list_routes_handler};
use actix_web::web;

pub fn routes_config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/api")
            .route("/routes", web::post().to(create_route_handler))
            .route("/routes", web::get().to(list_routes_handler))
            .route("/routes/db", web::get().to(list))
            .route("/routes/filter", web::get().to(list_routes_from_db)),
    );
}