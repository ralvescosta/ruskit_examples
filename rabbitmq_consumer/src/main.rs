mod handlers;
mod viewmodel;

use amqp::{
    client::AmqpImpl,
    dispatcher::Dispatcher,
    topology::{AmqpTopology, ExchangeDefinition, QueueDefinition},
};
use env::{self, ConfigBuilder, Empty};
use handlers::SimpleHandler;
use health_readiness::HealthReadinessServer;
use logging;
use std::error::Error;
use tracing::error;
use viewmodel::SimpleAmqpMessage;

const EXCHANGE: &str = "random-exchange";
const QUEUE: &str = "random-queue";
const ROUTING_KEY: &str = "random-routing-key";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (app_cfg, mut builder) = ConfigBuilder::new()
        .load_from_aws_secret()
        .amqp()
        .health()
        .laze_load();

    logging::setup(&app_cfg)?;

    let cfg = builder.build::<Empty>().await?;

    traces::otlp::setup(&cfg)?;

    let amqp = AmqpImpl::new(&cfg).await?;
    let mut dispatches = Dispatcher::new(amqp.clone());

    let queue = QueueDefinition::name(QUEUE)
        .with_dlq()
        .with_retry(18000, 3)
        .binding(EXCHANGE, ROUTING_KEY);

    AmqpTopology::new(amqp.clone())
        .exchange(ExchangeDefinition::name(EXCHANGE).direct())
        .queue(queue.clone())
        .install()
        .await?;

    let handler = SimpleHandler::new();
    dispatches
        .declare(queue.clone(), SimpleAmqpMessage::get_event(), handler)
        .expect("unexpected error while registering dispatch");

    let health_readiness =
        HealthReadinessServer::new(&cfg.health_readiness).rabbitmq(amqp.connection());

    match tokio::join!(health_readiness.run(), dispatches.consume_blocking()) {
        (Err(e), _) => {
            error!(error = e.to_string(), "error");
            panic!("{:?}", e)
        }
        (Ok(_), errors) => {
            for err in errors {
                if err.is_err() {
                    error!("error");
                    panic!("{:?}", err)
                }
            }
        }
    }

    Ok(())
}
