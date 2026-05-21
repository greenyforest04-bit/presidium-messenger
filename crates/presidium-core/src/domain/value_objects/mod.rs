//! # Value Objects
//!
//! Immutable value types that model domain concepts without identity.
//!
//! Value objects are compared by their contents, not by reference.
//! They are used throughout the domain to ensure type safety and
//! prevent primitive obsession.

use serde::{Deserialize, Serialize};

/// A strongly-typed wrapper around a network address (multiaddr).
///
/// Uses string representation to avoid coupling to a specific
/// multiaddr library at the domain level.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Multiaddr(String);

impl Multiaddr {
    /// Creates a new [`Multiaddr`] from a string representation.
    ///
    /// # Errors
    ///
    /// Returns an error if the address string is empty.
    pub fn new(addr: impl Into<String>) -> Result<Self, &'static str> {
        let s = addr.into();
        if s.is_empty() {
            return Err("multiaddr must not be empty");
        }
        Ok(Self(s))
    }

    /// Returns the underlying string representation.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for Multiaddr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A cryptographic key fingerprint for identification purposes.
///
/// Stores the raw bytes of a public key fingerprint.
/// Displayed as hexadecimal for human-readable contexts.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct KeyFingerprint(pub [u8; 32]);

impl KeyFingerprint {
    /// Creates a new fingerprint from raw bytes.
    #[must_use]
    pub const fn new(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// Returns a hexadecimal representation.
    #[must_use]
    pub fn to_hex(&self) -> String {
        self.0.iter().map(|b| format!("{b:02x}")).collect()
    }
}

/// Content moderation result produced by the local LLM.
///
/// This value object is used by the moderation pipeline to
/// communicate decisions about user-generated content.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ModerationResult {
    /// Content is safe to deliver.
    Safe,
    /// Content is flagged but not blocked (warning to user).
    Flagged {
        /// Reason for flagging.
        reason: String,
    },
    /// Content is blocked and a sarcophagus is created.
    Blocked {
        /// Category of violation (e.g., "extremism", "csam").
        category: String,
        /// Detailed reason for blocking.
        reason: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiaddr_valid() {
        let addr = Multiaddr::new("/ip4/127.0.0.1/tcp/4001").expect("valid addr");
        assert_eq!(addr.as_str(), "/ip4/127.0.0.1/tcp/4001");
    }

    #[test]
    fn test_multiaddr_empty_rejects() {
        assert!(Multiaddr::new("").is_err());
    }

    #[test]
    fn test_key_fingerprint_hex() {
        let fp = KeyFingerprint::new([0xFF; 32]);
        assert_eq!(fp.to_hex().len(), 64);
    }

    #[test]
    fn test_moderation_result_variants() {
        let safe = ModerationResult::Safe;
        let flagged = ModerationResult::Flagged {
            reason: "inappropriate language".into(),
        };
        let blocked = ModerationResult::Blocked {
            category: "extremism".into(),
            reason: "violates policy".into(),
        };
        assert_eq!(safe, ModerationResult::Safe);
        assert_ne!(safe, flagged);
        assert_ne!(flagged, blocked);
    }
}
