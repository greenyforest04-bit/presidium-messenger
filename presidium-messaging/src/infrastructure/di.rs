use crate::application::use_cases::{SendMessageInteractor, SendMessageUseCase};
use presidium_core::infrastructure::adapters::local_moderation_adapter::LocalModerationAdapter;
use presidium_crypto::infrastructure::adapters::libsignal_crypto_adapter::LibSignalCryptoAdapter;
use presidium_p2p::infrastructure::adapters::libp2p_network_adapter::Libp2pNetworkAdapter;
use presidium_storage::infrastructure::adapters::redb_storage_adapter::RedbStorageAdapter;

/// Dependency injection factory for use cases.
/// Wires up concrete adapter implementations to use case interactors.
/// TODO(Day 10+): Replace with a proper DI container or runtime builder
pub fn build_send_message_use_case() -> impl SendMessageUseCase {
    let crypto = LibSignalCryptoAdapter::new();
    let p2p = Libp2pNetworkAdapter::new();
    let storage = RedbStorageAdapter::new();
    let moderation = LocalModerationAdapter::new();
    SendMessageInteractor::new(crypto, p2p, storage, moderation)
}
