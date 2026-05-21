//! # Storage Schema Types
//!
//! Domain-level schema definitions for persisted data.

use presidium_core::domain::entities::{SessionId, UserId};
use serde::{Deserialize, Serialize};

/// A stored conversation between two users.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    /// Unique conversation identifier.
    pub id: String,
    /// The two participants in this conversation.
    pub participants: (UserId, UserId),
    /// Session ID associated with this conversation.
    pub session_id: SessionId,
    /// Monotonic timestamp of the last message.
    pub last_activity_ms: u64,
    /// Total number of messages in this conversation.
    pub message_count: u64,
}

/// A stored contact (user profile).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    /// The contact's user ID.
    pub user_id: UserId,
    /// A human-readable display name chosen by the user.
    pub display_name: String,
    /// A Base64-encoded avatar (optional).
    pub avatar: Option<String>,
    /// Whether this contact is verified (identity confirmed).
    pub verified: bool,
    /// Monotonic timestamp when this contact was first added.
    pub added_at_ms: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversation_creation() {
        let user1 = UserId::new([1u8; 32]);
        let user2 = UserId::new([2u8; 32]);
        let session = SessionId::new("s1");
        let conv = Conversation {
            id: "conv-1".into(),
            participants: (user1, user2),
            session_id: session,
            last_activity_ms: 1000,
            message_count: 0,
        };
        assert_eq!(conv.message_count, 0);
        assert_eq!(conv.id, "conv-1");
    }

    #[test]
    fn test_contact_creation() {
        let user = UserId::new([1u8; 32]);
        let contact = Contact {
            user_id: user,
            display_name: "Alice".into(),
            avatar: None,
            verified: false,
            added_at_ms: 500,
        };
        assert_eq!(contact.display_name, "Alice");
        assert!(!contact.verified);
    }
}
