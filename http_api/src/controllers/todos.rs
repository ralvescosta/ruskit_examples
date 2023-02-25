use crate::viewmodels::ToDoRequest;
use actix_web::{delete, get, post, web::Json, HttpRequest, HttpResponse, Responder};
use httpw::extractors::JwtAuthenticateExtractor;

/// Request to create a new ToDo.
///
/// If the request was registered correctly this endpoint will return 201 Accepted and 4xx/5xx if some error occur.
///
#[utoipa::path(
    request_body = ToDoRequest,
    responses(
        (status = 202, description = "Thing requested successfully", body = ToDoResponse),
        (status = 400, description = "Bad request", body = HttpErrorViewModel),
        (status = 401, description = "Unauthorized", body = HttpErrorViewModel),
        (status = 403, description = "Forbidden", body = HttpErrorViewModel),
        (status = 500, description = "Internal error", body = HttpErrorViewModel)
    ),
    security(
        ("auth" = [])
    )
)]
#[post("/")]
pub async fn post(_thing: Json<ToDoRequest>, _auth: JwtAuthenticateExtractor) -> impl Responder {
    HttpResponse::Ok().body("post::things")
}

#[get("/")]
pub async fn list(_req: HttpRequest, _: JwtAuthenticateExtractor) -> impl Responder {
    HttpResponse::Ok().body("list::things")
}

#[get("/{id}")]
pub async fn get(_req: HttpRequest, _: JwtAuthenticateExtractor) -> impl Responder {
    HttpResponse::Ok().body("get::things")
}

#[delete("/{id}")]
pub async fn delete(_req: HttpRequest, _: JwtAuthenticateExtractor) -> impl Responder {
    HttpResponse::Ok().body("delete::things")
}
