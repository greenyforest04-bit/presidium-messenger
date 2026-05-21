//! # Domain Layer
//!
//! Pure business entities, value objects, and domain errors.
//!
//! This module contains zero external dependencies — it models the core
//! business concepts of Presidium Messenger without any infrastructure
//! concerns (no database, no network, no crypto implementation details).

pub mod entities;
pub mod errors;
pub mod events;
pub mod value_objects;
