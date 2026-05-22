use async_trait::async_trait;
use presidium_core::domain::{MessageId, Timestamp, UserId};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StoredMessage {
    pub id: MessageId,
    pub sender: UserId,
    pub recipient: UserId,
    pub ciphertext: Vec<u8>,
    pub timestamp: Timestamp,
    pub delivered: bool,
    pub read: bool,
}

#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("Save failed: {0}")]
    SaveFailed(String),
    #[error("Read failed: {0}")]
    ReadFailed(String),
    #[error("Update failed: {0}")]
    UpdateFailed(String),
    #[error("Not found: {0}")]
    NotFound(String),
}

#[async_trait]
pub trait MessageStoragePort: Send + Sync {
    async fn save_outgoing_message(&self, msg: StoredMessage) -> Result<(), StorageError>;
    async fn save_incoming_message(&self, msg: StoredMessage) -> Result<(), StorageError>;
    async fn mark_delivered(&self, msg_id: &MessageId) -> Result<(), StorageError>;
    async fn mark_read(&self, msg_id: &MessageId) -> Result<(), StorageError>;
    async fn get_messages_for_user(
        &self, user_id: &UserId, limit: u32,
    ) -> Result<Vec<StoredMessage>, StorageError>;
}
