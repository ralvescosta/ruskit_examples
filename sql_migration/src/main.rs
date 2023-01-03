use std::{error::Error, sync::Arc};

use env::{Config, ConfigBuilder};
use migrator::{Migrator, SqliteDriver};
use sql_pool::sqlite::conn_pool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cfg: Config = ConfigBuilder::new().use_aws_secrets().build().await?;

    logging::setup(&cfg)?;

    let pool = conn_pool(&cfg)?;
    let pool = Arc::new(pool);

    let driver = SqliteDriver::new(pool.clone());
    let driver = Arc::new(driver);

    let migrator = Migrator::new(driver);

    migrator.exec_up(None, None).await?;

    Ok(())
}