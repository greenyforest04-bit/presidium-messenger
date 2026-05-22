use crate::application::ports::{MessageStoragePort, StorageError, StoredMessage};
use async_trait::async_trait;
use presidium_core::domain::{MessageId, UserId};

/// Stub adapter — will be replaced by redb integration
/// TODO(Day 6+): Implement real redb-based persistent storage
pub struct RedbStorageAdapter;

impl Default for RedbStorageAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl RedbStorageAdapter {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl MessageStoragePort for RedbStorageAdapter {
    async fn save_outgoing_message(&self, _msg: StoredMessage) -> Result<(), StorageError> {
        unimplemented!("TODO: Implement real outgoing message persistence with redb")
    }

    async fn save_incoming_message(&self, _msg: StoredMessage) -> Result<(), StorageError> {
        unimplemented!("TODO: Implement real incoming message persistence with redb")
    }

    async fn mark_delivered(&self, _msg_id: &MessageId) -> Result<(), StorageError> {
        unimplemented!("TODO: Implement delivery status update in redb")
    }

    async fn mark_read(&self, _msg_id: &MessageId) -> Result<(), StorageError> {
        unimplemented!("TODO: Implement read status update in redb")
    }

    async fn get_messages_for_user(
        &self, _user_id: &UserId, _limit: u32,
    ) -> Result<Vec<StoredMessage>, StorageError> {
        unimplemented!("TODO: Implement message retrieval from redb")
    }
}
