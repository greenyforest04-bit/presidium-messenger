//! # Crypto Adapters
//!
//! Placeholder adapter that implements [`presidium_core::application::ports::CryptoPort`]
//! using domain key types.

use crate::application::generate_keys::generate_identity_keypair;
use crate::domain::keypair::IdentityKeyPair;
use crate::domain::pre_key::{to_pre_key_bundle, SignedPreKey};
use async_trait::async_trait;
use presidium_core::application::ports::{CryptoPort, PreKeyBundle};
use presidium_core::domain::entities::{SessionId, UserId};
use presidium_core::domain::errors::DomainError;
use std::collections::HashMap;
use tokio::sync::RwLock;

/// A crypto adapter that generates real random keys but does not
/// perform actual encryption (Day 1 stub — real Signal integration TBD).
pub struct DevCryptoAdapter {
    /// Identity key pair (generated on first use).
    identity: RwLock<Option<IdentityKeyPair>>,
    /// Active sessions.
    sessions: RwLock<HashMap<String, SessionId>>,
}

impl DevCryptoAdapter {
    /// Creates a new development crypto adapter.
    #[must_use]
    pub fn new() -> Self {
        Self {
            identity: RwLock::new(None),
            sessions: RwLock::new(HashMap::new()),
        }
    }
}

impl Default for DevCryptoAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl CryptoPort for DevCryptoAdapter {
    async fn create_pre_key_bundle(&self) -> Result<PreKeyBundle, DomainError> {
        let kp = generate_identity_keypair().await?;

        // Store identity if not already stored
        {
            let mut identity = self.identity.write().await;
            if identity.is_none() {
                *identity = Some(kp.clone());
            }
        }

        let signed = SignedPreKey {
            id: 1,
            public_key: kp.public_key,
            signature: vec![0u8; 64], // Placeholder signature
        };

        Ok(to_pre_key_bundle(&kp.public_key, &signed, None))
    }

    async fn establish_session(
        &self,
        remote_user: &UserId,
        _bundle: &PreKeyBundle,
    ) -> Result<SessionId, DomainError> {
        let session_id = SessionId::new(format!("crypto-session-{}", remote_user.to_hex()));
        self.sessions
            .write()
            .await
            .insert(remote_user.to_hex(), session_id.clone());
        tracing::info!(
            target: "presidium::crypto",
            session_id = %session_id,
            "established crypto session"
        );
        Ok(session_id)
    }

    async fn encrypt_message(
        &self,
        _session_id: &SessionId,
        plaintext: &[u8],
    ) -> Result<Vec<u8>, DomainError> {
        // Stub: real Double Ratchet encryption TBD
        Ok(plaintext.to_vec())
    }

    async fn decrypt_message(
        &self,
        _session_id: &SessionId,
        ciphertext: &[u8],
    ) -> Result<Vec<u8>, DomainError> {
        // Stub: real Double Ratchet decryption TBD
        Ok(ciphertext.to_vec())
    }

    async fn close_session(&self, session_id: &SessionId) -> Result<(), DomainError> {
        self.sessions.write().await.retain(|_, v| v != session_id);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_dev_crypto_adapter_lifecycle() {
        let adapter = DevCryptoAdapter::new();
        let user = UserId::new([7u8; 32]);
        let bundle = adapter
            .create_pre_key_bundle()
            .await
            .expect("create bundle");
        assert_eq!(bundle.identity_key.len(), 32);

        let session = adapter
            .establish_session(&user, &bundle)
            .await
            .expect("session");
        let encrypted = adapter
            .encrypt_message(&session, b"hello")
            .await
            .expect("encrypt");
        let decrypted = adapter
            .decrypt_message(&session, &encrypted)
            .await
            .expect("decrypt");
        assert_eq!(decrypted, b"hello");

        adapter.close_session(&session).await.expect("close");
    }
}
