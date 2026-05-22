use crate::domain::UserId;
use async_trait::async_trait;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ContentVerdict {
    Safe,
    Unsafe(String),
    NeedsReview,
}

#[derive(Debug, thiserror::Error)]
pub enum ModerationError {
    #[error("Moderation check failed: {0}")]
    CheckFailed(String),
    #[error("Sarcophagus creation failed: {0}")]
    SarcophagusFailed(String),
}

#[async_trait]
pub trait ModerationPort: Send + Sync {
    async fn check_message(
        &self, sender: &UserId, plaintext: &str,
    ) -> Result<ContentVerdict, ModerationError>;
    async fn create_sarcophagus(
        &self, offender: &UserId, evidence: &str, reason: &str,
    ) -> Result<Vec<u8>, ModerationError>;
}
