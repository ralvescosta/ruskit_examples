use std::fmt::Display;

use amqp::errors::AmqpError;
use serde::{Deserialize, Serialize};
use tracing::error;

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct SimpleAmqpMessage {
    pub data: String,
    pub created_at: String,
}

impl Display for SimpleAmqpMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SimpleAmqpMessage")
    }
}

impl TryFrom<&[u8]> for SimpleAmqpMessage {
    type Error = AmqpError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        match serde_json::from_slice::<SimpleAmqpMessage>(value) {
            Ok(v) => Ok(v),
            Err(err) => {
                error!(
                    error = err.to_string(),
                    payload = format!("{:?}", value),
                    "parsing error"
                );
                Err(AmqpError::AckMessageDeserializationError(err.to_string()))
            }
        }
    }
}
