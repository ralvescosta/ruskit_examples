mod openapi;
mod todos;

use auth::jwt_manager::auth0::Auth0JwtManager;
use configs::{Configs, Empty};
use configs_builder::ConfigBuilder;
use httpw::server::HTTPServer;
use openapi::ApiDoc;
use std::error::Error;
use todos::routes as todos_routes;
use utoipa::OpenApi;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cfg = default_setup().await?;

    let doc = ApiDoc::openapi();

    let auth0_manager = Auth0JwtManager::new(&cfg.auth0);

    let server = HTTPServer::new(&cfg.app)
        .register(todos_routes::basic_routes())
        .jwt_manager(auth0_manager)
        .openapi(&doc);

    server.start().await?;

    Ok(())
}

async fn default_setup<'cfg>() -> Result<Configs<Empty>, Box<dyn Error>> {
    let cfg = ConfigBuilder::new()
        .use_aws_secret_manager()
        .otlp()
        .auth0()
        .build::<Empty>()
        .await?;

    traces::otlp::setup(&cfg)?;
    metrics::otlp::setup(&cfg)?;

    Ok(cfg)
}
