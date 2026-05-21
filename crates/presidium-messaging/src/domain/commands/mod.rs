//! # Command Objects
//!
//! Commands represent user-initiated actions in the messaging domain.
//!
//! Commands are distinct from events: commands are *intentions* that
//! may succeed or fail, while events are *facts* that have occurred.

use presidium_core::domain::entities::UserId;
use serde::{Deserialize, Serialize};

/// Command to send a message to another user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendMessageCommand {
    /// The sender's user ID.
    pub sender_id: UserId,
    /// The recipient's user ID.
    pub recipient_id: UserId,
    /// The plaintext message content.
    pub content: String,
}

/// Command to start a new conversation with a user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartConversationCommand {
    /// The user initiating the conversation.
    pub initiator_id: UserId,
    /// The user being invited to the conversation.
    pub invitee_id: UserId,
}

/// Command to delete a message (local only).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteMessageCommand {
    /// The user requesting deletion.
    pub requester_id: UserId,
    /// The ID of the message to delete.
    pub message_id: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_send_message_command() {
        let sender = UserId::new([1u8; 32]);
        let recipient = UserId::new([2u8; 32]);
        let cmd = SendMessageCommand {
            sender_id: sender,
            recipient_id: recipient,
            content: "Hello, world!".into(),
        };
        assert_eq!(cmd.content, "Hello, world!");
    }
}
