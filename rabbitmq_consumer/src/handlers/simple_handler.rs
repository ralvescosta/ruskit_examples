use crate::viewmodel::SimpleAmqpMessage;
use amqp::{errors::AmqpError, topology::ConsumerHandler};
use async_trait::async_trait;
use opentelemetry::Context;
use std::sync::Arc;
use tracing::info;

pub struct SimpleHandler {}

impl SimpleHandler {
    pub fn new() -> Arc<SimpleHandler> {
        Arc::new(SimpleHandler {})
    }
}

#[async_trait]
impl ConsumerHandler for SimpleHandler {
    async fn exec(&self, _ctx: &Context, data: &[u8]) -> Result<(), AmqpError> {
        let received = SimpleAmqpMessage::from_bytes(data)?;

        info!("amqp message received {:?}", received);
        Ok(())
    }
}
