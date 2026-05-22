use async_trait::async_trait;
use presidium_core::domain::{DeviceId, UserId};

#[derive(Debug, thiserror::Error)]
pub enum NetworkError {
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),
    #[error("Send failed: {0}")]
    SendFailed(String),
    #[error("Receive failed: {0}")]
    ReceiveFailed(String),
    #[error("Publish failed: {0}")]
    PublishFailed(String),
    #[error("Subscription failed: {0}")]
    SubscriptionFailed(String),
}

#[async_trait]
pub trait P2PPort: Send + Sync {
    async fn send_p2p(&self, target_device: &DeviceId, data: Vec<u8>) -> Result<(), NetworkError>;
    async fn receive_p2p(&self) -> Result<Option<Vec<u8>>, NetworkError>;
    async fn publish_pre_keys(
        &self, user_id: &UserId, device_id: &DeviceId, bundle: &[u8],
    ) -> Result<(), NetworkError>;
    async fn fetch_pre_keys(
        &self, user_id: &UserId, device_id: &DeviceId,
    ) -> Result<Vec<u8>, NetworkError>;
    async fn subscribe_topic(&self, topic: &str) -> Result<(), NetworkError>;
}
