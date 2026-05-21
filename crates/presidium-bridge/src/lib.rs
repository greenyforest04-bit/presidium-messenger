//! # Presidium Bridge
//!
//! Foreign Function Interface (FFI) bridge for platform interop.
//!
//! This crate provides bindings for:
//! - **Android**: Kotlin Multiplatform via UniFFI
//! - **iOS**: Swift via UniFFI
//! - **Desktop**: C ABI / WASM
//!
//! ## Status
//!
//! Day 1 placeholder — UniFFI bindings will be generated once
//! the core domain API stabilizes (after Day 7–10).
//!
//! ## Architecture
//!
//! This crate re-exports the public API from all other crates
//! and provides thin wrapper functions callable from foreign code.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

/// Placeholder — UniFFI bindings will be generated here.
///
/// This module will contain the generated bindings for:
/// - Android (Kotlin)
/// - iOS (Swift)
/// - Desktop (C ABI)
pub mod generated {
    //! Generated UniFFI bindings (placeholder for Day 1).
    //!
    //! In subsequent development days, this will be populated by
    //! the `uniffi-bindgen` tool from the `presidium-core` API surface.

    /// Information about the Presidium runtime version.
    #[must_use]
    pub fn runtime_version() -> &'static str {
        env!("CARGO_PKG_VERSION")
    }
}

/// Initializes the Presidium runtime from a mobile platform.
///
/// This function must be called once from the platform entry point
/// (e.g., Android `Application.onCreate()` or iOS `AppDelegate.init()`).
///
/// # Errors
///
/// Returns a string description if initialization fails.
pub fn initialize_runtime() -> Result<String, String> {
    tracing::info!(target: "presidium::bridge", "initializing runtime from bridge");
    Ok(format!("Presidium v{}", env!("CARGO_PKG_VERSION")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runtime_version() {
        let version = generated::runtime_version();
        assert!(!version.is_empty());
    }

    #[test]
    fn test_initialize_runtime() {
        let result = initialize_runtime();
        assert!(result.is_ok());
        let msg = result.unwrap();
        assert!(msg.contains("Presidium"));
    }
}
