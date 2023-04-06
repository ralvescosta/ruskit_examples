use crate::todos::viewmodels::ToDoRequest;
use actix_web::{delete, get, post, web::Json, HttpRequest, HttpResponse, Responder};
use httpw::extractors::JwtAuthenticateExtractor;

/// Request to create a new ToDo.
///
/// If the request was registered correctly this endpoint will return 201 Accepted and 4xx/5xx if some error occur.
///
#[utoipa::path(
    post,
    path = "",
    context_path = "/v1/todos",
    tag = "todos",
    request_body = ToDoRequest,
    responses(
        (status = 202, description = "Todo requested successfully", body = ToDoResponse),
        (status = 400, description = "Bad request", body = HTTPError),
        (status = 401, description = "Unauthorized", body = HTTPError),
        (status = 403, description = "Forbidden", body = HTTPError),
        (status = 500, description = "Internal error", body = HTTPError)
    ),
    security()
)]
#[post("")]
pub async fn post(_thing: Json<ToDoRequest>, _auth: JwtAuthenticateExtractor) -> impl Responder {
    HttpResponse::Ok().body("post::things")
}

/// Request to get all ToDo's that was created.
///
/// If the request was process correctly this endpoint will return 200 Ok and 4xx/5xx if some error occur.
///
#[utoipa::path(
    get,
    path = "",
    context_path = "/v1/todos",
    tag = "todos",
    responses(
        (status = 200, description = "Success", body = Vec<ToDoResponse>),
        (status = 400, description = "Bad request", body = HTTPError),
        (status = 401, description = "Unauthorized", body = HTTPError),
        (status = 403, description = "Forbidden", body = HTTPError),
        (status = 500, description = "Internal error", body = HTTPError)
    ),
    security(
        ("auth" = [])
    )
)]
#[get("")]
pub async fn list(_req: HttpRequest, _: JwtAuthenticateExtractor) -> impl Responder {
    HttpResponse::Ok().body("list::things")
}

/// Request to get a specific ToDo by ID.
///
/// If the request was process correctly this endpoint will return 200 Ok and 4xx/5xx if some error occur.
///
#[utoipa::path(
    get,
    path = "/{id}",
    context_path = "/v1/todos",
    tag = "todos",
    responses(
        (status = 200, description = "Success", body = ToDoResponse),
        (status = 400, description = "Bad request", body = HTTPError),
        (status = 401, description = "Unauthorized", body = HTTPError),
        (status = 403, description = "Forbidden", body = HTTPError),
        (status = 500, description = "Internal error", body = HTTPError)
    ),
    security(
        ("auth" = [])
    )
)]
#[get("/{id}")]
pub async fn get(_req: HttpRequest, _: JwtAuthenticateExtractor) -> impl Responder {
    HttpResponse::Ok().body("get::things")
}

/// Request to delete a specific ToDo by ID.
///
/// If the request was process correctly this endpoint will return 200 Ok and 4xx/5xx if some error occur.
///
#[utoipa::path(
    delete,
    path = "/{id}",
    context_path = "/v1/todos",
    tag = "todos",
    request_body = ToDoRequest,
    responses(
        (status = 200, description = "Deleted", body = ToDoResponse),
        (status = 400, description = "Bad request", body = HTTPError),
        (status = 401, description = "Unauthorized", body = HTTPError),
        (status = 403, description = "Forbidden", body = HTTPError),
        (status = 500, description = "Internal error", body = HTTPError)
    ),
    security(
        ("auth" = [])
    )
)]
#[delete("/{id}")]
pub async fn delete(_req: HttpRequest, _: JwtAuthenticateExtractor) -> impl Responder {
    HttpResponse::Ok().body("delete::things")
}
