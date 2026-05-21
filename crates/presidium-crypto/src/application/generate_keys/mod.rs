//! # Key Generation Use Case
//!
//! Interactor for generating new cryptographic key pairs.

use crate::domain::keypair::IdentityKeyPair;
use presidium_core::domain::errors::DomainError;
use rand::RngCore;

/// Generates a new Ed25519 identity key pair using the system CSPRNG.
///
/// # Errors
///
/// Returns a [`DomainError::OperationFailed`] if key generation fails,
/// which should never happen with a properly functioning OS.
pub async fn generate_identity_keypair() -> Result<IdentityKeyPair, DomainError> {
    // In a real implementation, this would use ed25519_dalek::SigningKey::generate()
    // and extract the public/private bytes. For the Day 1 skeleton, we use rand.
    let mut public_key = [0u8; 32];
    let mut private_key = [0u8; 32];

    let mut rng = rand::rngs::OsRng;
    rng.fill_bytes(&mut public_key);
    rng.fill_bytes(&mut private_key);

    tracing::debug!(target: "presidium::crypto", "generated new identity key pair");

    Ok(IdentityKeyPair {
        public_key,
        private_key,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_generate_identity_keypair() {
        let kp1 = generate_identity_keypair().await.expect("generate kp1");
        let kp2 = generate_identity_keypair().await.expect("generate kp2");

        // Two generated keypairs should have different keys
        assert_ne!(kp1.public_key, kp2.public_key);
        assert_ne!(kp1.private_key, kp2.private_key);
    }
}
