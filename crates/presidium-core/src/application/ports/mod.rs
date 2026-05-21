//! # Application Ports
//!
//! Trait definitions that form the boundary between the application
//! core and infrastructure adapters.
//!
//! These ports define the contracts that all infrastructure
//! implementations must fulfill. The domain logic depends only
//! on these abstractions, never on concrete implementations.

use crate::domain::entities::{Message, SessionId, UserId};
use crate::domain::errors::DomainError;
use crate::domain::events::DomainEvent;
use async_trait::async_trait;

/// Port for end-to-end encryption operations.
///
/// Implementations wrap concrete cryptographic libraries (e.g., libsignal-protocol-rust)
/// and provide a domain-friendly interface for session management and
/// message encryption/decryption.
///
/// # Design Notes
///
/// - All operations are async to support hardware-backed key stores.
/// - Errors are mapped to [`DomainError`] to keep the domain clean.
/// - Implementations must never log or expose secret key material.
#[async_trait]
pub trait CryptoPort: Send + Sync {
    /// Creates a new pre-key bundle for the current user.
    ///
    /// This bundle contains the identity key, signed pre-key, and one-time pre-keys
    /// needed by other users to initiate an encrypted session.
    async fn create_pre_key_bundle(&self) -> Result<PreKeyBundle, DomainError>;

    /// Establishes a new encrypted session with a remote user.
    ///
    /// Uses the PQXDH key agreement protocol (Signal) to derive
    /// shared secrets for forward-secret communication.
    async fn establish_session(
        &self,
        remote_user: &UserId,
        bundle: &PreKeyBundle,
    ) -> Result<SessionId, DomainError>;

    /// Encrypts a plaintext message for an established session.
    ///
    /// Returns the ciphertext (header + body) ready for transport.
    async fn encrypt_message(
        &self,
        session_id: &SessionId,
        plaintext: &[u8],
    ) -> Result<Vec<u8>, DomainError>;

    /// Decrypts a ciphertext message using an established session.
    ///
    /// Performs Double Ratchet decryption and returns the plaintext.
    async fn decrypt_message(
        &self,
        session_id: &SessionId,
        ciphertext: &[u8],
    ) -> Result<Vec<u8>, DomainError>;

    /// Closes and deletes a session, purging all key material.
    async fn close_session(&self, session_id: &SessionId) -> Result<(), DomainError>;
}

/// Port for message transport over the P2P network.
///
/// Implementations handle the actual delivery of encrypted messages
/// using the `libp2p` stack (`GossipSub`, direct messages, etc.).
#[async_trait]
pub trait MessageTransportPort: Send + Sync {
    /// Sends an encrypted payload to the specified recipient.
    ///
    /// The payload is already encrypted — this port only handles transport.
    async fn send(&self, recipient: &UserId, payload: &[u8]) -> Result<(), DomainError>;

    /// Receives the next available encrypted message.
    ///
    /// This is an async stream-like operation that yields messages
    /// as they arrive from the network.
    async fn receive(&self) -> Result<(UserId, Vec<u8>), DomainError>;
}

/// Port for persistent storage of messages, sessions, and state.
///
/// Implementations may use `SQLite`, `Sled`, `RocksDB`, or any other
/// storage backend suitable for the target platform.
#[async_trait]
pub trait StoragePort: Send + Sync {
    /// Stores a message in the local database.
    async fn store_message(&self, message: &Message) -> Result<(), DomainError>;

    /// Retrieves a message by its unique ID.
    async fn get_message(&self, message_id: &str) -> Result<Message, DomainError>;

    /// Retrieves all messages in a given session, ordered by sequence.
    async fn get_session_messages(
        &self,
        session_id: &SessionId,
    ) -> Result<Vec<Message>, DomainError>;

    /// Persists a domain event for audit/event-sourcing purposes.
    async fn store_event(&self, event: &DomainEvent) -> Result<(), DomainError>;
}

/// Port for content moderation via on-device LLM.
///
/// Implementations invoke a local GGUF model (Gemma-2B / Phi-3)
/// to classify content before delivery.
#[async_trait]
pub trait ModerationPort: Send + Sync {
    /// Analyzes message content for policy violations.
    ///
    /// Returns a [`ModerationResult`](crate::domain::value_objects::ModerationResult)
    /// indicating whether the content is safe, flagged, or blocked.
    async fn moderate(&self, content: &str) -> Result<ModerationResult, DomainError>;
}

// Re-export for convenience
use crate::domain::value_objects::ModerationResult;

/// A pre-key bundle containing public key material for session establishment.
///
/// This is a domain-level representation — concrete serialization
/// for wire format is handled by the infrastructure adapter.
#[derive(Debug, Clone)]
pub struct PreKeyBundle {
    /// The identity public key of the bundle owner.
    pub identity_key: Vec<u8>,
    /// The signed pre-key.
    pub signed_pre_key: Vec<u8>,
    /// Signature of the signed pre-key by the identity key.
    pub signed_pre_key_signature: Vec<u8>,
    /// A one-time pre-key (may be empty if exhausted).
    pub one_time_pre_key: Option<Vec<u8>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A mock crypto port for testing purposes.
    struct MockCryptoPort;

    #[async_trait]
    impl CryptoPort for MockCryptoPort {
        async fn create_pre_key_bundle(&self) -> Result<PreKeyBundle, DomainError> {
            Ok(PreKeyBundle {
                identity_key: vec![1; 32],
                signed_pre_key: vec![2; 32],
                signed_pre_key_signature: vec![3; 64],
                one_time_pre_key: Some(vec![4; 32]),
            })
        }

        async fn establish_session(
            &self,
            _remote_user: &UserId,
            _bundle: &PreKeyBundle,
        ) -> Result<SessionId, DomainError> {
            Ok(SessionId::new("test-session"))
        }

        async fn encrypt_message(
            &self,
            _session_id: &SessionId,
            plaintext: &[u8],
        ) -> Result<Vec<u8>, DomainError> {
            Ok(plaintext.to_vec())
        }

        async fn decrypt_message(
            &self,
            _session_id: &SessionId,
            ciphertext: &[u8],
        ) -> Result<Vec<u8>, DomainError> {
            Ok(ciphertext.to_vec())
        }

        async fn close_session(&self, _session_id: &SessionId) -> Result<(), DomainError> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_mock_crypto_port_roundtrip() {
        let crypto = MockCryptoPort;
        let bundle = crypto
            .create_pre_key_bundle()
            .await
            .expect("bundle creation");
        assert_eq!(bundle.identity_key.len(), 32);

        let user = UserId::new([1u8; 32]);
        let session = crypto
            .establish_session(&user, &bundle)
            .await
            .expect("session establishment");
        assert_eq!(session.0, "test-session");

        let plaintext = b"hello presidium";
        let ciphertext = crypto
            .encrypt_message(&session, plaintext)
            .await
            .expect("encrypt");
        let decrypted = crypto
            .decrypt_message(&session, &ciphertext)
            .await
            .expect("decrypt");
        assert_eq!(decrypted, plaintext);

        crypto.close_session(&session).await.expect("close session");
    }
}
