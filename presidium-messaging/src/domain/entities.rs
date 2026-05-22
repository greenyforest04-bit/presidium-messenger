use presidium_core::domain::{MessageId, Timestamp, UserId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageStatus {
    Pending,
    Sent,
    Delivered,
    Read,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: MessageId,
    pub sender: UserId,
    pub recipient: UserId,
    pub ciphertext: Vec<u8>,
    pub timestamp: Timestamp,
    pub status: MessageStatus,
}
