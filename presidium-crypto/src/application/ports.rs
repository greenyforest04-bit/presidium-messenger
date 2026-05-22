use async_trait::async_trait;
use presidium_core::domain::{DeviceId, PreKeyBundle, SessionId, UserId};

#[derive(Debug, thiserror::Error)]
pub enum CryptoError {
    #[error("Key generation failed: {0}")]
    KeyGeneration(String),
    #[error("Session establishment failed: {0}")]
    SessionEstablishment(String),
    #[error("Encryption failed: {0}")]
    Encryption(String),
    #[error("Decryption failed: {0}")]
    Decryption(String),
    #[error("Ratchet rotation failed: {0}")]
    RatchetRotation(String),
}

#[async_trait]
pub trait E2EECryptoPort: Send + Sync {
    async fn generate_pre_key_bundle(
        &self, device_id: &DeviceId,
    ) -> Result<PreKeyBundle, CryptoError>;
    async fn establish_session(
        &self, remote_user: &UserId, remote_device: &DeviceId, bundle: &PreKeyBundle,
    ) -> Result<SessionId, CryptoError>;
    async fn encrypt_message(
        &self, session_id: &SessionId, plaintext: &[u8],
    ) -> Result<Vec<u8>, CryptoError>;
    async fn decrypt_message(
        &self, session_id: &SessionId, ciphertext: &[u8],
    ) -> Result<Vec<u8>, CryptoError>;
    async fn rotate_ratchet(&self, session_id: &SessionId) -> Result<(), CryptoError>;
}
