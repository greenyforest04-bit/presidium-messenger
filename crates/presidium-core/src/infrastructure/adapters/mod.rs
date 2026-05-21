//! # Adapter Implementations
//!
//! Placeholder adapters that implement the core application ports.
//!
//! These stubs allow the project to compile and test the architecture
//! from Day 1. Real implementations will replace these as each
//! subsystem (crypto, p2p, storage, llm) is developed.

use crate::application::ports::{
    CryptoPort, MessageTransportPort, ModerationPort, PreKeyBundle, StoragePort,
};
use crate::domain::entities::{Message, SessionId, UserId};
use crate::domain::errors::DomainError;
use crate::domain::events::DomainEvent;
use crate::domain::value_objects::ModerationResult;
use async_trait::async_trait;
use std::collections::HashMap;
use tokio::sync::RwLock;

/// Stub implementation of [`CryptoPort`] for development and testing.
///
/// This adapter performs no real encryption — it simply passes
/// plaintext through. **Must be replaced** before any production use.
pub struct StubCryptoAdapter {
    /// In-memory session registry.
    sessions: RwLock<HashMap<String, SessionId>>,
}

impl StubCryptoAdapter {
    /// Creates a new stub crypto adapter.
    #[must_use]
    pub fn new() -> Self {
        Self {
            sessions: RwLock::new(HashMap::new()),
        }
    }
}

impl Default for StubCryptoAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl CryptoPort for StubCryptoAdapter {
    async fn create_pre_key_bundle(&self) -> Result<PreKeyBundle, DomainError> {
        Ok(PreKeyBundle {
            identity_key: vec![0u8; 32],
            signed_pre_key: vec![0u8; 32],
            signed_pre_key_signature: vec![0u8; 64],
            one_time_pre_key: Some(vec![0u8; 32]),
        })
    }

    async fn establish_session(
        &self,
        remote_user: &UserId,
        _bundle: &PreKeyBundle,
    ) -> Result<SessionId, DomainError> {
        let session_id = SessionId::new(format!("stub-session-{}", remote_user.to_hex()));
        self.sessions
            .write()
            .await
            .insert(remote_user.to_hex(), session_id.clone());
        Ok(session_id)
    }

    async fn encrypt_message(
        &self,
        _session_id: &SessionId,
        plaintext: &[u8],
    ) -> Result<Vec<u8>, DomainError> {
        // Stub: no encryption, just clone
        Ok(plaintext.to_vec())
    }

    async fn decrypt_message(
        &self,
        _session_id: &SessionId,
        ciphertext: &[u8],
    ) -> Result<Vec<u8>, DomainError> {
        // Stub: no decryption, just clone
        Ok(ciphertext.to_vec())
    }

    async fn close_session(&self, session_id: &SessionId) -> Result<(), DomainError> {
        self.sessions.write().await.retain(|_, v| v != session_id);
        Ok(())
    }
}

/// Stub implementation of [`MessageTransportPort`].
///
/// Messages are stored in memory and never actually sent over a network.
pub struct StubTransportAdapter {
    /// Inbox for the stub.
    inbox: RwLock<Vec<(UserId, Vec<u8>)>>,
}

impl StubTransportAdapter {
    /// Creates a new stub transport adapter.
    #[must_use]
    pub fn new() -> Self {
        Self {
            inbox: RwLock::new(Vec::new()),
        }
    }
}

impl Default for StubTransportAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl MessageTransportPort for StubTransportAdapter {
    async fn send(&self, recipient: &UserId, payload: &[u8]) -> Result<(), DomainError> {
        self.inbox
            .write()
            .await
            .push((recipient.clone(), payload.to_vec()));
        tracing::debug!(target: "presidium::transport", "stub: queued message for {}", recipient);
        Ok(())
    }

    async fn receive(&self) -> Result<(UserId, Vec<u8>), DomainError> {
        let mut inbox = self.inbox.write().await;
        inbox
            .pop()
            .ok_or_else(|| DomainError::ResourceUnavailable("no messages in stub inbox".into()))
    }
}

/// Stub implementation of [`StoragePort`].
///
/// All data is kept in memory and lost on process exit.
pub struct StubStorageAdapter {
    messages: RwLock<Vec<Message>>,
    events: RwLock<Vec<DomainEvent>>,
}

impl StubStorageAdapter {
    /// Creates a new stub storage adapter.
    #[must_use]
    pub fn new() -> Self {
        Self {
            messages: RwLock::new(Vec::new()),
            events: RwLock::new(Vec::new()),
        }
    }
}

impl Default for StubStorageAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl StoragePort for StubStorageAdapter {
    async fn store_message(&self, message: &Message) -> Result<(), DomainError> {
        self.messages.write().await.push(message.clone());
        Ok(())
    }

    async fn get_message(&self, message_id: &str) -> Result<Message, DomainError> {
        let messages = self.messages.read().await;
        messages
            .iter()
            .find(|m| m.id == message_id)
            .cloned()
            .ok_or_else(|| DomainError::MessageNotFound(message_id.to_string()))
    }

    async fn get_session_messages(
        &self,
        session_id: &SessionId,
    ) -> Result<Vec<Message>, DomainError> {
        let mut result: Vec<Message> = self
            .messages
            .read()
            .await
            .iter()
            .filter(|m| m.session_id == *session_id)
            .cloned()
            .collect();
        result.sort_by_key(|m| m.sequence);
        Ok(result)
    }

    async fn store_event(&self, event: &DomainEvent) -> Result<(), DomainError> {
        self.events.write().await.push(event.clone());
        Ok(())
    }
}

/// Stub implementation of [`ModerationPort`].
///
/// Always returns [`ModerationResult::Safe`].
pub struct StubModerationAdapter;

impl StubModerationAdapter {
    /// Creates a new stub moderation adapter.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Default for StubModerationAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ModerationPort for StubModerationAdapter {
    async fn moderate(&self, _content: &str) -> Result<ModerationResult, DomainError> {
        Ok(ModerationResult::Safe)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_stub_crypto_adapter_lifecycle() {
        let adapter = StubCryptoAdapter::new();
        let user = UserId::new([5u8; 32]);
        let bundle = adapter
            .create_pre_key_bundle()
            .await
            .expect("create bundle");
        let session = adapter
            .establish_session(&user, &bundle)
            .await
            .expect("establish");
        let encrypted = adapter
            .encrypt_message(&session, b"test")
            .await
            .expect("encrypt");
        let decrypted = adapter
            .decrypt_message(&session, &encrypted)
            .await
            .expect("decrypt");
        assert_eq!(decrypted, b"test");
        adapter.close_session(&session).await.expect("close");
    }

    #[tokio::test]
    async fn test_stub_transport_send_receive() {
        let adapter = StubTransportAdapter::new();
        let recipient = UserId::new([1u8; 32]);
        adapter.send(&recipient, b"payload").await.expect("send");
        let (recv_user, data) = adapter.receive().await.expect("receive");
        assert_eq!(recv_user, recipient);
        assert_eq!(data, b"payload");
    }

    #[tokio::test]
    async fn test_stub_storage_roundtrip() {
        let adapter = StubStorageAdapter::new();
        let sender = UserId::new([1u8; 32]);
        let recipient = UserId::new([2u8; 32]);
        let session = SessionId::new("s1");
        let msg = Message::new("m1", sender, recipient, session.clone(), "hello", 100, 0);

        adapter.store_message(&msg).await.expect("store");
        let retrieved = adapter.get_message("m1").await.expect("retrieve");
        assert_eq!(retrieved.content, "hello");

        let session_msgs = adapter
            .get_session_messages(&session)
            .await
            .expect("session msgs");
        assert_eq!(session_msgs.len(), 1);
    }

    #[tokio::test]
    async fn test_stub_moderation_always_safe() {
        let adapter = StubModerationAdapter::new();
        let result = adapter.moderate("any content").await.expect("moderate");
        assert_eq!(result, ModerationResult::Safe);
    }
}
