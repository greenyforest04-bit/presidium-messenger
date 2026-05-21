//! # Data Transfer Objects
//!
//! DTOs carry data between the application layer and external interfaces
//! (mobile client, API, etc.).

use presidium_core::domain::entities::{SessionId, UserId};
use serde::{Deserialize, Serialize};

/// Response returned after successfully sending a message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageSentResponse {
    /// The ID of the sent message.
    pub message_id: String,
    /// The session under which the message was sent.
    pub session_id: SessionId,
    /// Monotonic timestamp of when the message was sent.
    pub timestamp_ms: u64,
    /// Whether the message was delivered immediately.
    pub delivered_immediately: bool,
}

/// Summary of a conversation for list views.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationSummary {
    /// The other participant's user ID.
    pub peer_id: UserId,
    /// The last message in the conversation (truncated).
    pub last_message: String,
    /// Monotonic timestamp of the last activity.
    pub last_activity_ms: u64,
    /// Unread message count.
    pub unread_count: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_sent_response() {
        let resp = MessageSentResponse {
            message_id: "m1".into(),
            session_id: SessionId::new("s1"),
            timestamp_ms: 1000,
            delivered_immediately: true,
        };
        assert!(resp.delivered_immediately);
    }

    #[test]
    fn test_conversation_summary() {
        let peer = UserId::new([1u8; 32]);
        let summary = ConversationSummary {
            peer_id: peer,
            last_message: "Hey there...".into(),
            last_activity_ms: 5000,
            unread_count: 3,
        };
        assert_eq!(summary.unread_count, 3);
    }
}
