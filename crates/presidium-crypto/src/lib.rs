//! # Presidium Crypto
//!
//! End-to-end encryption subsystem for Presidium Messenger.
//!
//! This crate implements the cryptographic layer responsible for:
//! - Key generation (X25519, Ed25519)
//! - Pre-key bundle creation and management
//! - Session establishment via the PQXDH protocol
//! - Message encryption/decryption using Double Ratchet
//!
//! ## Architecture
//!
//! Follows Hexagonal Architecture with ports defined in
//! [`presidium_core::application::ports::CryptoPort`] and
//! concrete implementations provided here.
//!
//! ## Security Notes
//!
//! - All key material is zeroized on drop where possible.
//! - No secrets are ever logged or included in error messages.
//! - Random number generation uses `OsRng` (CSPRNG).

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod application;
pub mod domain;
pub mod infrastructure;
