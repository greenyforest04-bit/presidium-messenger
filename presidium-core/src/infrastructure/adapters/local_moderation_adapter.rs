use crate::application::ports::{ContentVerdict, ModerationError, ModerationPort};
use crate::domain::UserId;
use async_trait::async_trait;

/// Stub adapter — will be replaced by local on-device moderation
/// TODO(Day 9+): Implement real content moderation with on-device LLM + keyword filtering
pub struct LocalModerationAdapter;

impl Default for LocalModerationAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl LocalModerationAdapter {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl ModerationPort for LocalModerationAdapter {
    async fn check_message(
        &self, _sender: &UserId, _plaintext: &str,
    ) -> Result<ContentVerdict, ModerationError> {
        unimplemented!("TODO: Implement local content moderation with keyword filter + LLM")
    }

    async fn create_sarcophagus(
        &self, _offender: &UserId, _evidence: &str, _reason: &str,
    ) -> Result<Vec<u8>, ModerationError> {
        unimplemented!(
            "TODO: Implement sarcophagus creation (encrypted package for law enforcement)"
        )
    }
}
