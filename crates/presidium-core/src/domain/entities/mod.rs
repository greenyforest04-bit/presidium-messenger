//! # Domain Entities
//!
//! Core entities representing the fundamental business objects
//! in the Presidium Messenger system.

use serde::{Deserialize, Serialize};

/// Unique identifier for any Presidium user.
///
/// Wraps a 32-byte Ed25519 public key fingerprint to ensure
/// type safety and prevent mixing up raw byte arrays.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserId(pub [u8; 32]);

impl UserId {
    /// Creates a new [`UserId`] from a 32-byte array.
    #[must_use]
    pub const fn new(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// Returns the raw bytes of this user identifier.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    /// Returns a hexadecimal string representation of this ID.
    #[must_use]
    pub fn to_hex(&self) -> String {
        self.0.iter().map(|b| format!("{b:02x}")).collect()
    }
}

impl std::fmt::Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "UserId({})", self.to_hex())
    }
}

/// Unique identifier for an encrypted messaging session.
///
/// Distinct from [`UserId`] — a single user may have multiple
/// concurrent sessions with different devices or contacts.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SessionId(pub String);

impl SessionId {
    /// Creates a new [`SessionId`] from a string.
    #[must_use]
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

impl std::fmt::Display for SessionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SessionId({})", self.0)
    }
}

/// Represents a single message in the Presidium network.
///
/// Messages are always encrypted before transport — this entity
/// operates on the plaintext domain model. Encryption/decryption
/// is handled by the crypto adapter via the [`CryptoPort`](super::super::application::ports::CryptoPort).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// Unique message identifier (UUID v4).
    pub id: String,
    /// Sender of the message.
    pub sender_id: UserId,
    /// Recipient of the message.
    pub recipient_id: UserId,
    /// Session under which this message was encrypted.
    pub session_id: SessionId,
    /// Message content (plaintext, before encryption).
    pub content: String,
    /// Monotonic timestamp (Unix epoch milliseconds).
    pub timestamp_ms: u64,
    /// Message sequence number within the session.
    pub sequence: u64,
}

impl Message {
    /// Creates a new [`Message`] with the given parameters.
    ///
    /// # Panics
    ///
    /// Panics if `sender_id` equals `recipient_id` (self-messages not allowed).
    #[must_use]
    pub fn new(
        id: impl Into<String>,
        sender_id: UserId,
        recipient_id: UserId,
        session_id: SessionId,
        content: impl Into<String>,
        timestamp_ms: u64,
        sequence: u64,
    ) -> Self {
        assert_ne!(
            sender_id, recipient_id,
            "Self-messages are not supported in Presidium Messenger"
        );
        Self {
            id: id.into(),
            sender_id,
            recipient_id,
            session_id,
            content: content.into(),
            timestamp_ms,
            sequence,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_id_creation_and_display() {
        let bytes = [42u8; 32];
        let user_id = UserId::new(bytes);
        assert_eq!(user_id.to_hex().len(), 64);
        assert!(format!("{user_id}").starts_with("UserId("));
    }

    #[test]
    fn test_user_id_equality() {
        let a = UserId::new([1u8; 32]);
        let b = UserId::new([1u8; 32]);
        let c = UserId::new([2u8; 32]);
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn test_message_creation() {
        let sender = UserId::new([1u8; 32]);
        let recipient = UserId::new([2u8; 32]);
        let session = SessionId::new("test-session");
        let msg = Message::new(
            "msg-1",
            sender.clone(),
            recipient,
            session,
            "Hello",
            1000,
            0,
        );
        assert_eq!(msg.id, "msg-1");
        assert_eq!(msg.sender_id, sender);
        assert_eq!(msg.sequence, 0);
    }

    #[test]
    #[should_panic(expected = "Self-messages are not supported")]
    fn test_self_message_panics() {
        let user = UserId::new([1u8; 32]);
        let session = SessionId::new("session");
        let _ = Message::new("msg-self", user.clone(), user, session, "hi", 0, 0);
    }
}
