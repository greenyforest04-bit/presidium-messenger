//! # Extended Storage Ports
//!
//! Additional storage operations beyond the core [`presidium_core::application::ports::StoragePort`].

use crate::domain::schema::{Contact, Conversation};
use async_trait::async_trait;
use presidium_core::domain::entities::UserId;
use presidium_core::domain::errors::DomainError;

/// Extended storage operations for conversations and contacts.
#[async_trait]
pub trait ExtendedStoragePort: Send + Sync {
    /// Creates or retrieves an existing conversation between two users.
    async fn get_or_create_conversation(
        &self,
        user_a: &UserId,
        user_b: &UserId,
    ) -> Result<Conversation, DomainError>;

    /// Retrieves all conversations ordered by last activity.
    async fn list_conversations(&self) -> Result<Vec<Conversation>, DomainError>;

    /// Saves or updates a contact.
    async fn save_contact(&self, contact: &Contact) -> Result<(), DomainError>;

    /// Retrieves a contact by user ID.
    async fn get_contact(&self, user_id: &UserId) -> Result<Option<Contact>, DomainError>;

    /// Deletes all data associated with a conversation.
    async fn delete_conversation(&self, conversation_id: &str) -> Result<(), DomainError>;
}

#[cfg(test)]
mod tests {
    fn _assert_send_sync<T: Send + Sync>() {}

    #[test]
    fn test_extended_storage_port_is_object_safe() {
        fn _use(_: &dyn crate::application::ports::ExtendedStoragePort) {}
    }
}
