//! # Pre-Key Domain Types
//!
//! Pre-keys used in the X3DH/PQXDH key agreement protocol.

use presidium_core::application::ports::PreKeyBundle;
use serde::{Deserialize, Serialize};

/// A signed pre-key with its signature.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedPreKey {
    /// The signed pre-key identifier.
    pub id: u32,
    /// The public key of the signed pre-key (32 bytes).
    pub public_key: [u8; 32],
    /// Signature over the public key by the identity key.
    pub signature: Vec<u8>,
}

/// A one-time pre-key for forward secrecy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OneTimePreKey {
    /// The one-time pre-key identifier.
    pub id: u32,
    /// The public key of the one-time pre-key (32 bytes).
    pub public_key: [u8; 32],
}

/// Converts domain pre-key types to the core [`PreKeyBundle`] for port compatibility.
#[must_use]
pub fn to_pre_key_bundle(
    identity_key: &[u8],
    signed: &SignedPreKey,
    one_time: Option<&OneTimePreKey>,
) -> PreKeyBundle {
    PreKeyBundle {
        identity_key: identity_key.to_vec(),
        signed_pre_key: signed.public_key.to_vec(),
        signed_pre_key_signature: signed.signature.clone(),
        one_time_pre_key: one_time.map(|ot| ot.public_key.to_vec()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signed_pre_key_creation() {
        let spk = SignedPreKey {
            id: 1,
            public_key: [42u8; 32],
            signature: vec![1, 2, 3],
        };
        assert_eq!(spk.id, 1);
        assert_eq!(spk.signature.len(), 3);
    }

    #[test]
    fn test_to_pre_key_bundle() {
        let identity = [1u8; 32];
        let signed = SignedPreKey {
            id: 1,
            public_key: [2u8; 32],
            signature: vec![3; 64],
        };
        let one_time = OneTimePreKey {
            id: 100,
            public_key: [4u8; 32],
        };

        let bundle = to_pre_key_bundle(&identity, &signed, Some(&one_time));
        assert_eq!(bundle.identity_key.len(), 32);
        assert!(bundle.one_time_pre_key.is_some());
    }
}
