//! # Storage Adapters
//!
//! Stub in-memory storage adapter implementing both core and extended storage ports.

use crate::application::ports::ExtendedStoragePort;
use crate::domain::schema::{Contact, Conversation};
use async_trait::async_trait;
use presidium_core::application::ports::StoragePort;
use presidium_core::domain::entities::{Message, SessionId, UserId};
use presidium_core::domain::errors::DomainError;
use presidium_core::domain::events::DomainEvent;
use std::collections::HashMap;
use tokio::sync::RwLock;

/// In-memory stub storage adapter for development and testing.
///
/// All data is held in memory and lost when the process exits.
/// Thread-safe via `RwLock`.
pub struct InMemoryStorageAdapter {
    /// Messages indexed by message ID.
    messages: RwLock<HashMap<String, Message>>,
    /// Domain events.
    events: RwLock<Vec<DomainEvent>>,
    /// Conversations indexed by conversation ID.
    conversations: RwLock<HashMap<String, Conversation>>,
    /// Contacts indexed by user ID hex.
    contacts: RwLock<HashMap<String, Contact>>,
}

impl InMemoryStorageAdapter {
    /// Creates a new in-memory storage adapter.
    #[must_use]
    pub fn new() -> Self {
        Self {
            messages: RwLock::new(HashMap::new()),
            events: RwLock::new(Vec::new()),
            conversations: RwLock::new(HashMap::new()),
            contacts: RwLock::new(HashMap::new()),
        }
    }

    /// Generates a conversation key from two user IDs.
    /// The key is always the lexicographically smaller hex first.
    fn conv_key(a: &UserId, b: &UserId) -> String {
        let ha = a.to_hex();
        let hb = b.to_hex();
        if ha < hb {
            format!("{ha}-{hb}")
        } else {
            format!("{hb}-{ha}")
        }
    }
}

impl Default for InMemoryStorageAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl StoragePort for InMemoryStorageAdapter {
    async fn store_message(&self, message: &Message) -> Result<(), DomainError> {
        let mut messages = self.messages.write().await;
        messages.insert(message.id.clone(), message.clone());
        Ok(())
    }

    async fn get_message(&self, message_id: &str) -> Result<Message, DomainError> {
        let messages = self.messages.read().await;
        messages
            .get(message_id)
            .cloned()
            .ok_or_else(|| DomainError::MessageNotFound(message_id.to_string()))
    }

    async fn get_session_messages(
        &self,
        session_id: &SessionId,
    ) -> Result<Vec<Message>, DomainError> {
        let messages = self.messages.read().await;
        let mut result: Vec<Message> = messages
            .values()
            .filter(|m| m.session_id == *session_id)
            .cloned()
            .collect();
        result.sort_by_key(|m| m.sequence);
        Ok(result)
    }

    async fn store_event(&self, event: &DomainEvent) -> Result<(), DomainError> {
        let mut events = self.events.write().await;
        events.push(event.clone());
        Ok(())
    }
}

#[async_trait]
impl ExtendedStoragePort for InMemoryStorageAdapter {
    async fn get_or_create_conversation(
        &self,
        user_a: &UserId,
        user_b: &UserId,
    ) -> Result<Conversation, DomainError> {
        let key = Self::conv_key(user_a, user_b);
        let mut conversations = self.conversations.write().await;

        if let Some(conv) = conversations.get(&key) {
            return Ok(conv.clone());
        }

        let conv = Conversation {
            id: key.clone(),
            participants: (user_a.clone(), user_b.clone()),
            session_id: SessionId::new(format!("conv-{key}")),
            last_activity_ms: 0,
            message_count: 0,
        };
        conversations.insert(key, conv.clone());
        Ok(conv)
    }

    async fn list_conversations(&self) -> Result<Vec<Conversation>, DomainError> {
        let conversations = self.conversations.read().await;
        let mut result: Vec<Conversation> = conversations.values().cloned().collect();
        result.sort_by(|a, b| b.last_activity_ms.cmp(&a.last_activity_ms));
        Ok(result)
    }

    async fn save_contact(&self, contact: &Contact) -> Result<(), DomainError> {
        let mut contacts = self.contacts.write().await;
        contacts.insert(contact.user_id.to_hex(), contact.clone());
        Ok(())
    }

    async fn get_contact(&self, user_id: &UserId) -> Result<Option<Contact>, DomainError> {
        let contacts = self.contacts.read().await;
        Ok(contacts.get(&user_id.to_hex()).cloned())
    }

    async fn delete_conversation(&self, conversation_id: &str) -> Result<(), DomainError> {
        let mut conversations = self.conversations.write().await;
        conversations.remove(conversation_id);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_in_memory_message_roundtrip() {
        let adapter = InMemoryStorageAdapter::new();
        let sender = UserId::new([1u8; 32]);
        let recipient = UserId::new([2u8; 32]);
        let session = SessionId::new("s1");
        let msg = Message::new("m1", sender, recipient, session, "hello", 100, 0);

        adapter.store_message(&msg).await.expect("store");
        let fetched = adapter.get_message("m1").await.expect("get");
        assert_eq!(fetched.content, "hello");
    }

    #[tokio::test]
    async fn test_conversation_create_and_list() {
        let adapter = InMemoryStorageAdapter::new();
        let user1 = UserId::new([1u8; 32]);
        let user2 = UserId::new([2u8; 32]);

        let conv = adapter
            .get_or_create_conversation(&user1, &user2)
            .await
            .expect("create");
        assert_eq!(conv.message_count, 0);

        // Calling again should return the same conversation
        let conv2 = adapter
            .get_or_create_conversation(&user1, &user2)
            .await
            .expect("get");
        assert_eq!(conv.id, conv2.id);

        let list = adapter.list_conversations().await.expect("list");
        assert_eq!(list.len(), 1);
    }

    #[tokio::test]
    async fn test_contact_save_and_retrieve() {
        let adapter = InMemoryStorageAdapter::new();
        let user = UserId::new([3u8; 32]);
        let contact = Contact {
            user_id: user.clone(),
            display_name: "Bob".into(),
            avatar: None,
            verified: true,
            added_at_ms: 1000,
        };

        adapter.save_contact(&contact).await.expect("save");
        let fetched = adapter.get_contact(&user).await.expect("get");
        assert!(fetched.is_some());
        let fetched = fetched.unwrap();
        assert_eq!(fetched.display_name, "Bob");
        assert!(fetched.verified);
    }

    #[tokio::test]
    async fn test_delete_conversation() {
        let adapter = InMemoryStorageAdapter::new();
        let user1 = UserId::new([1u8; 32]);
        let user2 = UserId::new([2u8; 32]);

        let conv = adapter
            .get_or_create_conversation(&user1, &user2)
            .await
            .expect("create");
        adapter.delete_conversation(&conv.id).await.expect("delete");

        let list = adapter.list_conversations().await.expect("list");
        assert_eq!(list.len(), 0);
    }
}
