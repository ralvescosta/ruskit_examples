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
use configs::Empty;
use configs_builder::ConfigBuilder;
use handlers::SimpleHandler;
use std::error::Error;
use tracing::error;
use viewmodel::SimpleAmqpMessage;

const EXCHANGE: &str = "random-exchange";
const QUEUE: &str = "random-queue";
const ROUTING_KEY: &str = "random-routing-key";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cfgs = ConfigBuilder::new()
        .use_aws_secret_manager()
        .amqp()
        .health()
        .build::<Empty>()
        .await?;

    traces::otlp::setup(&cfgs)?;

    let (_, channel) = channel::new_amqp_channel(&cfgs).await?;

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
