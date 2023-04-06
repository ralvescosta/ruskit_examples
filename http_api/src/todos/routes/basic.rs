use crate::todos::controllers;
use actix_web::web::{self, ServiceConfig};
use httpw::server::RouteConfig;

pub fn basic_routes() -> RouteConfig {
    |cfg: &mut ServiceConfig| {
        cfg.service(
            web::scope("/v1/todos")
                .service(controllers::post)
                .service(controllers::list)
                .service(controllers::get)
                .service(controllers::delete),
        );
    }
}
