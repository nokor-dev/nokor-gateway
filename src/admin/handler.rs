use crate::api::handler::{create_route_handler, list_routes_handler};
use actix_web::web;

pub fn routes_config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/api")
            .route("/routes", web::post().to(create_route_handler))
            .route("/routes", web::get().to(list_routes_handler)),
    );
}