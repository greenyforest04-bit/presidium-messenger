//! # Key Pair Domain Types
//!
//! Represents cryptographic key pairs for identity and exchange.

use serde::{Deserialize, Serialize};

/// An Ed25519 identity key pair for user identification.
///
/// The public key serves as the user's [`presidium_core::domain::entities::UserId`].
/// The private key must never leave the device's secure enclave.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityKeyPair {
    /// The public identity key (32 bytes).
    pub public_key: [u8; 32],
    /// The private identity key (32 bytes). Handle with extreme care.
    #[serde(skip_serializing)]
    pub private_key: [u8; 32],
}

/// An X25519 key exchange pair for Diffie-Hellman operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyExchangePair {
    /// The public exchange key (32 bytes).
    pub public_key: [u8; 32],
    /// The private exchange key (32 bytes).
    #[serde(skip_serializing)]
    pub private_key: [u8; 32],
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity_key_pair_fields() {
        let kp = IdentityKeyPair {
            public_key: [1u8; 32],
            private_key: [2u8; 32],
        };
        assert_eq!(kp.public_key.len(), 32);
    }

    #[test]
    fn test_key_exchange_pair_fields() {
        let kp = KeyExchangePair {
            public_key: [3u8; 32],
            private_key: [4u8; 32],
        };
        assert_eq!(kp.private_key.len(), 32);
    }

    #[test]
    fn test_identity_key_pair_serialize_skips_private() {
        let kp = IdentityKeyPair {
            public_key: [1u8; 32],
            private_key: [2u8; 32],
        };
        let json = serde_json::to_string(&kp).expect("serialize");
        assert!(!json.contains("private_key"));
    }
}
