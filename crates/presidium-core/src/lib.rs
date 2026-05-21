//! # Presidium Core
//!
//! Core domain layer for the Presidium Messenger.
//!
//! This crate contains fundamental domain entities, value objects,
//! application configuration, observability setup, and port definitions
//! shared across all other Presidium crates.
//!
//! ## Architecture
//!
//! This crate follows **Hexagonal Architecture** (Ports & Adapters):
//! - [`domain`] — Pure business entities, value objects, domain errors.
//!   No external dependencies.
//! - [`application`] — Use cases (interactors) and port trait definitions.
//!   Depends only on the domain layer.
//! - [`infrastructure`] — Concrete adapter implementations.
//!   Depends on application ports.
//!
//! ## Design Principles
//!
//! - **Domain-first**: Business rules live here, infrastructure is pluggable.
//! - **No unsafe code**: This crate forbids `unsafe` blocks entirely.
//! - **Async by default**: All I/O operations are asynchronous via `tokio`.
//! - **Observable**: All operations are instrumented with `tracing`.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod application;
pub mod config;
pub mod domain;
pub mod infrastructure;
pub mod observability;
