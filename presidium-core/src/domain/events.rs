use crate::domain::value_objects::{MessageId, Timestamp, UserId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DomainEvent {
    MessageSent { message_id: MessageId, from: UserId, to: UserId, timestamp: Timestamp },
    MessageDelivered { message_id: MessageId, to: UserId, timestamp: Timestamp },
}
