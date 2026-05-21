//! # Presidium Storage
//!
//! Persistent storage subsystem for Presidium Messenger.
//!
//! Provides encrypted, reliable storage for:
//! - Messages and conversation history
//! - Session state and key material
//! - User profiles and contacts
//! - Domain events for audit/event sourcing
//!
//! ## Architecture
//!
//! Implements [`presidium_core::application::ports::StoragePort`]
//! with concrete backends (`SQLite` planned for MVP).
//!
//! ## Security
//!
//! - All data at rest is encrypted (AES-GCM).
//! - Key material is stored separately from message data.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod application;
pub mod domain;
pub mod infrastructure;
