//! # Application Layer
//!
//! Use cases (interactors) and port definitions.
//!
//! The application layer orchestrates domain logic through use cases
//! and defines ports (trait interfaces) that infrastructure adapters
//! must implement. This layer depends only on the domain layer.

pub mod ports;
