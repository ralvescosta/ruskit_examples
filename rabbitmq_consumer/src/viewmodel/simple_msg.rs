use amqp::types::AmqpMessage;
use errors::amqp::AmqpError;
use serde::{Deserialize, Serialize};
use tracing::error;

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct SimpleAmqpMessage {
    pub data: String,
    pub created_at: String,
}

impl SimpleAmqpMessage {
    pub fn from_bytes(b: &[u8]) -> Result<SimpleAmqpMessage, AmqpError> {
        serde_json::from_slice::<SimpleAmqpMessage>(b).map_err(|e| {
            error!(
                error = e.to_string(),
                payload = format!("{:?}", b),
                "parsing error"
            );
            AmqpError::AckMessageDeserializationError(e.to_string())
        })
    }
}

impl AmqpMessage for SimpleAmqpMessage {
    fn get_type(&self) -> String {
        "SimpleAmqpMessage".to_owned()
    }
}

impl SimpleAmqpMessage {
    pub fn get_event() -> String {
        "SimpleAmqpMessage".to_owned()
    }
}
