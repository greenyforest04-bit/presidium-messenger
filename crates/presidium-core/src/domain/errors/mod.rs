//! # Domain Errors
//!
//! Centralized error types for the Presidium domain layer.
//!
//! All domain errors are defined here using [`thiserror`] to provide
//! ergonomic error handling with meaningful context. Infrastructure
//! adapters map their own errors to these domain errors.

use crate::domain::entities::{SessionId, UserId};
use thiserror::Error;

/// Top-level error type for the Presidium core domain.
///
/// All domain operations return `Result<T, DomainError>`.
/// Each variant carries contextual information for logging
/// and user-facing error messages.
#[derive(Error, Debug)]
pub enum DomainError {
    /// A messaging session was not found for the given user.
    #[error("session not found for user {0}")]
    SessionNotFound(UserId),

    /// A specific session ID could not be resolved.
    #[error("session ID not found: {0}")]
    SessionIdNotFound(SessionId),

    /// The operation requires an existing session that is not yet established.
    #[error("session not established with user {0}")]
    SessionNotEstablished(UserId),

    /// A message could not be found by its ID.
    #[error("message not found: {0}")]
    MessageNotFound(String),

    /// Attempted to send a message with invalid parameters.
    #[error("invalid message: {0}")]
    InvalidMessage(String),

    /// Configuration error — the application was misconfigured.
    #[error("configuration error: {0}")]
    ConfigError(String),

    /// A required resource was not available (e.g., storage unavailable).
    #[error("resource unavailable: {0}")]
    ResourceUnavailable(String),

    /// Generic operation failure with context.
    #[error("operation failed: {0}")]
    OperationFailed(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let user = UserId::new([1u8; 32]);
        let err = DomainError::SessionNotFound(user.clone());
        assert!(err.to_string().contains("session not found"));
    }

    #[test]
    fn test_error_debug() {
        let err = DomainError::InvalidMessage("empty content".into());
        let debug_str = format!("{err:?}");
        assert!(debug_str.contains("InvalidMessage"));
    }
}
