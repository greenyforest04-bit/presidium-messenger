//! # Messaging Application Layer
//!
//! Use cases (interactors) for the messaging domain.
//!
//! Each use case orchestrates one or more ports to fulfill a
//! user-level action. Use cases are the only place where
//! domain ports are combined and called.

pub mod send_message;
