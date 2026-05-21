//! # Send Message Use Case
//!
//! Orchestrates the full message sending pipeline:
//! 1. Validate the command
//! 2. Moderate content via LLM
//! 3. Create domain Message entity
//! 4. Encrypt via CryptoPort
//! 5. Store locally via StoragePort
//! 6. Transport via MessageTransportPort
//! 7. Emit domain event

use presidium_core::application::ports::{
    CryptoPort, MessageTransportPort, ModerationPort, StoragePort,
};
use presidium_core::domain::entities::{Message, SessionId};
use presidium_core::domain::errors::DomainError;
use presidium_core::domain::events::DomainEvent;
use presidium_core::domain::value_objects::ModerationResult;
use crate::domain::commands::SendMessageCommand;
use crate::domain::dtos::MessageSentResponse;

/// Executes the send message use case.
///
/// This is the primary orchestrator for outgoing messages. It coordinates
/// between crypto, storage, transport, and moderation subsystems.
///
/// # Errors
///
/// Returns a [`DomainError`] if any step in the pipeline fails:
/// - Content blocked by moderation
/// - Encryption failure
/// - Storage failure
/// - Transport failure
pub async fn execute_send_message(
    command: SendMessageCommand,
    crypto: &dyn CryptoPort,
    transport: &dyn MessageTransportPort,
    storage: &dyn StoragePort,
    moderation: &dyn ModerationPort,
) -> Result<MessageSentResponse, DomainError> {
    // Step 1: Validate content is not empty
    if command.content.trim().is_empty() {
        return Err(DomainError::InvalidMessage("message content is empty".into()));
    }

    // Self-message check
    if command.sender_id == command.recipient_id {
        return Err(DomainError::InvalidMessage(
            "self-messages are not supported".into(),
        ));
    }

    // Step 2: Content moderation
    let moderation_result = moderation.moderate(&command.content).await?;
    if let ModerationResult::Blocked { category, reason } = moderation_result {
        tracing::warn!(
            target: "presidium::messaging",
            category = %category,
            "message blocked by moderation"
        );
        // TODO: Create Sarcophagus for blocked content
        return Err(DomainError::InvalidMessage(format!(
            "content blocked ({category}): {reason}"
        )));
    }

    // Step 3: Get or establish session
    let session_id = SessionId::new(format!(
        "session-{}-{}",
        command.sender_id.to_hex(),
        command.recipient_id.to_hex()
    ));

    // Step 4: Create domain message
    let message_id = uuid_placeholder();
    let timestamp_ms = current_timestamp_ms();
    let message = Message::new(
        &message_id,
        command.sender_id.clone(),
        command.recipient_id.clone(),
        session_id.clone(),
        &command.content,
        timestamp_ms,
        0, // Sequence will be assigned by storage
    );

    // Step 5: Encrypt
    let plaintext = command.content.as_bytes();
    let ciphertext = crypto.encrypt_message(&session_id, plaintext).await?;

    // Step 6: Store locally
    storage.store_message(&message).await?;
    storage
        .store_event(&DomainEvent::MessageQueued {
            message: message.clone(),
        })
        .await?;

    // Step 7: Transport
    transport.send(&command.recipient_id, &ciphertext).await?;

    tracing::info!(
        target: "presidium::messaging",
        message_id = %message_id,
        recipient = %command.recipient_id,
        "message sent successfully"
    );

    Ok(MessageSentResponse {
        message_id,
        session_id,
        timestamp_ms,
        delivered_immediately: true,
    })
}

/// Generates a placeholder message ID.
///
/// In production, this will use UUID v4. For Day 1, we generate
/// a random hex string.
fn uuid_placeholder() -> String {
    use rand::RngCore;
    let mut bytes = [0u8; 16];
    rand::rngs::OsRng.fill_bytes(&mut bytes);
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// Returns the current Unix timestamp in milliseconds.
fn current_timestamp_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use presidium_core::domain::entities::UserId;
    use presidium_core::infrastructure::adapters::{
        StubCryptoAdapter, StubModerationAdapter, StubStorageAdapter, StubTransportAdapter,
    };

    fn make_test_command() -> SendMessageCommand {
        SendMessageCommand {
            sender_id: UserId::new([1u8; 32]),
            recipient_id: UserId::new([2u8; 32]),
            content: "Hello, Presidium!".into(),
        }
    }

    #[tokio::test]
    async fn test_send_message_success() {
        let cmd = make_test_command();
        let crypto = StubCryptoAdapter::new();
        let transport = StubTransportAdapter::new();
        let storage = StubStorageAdapter::new();
        let moderation = StubModerationAdapter::new();

        let response = execute_send_message(
            cmd,
            &crypto,
            &transport,
            &storage,
            &moderation,
        )
        .await
        .expect("send message");

        assert!(!response.message_id.is_empty());
        assert!(response.delivered_immediately);
    }

    #[tokio::test]
    async fn test_send_empty_message_rejected() {
        let cmd = SendMessageCommand {
            sender_id: UserId::new([1u8; 32]),
            recipient_id: UserId::new([2u8; 32]),
            content: "   ".into(), // whitespace only
        };
        let crypto = StubCryptoAdapter::new();
        let transport = StubTransportAdapter::new();
        let storage = StubStorageAdapter::new();
        let moderation = StubModerationAdapter::new();

        let result = execute_send_message(cmd, &crypto, &transport, &storage, &moderation).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            DomainError::InvalidMessage(msg) => {
                assert!(msg.contains("empty"));
            }
            other => panic!("expected InvalidMessage, got: {other}"),
        }
    }

    #[tokio::test]
    async fn test_self_message_rejected() {
        let user = UserId::new([1u8; 32]);
        let cmd = SendMessageCommand {
            sender_id: user.clone(),
            recipient_id: user,
            content: "self".into(),
        };
        let crypto = StubCryptoAdapter::new();
        let transport = StubTransportAdapter::new();
        let storage = StubStorageAdapter::new();
        let moderation = StubModerationAdapter::new();

        let result = execute_send_message(cmd, &crypto, &transport, &storage, &moderation).await;
        assert!(result.is_err());
    }
}
