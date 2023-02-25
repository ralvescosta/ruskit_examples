use env::{ConfigBuilder, Empty};
use migrator::{Migrator, SqliteDriver};
use sql_pool::sqlite::conn_pool;
use std::{error::Error, sync::Arc};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (app_cfg, mut builder) = ConfigBuilder::new().load_from_aws_secret().laze_load();

    logging::setup(&app_cfg)?;

    let cfg = builder.build::<Empty>().await?;

    let pool = conn_pool(&cfg.sqlite)?;
    let pool = Arc::new(pool);

    let driver = SqliteDriver::new(pool.clone());
    let driver = Arc::new(driver);

    let migrator = Migrator::new(driver);

    migrator.exec_up(None, None).await?;

    Ok(())
}
