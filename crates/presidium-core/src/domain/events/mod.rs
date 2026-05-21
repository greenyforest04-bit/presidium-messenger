//! # Domain Events
//!
//! Events that represent significant state changes in the domain.
//!
//! Domain events are used for:
//! - Cross-aggregate communication (event-driven architecture)
//! - Audit logging (via observability layer)
//! - Future event sourcing capabilities

use crate::domain::entities::{Message, SessionId, UserId};
use serde::{Deserialize, Serialize};

/// A domain event representing a meaningful state transition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DomainEvent {
    /// A new encrypted session has been established between two users.
    SessionEstablished {
        /// User who initiated the session.
        initiator: UserId,
        /// User who accepted the session.
        responder: UserId,
        /// The newly created session identifier.
        session_id: SessionId,
    },

    /// A message has been successfully encrypted and queued for delivery.
    MessageQueued {
        /// The encrypted message that was queued.
        message: Message,
    },

    /// A message has been delivered to the recipient's device.
    MessageDelivered {
        /// Unique message identifier.
        message_id: String,
        /// Recipient who received the message.
        recipient: UserId,
    },

    /// A message has been read by the recipient.
    MessageRead {
        /// Unique message identifier.
        message_id: String,
        /// Reader of the message.
        reader: UserId,
    },

    /// An existing session has been closed or expired.
    SessionClosed {
        /// The session that was closed.
        session_id: SessionId,
        /// Reason for closure.
        reason: String,
    },
}

impl DomainEvent {
    /// Returns a human-readable label for this event type.
    #[must_use]
    pub fn event_type(&self) -> &'static str {
        match self {
            Self::SessionEstablished { .. } => "session_established",
            Self::MessageQueued { .. } => "message_queued",
            Self::MessageDelivered { .. } => "message_delivered",
            Self::MessageRead { .. } => "message_read",
            Self::SessionClosed { .. } => "session_closed",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_type_labels() {
        let user1 = UserId::new([1u8; 32]);
        let user2 = UserId::new([2u8; 32]);
        let session = SessionId::new("s1");

        let event = DomainEvent::SessionEstablished {
            initiator: user1,
            responder: user2,
            session_id: session,
        };
        assert_eq!(event.event_type(), "session_established");
    }
}
