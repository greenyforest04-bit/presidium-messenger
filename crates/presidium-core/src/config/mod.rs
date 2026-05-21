//! # Application Configuration
//!
//! Centralized configuration management for Presidium Messenger.
//!
//! Configuration is loaded from `presidium.toml` or environment variables
//! (prefixed with `PRESIDIUM_`) using the [`figment`] crate. The config
//! is parsed once at startup and made available via [`AppConfig`].

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Root application configuration for Presidium Messenger.
///
/// This struct is deserialized from `presidium.toml` or environment
/// variables. See [`AppConfig::load`] for details on precedence.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// Network configuration (P2P, relay, discovery).
    pub network: NetworkConfig,
    /// Storage configuration (database, cache).
    pub storage: StorageConfig,
    /// Cryptography configuration (key algorithms, parameters).
    pub crypto: CryptoConfig,
    /// LLM configuration (model, quantization, hardware).
    #[serde(default)]
    pub llm: LlmConfig,
}

/// Network-related configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// Listen address for incoming P2P connections.
    #[serde(default = "default_listen_addr")]
    pub listen_addr: String,
    /// Bootstrap nodes for initial DHT discovery.
    #[serde(default)]
    pub bootstrap_nodes: Vec<String>,
    /// Enable mDNS local peer discovery.
    #[serde(default = "default_true")]
    pub mdns_enabled: bool,
    /// Relay configuration for NAT traversal.
    #[serde(default)]
    pub relay: RelayConfig,
}

/// Relay configuration for NAT traversal.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelayConfig {
    /// Enable circuit relay v2.
    #[serde(default = "default_true")]
    pub enabled: bool,
    /// Maximum number of relay connections.
    #[serde(default = "default_max_relay_connections")]
    pub max_connections: usize,
}

/// Storage configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    /// Path to the database directory.
    #[serde(default = "default_db_path")]
    pub database_path: PathBuf,
    /// Maximum database size in megabytes.
    #[serde(default = "default_max_db_size_mb")]
    pub max_size_mb: usize,
}

/// Cryptography configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoConfig {
    /// Default key exchange protocol.
    #[serde(default = "default_kex_protocol")]
    pub key_exchange_protocol: String,
    /// Enable post-quantum key agreement (Kyber).
    #[serde(default = "default_true")]
    pub post_quantum_enabled: bool,
}

/// LLM (on-device model) configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmConfig {
    /// Path to the GGUF model file.
    pub model_path: Option<PathBuf>,
    /// Quantization level (4-bit, 8-bit).
    #[serde(default = "default_quantization")]
    pub quantization: String,
    /// Enable GPU/NPU acceleration.
    #[serde(default = "default_true")]
    pub hardware_acceleration: bool,
}

impl Default for RelayConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_connections: default_max_relay_connections(),
        }
    }
}

impl Default for LlmConfig {
    fn default() -> Self {
        Self {
            model_path: None,
            quantization: default_quantization(),
            hardware_acceleration: true,
        }
    }
}

impl AppConfig {
    /// Loads application configuration from the default sources.
    ///
    /// Configuration is resolved in the following priority order:
    /// 1. Environment variables (prefixed with `PRESIDIUM_`)
    /// 2. `presidium.toml` file in the current directory
    /// 3. Built-in defaults
    ///
    /// # Errors
    ///
    /// Returns an error if configuration parsing fails or required
    /// fields are missing from all sources.
    pub fn load() -> Result<Self, config::ConfigError> {
        Self::load_from(PathBuf::from("presidium.toml"))
    }

    /// Loads configuration from a specific file path.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read or parsed.
    pub fn load_from(path: PathBuf) -> Result<Self, config::ConfigError> {
        use figment::{Figment, providers::Format, providers::Toml, providers::Env};

        Figment::new()
            .merge(Toml::file(path))
            .merge(Env::prefixed("PRESIDIUM_").split("__"))
            .extract()
    }
}

mod config {
    pub use figment::Error as ConfigError;
}

// Default value functions for serde defaults
fn default_listen_addr() -> String {
    "/ip4/0.0.0.0/tcp/0".to_string()
}

fn default_true() -> bool {
    true
}

fn default_max_relay_connections() -> usize {
    16
}

fn default_db_path() -> PathBuf {
    PathBuf::from("./data/presidium.db")
}

fn default_max_db_size_mb() -> usize {
    512
}

fn default_kex_protocol() -> String {
    "PQXDH".to_string()
}

fn default_quantization() -> String {
    "q4".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_app_config() {
        let config = AppConfig {
            network: NetworkConfig {
                listen_addr: default_listen_addr(),
                bootstrap_nodes: vec![],
                mdns_enabled: true,
                relay: RelayConfig::default(),
            },
            storage: StorageConfig {
                database_path: default_db_path(),
                max_size_mb: default_max_db_size_mb(),
            },
            crypto: CryptoConfig {
                key_exchange_protocol: default_kex_protocol(),
                post_quantum_enabled: true,
            },
            llm: LlmConfig::default(),
        };
        assert_eq!(config.network.listen_addr, "/ip4/0.0.0.0/tcp/0");
        assert_eq!(config.crypto.key_exchange_protocol, "PQXDH");
        assert!(config.crypto.post_quantum_enabled);
        assert_eq!(config.llm.quantization, "q4");
    }

    #[test]
    fn test_config_serialization_roundtrip() {
        let config = AppConfig {
            network: NetworkConfig {
                listen_addr: "/ip4/127.0.0.1/tcp/9090".into(),
                bootstrap_nodes: vec!["/ip4/1.2.3.4/tcp/4001".into()],
                mdns_enabled: false,
                relay: RelayConfig {
                    enabled: true,
                    max_connections: 32,
                },
            },
            storage: StorageConfig {
                database_path: PathBuf::from("/tmp/test.db"),
                max_size_mb: 1024,
            },
            crypto: CryptoConfig {
                key_exchange_protocol: "X3DH".into(),
                post_quantum_enabled: false,
            },
            llm: LlmConfig {
                model_path: Some(PathBuf::from("/models/gemma-2b.gguf")),
                quantization: "q8".into(),
                hardware_acceleration: false,
            },
        };
        let toml_str = toml::to_string(&config).expect("serialize");
        let deserialized: AppConfig = toml::from_str(&toml_str).expect("deserialize");
        assert_eq!(deserialized.network.listen_addr, "/ip4/127.0.0.1/tcp/9090");
        assert_eq!(deserialized.llm.quantization, "q8");
        assert_eq!(deserialized.storage.max_size_mb, 1024);
    }
}
