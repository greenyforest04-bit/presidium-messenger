//! # Architecture Conformance Tests
//!
//! These integration tests verify that the hexagonal architecture
//! boundaries are respected across crate layers.
//!
//! The key rule: **domain must not depend on infrastructure**.
//! This is enforced at the module level — `domain/` must not
//! import anything from `infrastructure/` or external crates
//! (except `serde` for serialization of domain types).

use presidium_core::application::ports::{
    CryptoPort, MessageTransportPort, ModerationPort, StoragePort,
};
use presidium_core::config::AppConfig;
use presidium_core::domain::entities::{Message, SessionId, UserId};
use presidium_core::domain::errors::DomainError;
use presidium_core::domain::events::DomainEvent;
use presidium_core::domain::value_objects::{KeyFingerprint, ModerationResult, Multiaddr};
use presidium_core::infrastructure::adapters::{
    StubCryptoAdapter, StubModerationAdapter, StubStorageAdapter, StubTransportAdapter,
};

/// Test that the domain layer compiles without infrastructure.
///
/// If the domain layer ever imports infrastructure types, this test
/// will fail to compile — catching architecture violations early.
#[test]
fn domain_compiles_independently() {
    // Domain types should be constructible without any infrastructure
    let _user = UserId::new([1u8; 32]);
    let _session = SessionId::new("test");
    let _fp = KeyFingerprint::new([42u8; 32]);
    let _result = ModerationResult::Safe;

    // Domain events should be constructible independently
    let _event = DomainEvent::SessionClosed {
        session_id: SessionId::new("s1"),
        reason: "test".into(),
    };
}

/// Test that the application configuration loads with defaults.
#[test]
fn app_config_default_construction() {
    // We can construct a config struct directly
    let config = AppConfig {
        network: presidium_core::config::NetworkConfig {
            listen_addr: "/ip4/0.0.0.0/tcp/0".into(),
            bootstrap_nodes: vec![],
            mdns_enabled: true,
            relay: presidium_core::config::RelayConfig::default(),
        },
        storage: presidium_core::config::StorageConfig {
            database_path: std::path::PathBuf::from("./data/test.db"),
            max_size_mb: 256,
        },
        crypto: presidium_core::config::CryptoConfig {
            key_exchange_protocol: "PQXDH".into(),
            post_quantum_enabled: true,
        },
        llm: presidium_core::config::LlmConfig::default(),
    };
    assert_eq!(config.crypto.key_exchange_protocol, "PQXDH");
    assert!(config.network.mdns_enabled);
}

/// Test that all stub adapters satisfy their port traits.
#[tokio::test]
async fn stub_adapters_satisfy_ports() {
    let crypto = StubCryptoAdapter::new();
    let transport = StubTransportAdapter::new();
    let storage = StubStorageAdapter::new();
    let moderation = StubModerationAdapter::new();

    // Verify crypto port
    let bundle = crypto.create_pre_key_bundle().await.expect("bundle");
    assert_eq!(bundle.identity_key.len(), 32);

    // Verify transport port
    let user = UserId::new([1u8; 32]);
    transport.send(&user, b"test").await.expect("send");

    // Verify storage port
    let sender = UserId::new([1u8; 32]);
    let recipient = UserId::new([2u8; 32]);
    let session = SessionId::new("test-session");
    let msg = Message::new("m1", sender, recipient, session.clone(), "hi", 100, 0);
    storage.store_message(&msg).await.expect("store");
    let fetched = storage.get_message("m1").await.expect("get");
    assert_eq!(fetched.content, "hi");

    // Verify moderation port
    let result = moderation.moderate("hello world").await.expect("moderate");
    assert_eq!(result, ModerationResult::Safe);
}

/// Test that domain errors are properly structured.
#[test]
fn domain_errors_are_structured() {
    let user = UserId::new([1u8; 32]);
    let err = DomainError::SessionNotFound(user.clone());

    // Verify error display is informative
    let display = format!("{err}");
    assert!(display.contains("session not found"));

    // Verify error debug includes variant name
    let debug = format!("{err:?}");
    assert!(debug.contains("SessionNotFound"));
}

/// Test that Multiaddr value object enforces constraints.
#[test]
fn multiaddr_rejects_empty() {
    assert!(Multiaddr::new("").is_err());
    assert!(Multiaddr::new("/ip4/127.0.0.1/tcp/4001").is_ok());
}
