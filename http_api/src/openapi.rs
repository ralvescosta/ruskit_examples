use crate::controllers::todos;
use httpw::viewmodels::HttpErrorViewModel;
use utoipa::{OpenApi, openapi::{self, security::{SecurityScheme, Http, HttpAuthScheme}}, Modify};
use crate::viewmodels;

#[derive(OpenApi)]
#[openapi(
        paths(
            todos::post,
        ),
        components(
            schemas(
                viewmodels::ToDoRequest, 
                viewmodels::ToDoResponse, 
                HttpErrorViewModel,
            )
        ),
        tags(
            (name = "todos", description = "ToDo's management endpoints.")
        ),
        modifiers(&SecurityAddon)
    )]
pub struct ApiDoc;

pub struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut openapi::OpenApi) {
      let components = openapi.components.as_mut().unwrap();
      components.add_security_scheme("auth", SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)))
    }
}

