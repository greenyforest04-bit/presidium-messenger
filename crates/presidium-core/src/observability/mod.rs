//! # Observability
//!
//! Tracing initialization and utilities for Presidium Messenger.
//!
//! This module provides a one-call setup for structured logging
//! using the `tracing` ecosystem. In development mode, logs are
//! formatted for human readability. In production, they are emitted
//! as JSON for ingestion by log aggregation systems.

use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

/// Initializes the global tracing subscriber.
///
/// Call this exactly once at the beginning of `main()` or the
/// platform-specific entry point (Android `on_create`, etc.).
///
/// # Behavior
///
/// - Reads the `RUST_LOG` environment variable to set log levels.
///   If unset, defaults to `presidium=debug,tower_http=debug`.
/// - In development builds, uses a pretty-printed formatter.
/// - When `PRESIDIUM_JSON_LOGS=true`, emits structured JSON logs.
///
/// # Example
///
/// ```no_run
/// use presidium_core::observability;
///
/// #[tokio::main]
/// async fn main() {
///     observability::init_tracing();
///     tracing::info!("Presidium Messenger starting");
/// }
/// ```
pub fn init_tracing() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("presidium=debug,tower_http=debug"));

    let use_json = std::env::var("PRESIDIUM_JSON_LOGS").is_ok_and(|v| v == "true" || v == "1");

    if use_json {
        tracing_subscriber::registry()
            .with(filter)
            .with(fmt::layer().json())
            .init();
    } else {
        tracing_subscriber::registry()
            .with(filter)
            .with(fmt::layer().pretty())
            .init();
    }

    tracing::info!(
        target: "presidium::init",
        version = env!("CARGO_PKG_VERSION"),
        "tracing initialized"
    );
}

/// Returns a default [`EnvFilter`] suitable for tests.
///
/// This is a convenience function for test modules that need
/// tracing output without full initialization.
#[must_use]
pub fn test_filter() -> EnvFilter {
    EnvFilter::new("presidium=trace")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test_filter_creation() {
        let filter = test_filter();
        // Ensure the filter was created successfully
        let _ = format!("{filter:?}");
    }
}
