use crate::controllers::todos;
use actix_web::web::{self, ServiceConfig};
use httpw::server::RouteConfig;

pub fn todos_routes() -> RouteConfig {
    |cfg: &mut ServiceConfig| {
        cfg.service(
            web::scope("/v1/todos")
                .service(todos::post)
                .service(todos::list)
                .service(todos::get)
                .service(todos::delete),
        );
    }
}
