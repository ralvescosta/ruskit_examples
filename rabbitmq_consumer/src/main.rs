mod handlers;
mod viewmodel;

use amqp::{
    channel,
    dispatcher::{AmqpDispatcher, Dispatcher},
    exchange::ExchangeDefinition,
    publisher::AmqpPublisher,
    queue::{QueueBinding, QueueDefinition},
    topology::{AmqpTopology, Topology},
};
use env::{self, ConfigBuilder, Empty};
use handlers::SimpleHandler;
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

    let (_, channel) = channel::new_amqp_channel(&cfg).await?;

    let queue_def = QueueDefinition::new(QUEUE)
        .with_dlq()
        .with_retry(18000, 3)
        .durable();
    AmqpTopology::new(channel.clone())
        .queue(&queue_def)
        .exchange(
            &ExchangeDefinition::new(EXCHANGE)
                .durable()
                .kind(&amqp::exchange::ExchangeKind::Direct),
        )
        .queue_binding(
            &QueueBinding::new(QUEUE)
                .exchange(EXCHANGE)
                .routing_key(ROUTING_KEY),
        )
        .install()
        .await?;

    let publisher = AmqpPublisher::new(channel.clone());

    let handler = SimpleHandler::new(publisher);

    let dispatches = AmqpDispatcher::new(channel.clone()).register(
        &queue_def,
        &SimpleAmqpMessage::default(),
        handler,
    );

    let (result,) = tokio::join!(dispatches.consume_blocking());

    for err in result {
        if err.is_err() {
            error!("error");
            panic!("{:?}", err)
        }
    }

    Ok(())
}
