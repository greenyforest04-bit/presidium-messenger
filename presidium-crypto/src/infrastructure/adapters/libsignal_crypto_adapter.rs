use crate::application::ports::{CryptoError, E2EECryptoPort};
use async_trait::async_trait;
use presidium_core::domain::{DeviceId, PreKeyBundle, SessionId, UserId};

/// Stub adapter — will be replaced by libsignal-protocol-rust integration
/// TODO(Day 7+): Implement real PQXDH + Double Ratchet
pub struct LibSignalCryptoAdapter;

impl Default for LibSignalCryptoAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl LibSignalCryptoAdapter {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl E2EECryptoPort for LibSignalCryptoAdapter {
    async fn generate_pre_key_bundle(
        &self, _device_id: &DeviceId,
    ) -> Result<PreKeyBundle, CryptoError> {
        unimplemented!("TODO: Implement real pre-key bundle generation with libsignal")
    }

    async fn establish_session(
        &self, _remote_user: &UserId, _remote_device: &DeviceId, _bundle: &PreKeyBundle,
    ) -> Result<SessionId, CryptoError> {
        unimplemented!("TODO: Implement real PQXDH session establishment")
    }

    async fn encrypt_message(
        &self, _session_id: &SessionId, _plaintext: &[u8],
    ) -> Result<Vec<u8>, CryptoError> {
        unimplemented!("TODO: Implement real Double Ratchet encryption")
    }

    async fn decrypt_message(
        &self, _session_id: &SessionId, _ciphertext: &[u8],
    ) -> Result<Vec<u8>, CryptoError> {
        unimplemented!("TODO: Implement real Double Ratchet decryption")
    }

    async fn rotate_ratchet(&self, _session_id: &SessionId) -> Result<(), CryptoError> {
        unimplemented!("TODO: Implement real ratchet rotation")
    }
}
