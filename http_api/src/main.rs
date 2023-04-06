mod openapi;
mod todos;

use auth::jwt_manager::auth0::Auth0JwtManager;
use env::{ConfigBuilder, Configs, Empty};
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
    let (app_cfg, mut builder) = ConfigBuilder::new()
        .load_from_aws_secret()
        .otlp()
        .auth0()
        .laze_load();

    logging::setup(&app_cfg)?;

    let cfg = builder.build::<Empty>().await?;

    traces::otlp::setup(&cfg)?;
    metrics::otlp::setup(&cfg)?;

    Ok(cfg)
}
