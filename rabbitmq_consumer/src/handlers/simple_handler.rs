use crate::viewmodel::SimpleAmqpMessage;
use amqp::{
    dispatcher::ConsumerHandler,
    errors::AmqpError,
    publisher::{Payload, Publisher},
};
use async_trait::async_trait;
use opentelemetry::Context;
use serde::Serialize;
use std::{collections::BTreeMap, fmt::Display, sync::Arc};
use tracing::info;

pub struct SimpleHandler {
    publisher: Arc<dyn Publisher>,
}

impl SimpleHandler {
    pub fn new(publisher: Arc<dyn Publisher>) -> Arc<SimpleHandler> {
        Arc::new(SimpleHandler { publisher })
    }
}

#[derive(Serialize)]
pub struct SimpleMessage {}

impl Display for SimpleMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SimpleMessage")
    }
}

#[async_trait]
impl ConsumerHandler for SimpleHandler {
    async fn exec(&self, ctx: &Context, data: &[u8]) -> Result<(), AmqpError> {
        let received = SimpleAmqpMessage::try_from(data)?;

        info!("amqp message received {:?}", received);

        let params = BTreeMap::default();
        self.publisher
            .simple_publish(
                ctx,
                "",
                &Payload::new(&SimpleMessage {}).unwrap(),
                Some(&params),
            )
            .await
            .unwrap();
        Ok(())
    }
}
