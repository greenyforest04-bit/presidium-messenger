//! # Crypto Infrastructure Layer
//!
//! Concrete implementations of crypto operations.
//!
//! Will contain:
//! - `LibSignalCryptoAdapter` — wraps libsignal-protocol-rust
//! - `KyberAdapter` — post-quantum key encapsulation
//! - `DoubleRatchetSession` — ratchet state machine

pub mod adapters;
