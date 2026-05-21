//! # Presidium Messaging
//!
//! Messaging domain layer for Presidium Messenger.
//!
//! This crate orchestrates the full message lifecycle:
//! 1. User composes a message
//! 2. Content is moderated by the local LLM
//! 3. Message is encrypted via the crypto subsystem
//! 4. Encrypted payload is transported via the P2P subsystem
//! 5. Message is stored locally for conversation history
//!
//! ## Architecture
//!
//! This crate is the "application services" layer — it coordinates
//! between core ports (crypto, transport, storage, moderation)
//! but does not contain infrastructure implementations.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod application;
pub mod domain;
