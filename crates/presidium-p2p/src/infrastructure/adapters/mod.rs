//! # P2P Adapters
//!
//! Stub implementations of P2P ports for development.

use crate::application::ports::{DhtPort, PeerDiscoveryPort};
use crate::domain::peer::{Peer, PeerState};
use async_trait::async_trait;
use presidium_core::domain::entities::UserId;
use presidium_core::domain::errors::DomainError;
use presidium_core::domain::value_objects::Multiaddr;
use std::collections::HashMap;
use tokio::sync::RwLock;

/// Stub P2P network adapter for development and testing.
///
/// All network operations are simulated in-memory.
pub struct StubP2pAdapter {
    /// Known peers registry.
    peers: RwLock<HashMap<String, Peer>>,
    /// Whether the adapter is currently "listening".
    listening: RwLock<bool>,
}

impl StubP2pAdapter {
    /// Creates a new stub P2P adapter.
    #[must_use]
    pub fn new() -> Self {
        Self {
            peers: RwLock::new(HashMap::new()),
            listening: RwLock::new(false),
        }
    }
}

impl Default for StubP2pAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl PeerDiscoveryPort for StubP2pAdapter {
    async fn start_listening(&self, _addr: &str) -> Result<(), DomainError> {
        *self.listening.write().await = true;
        tracing::info!(target: "presidium::p2p", "stub: started listening");
        Ok(())
    }

    async fn stop_listening(&self) -> Result<(), DomainError> {
        *self.listening.write().await = false;
        tracing::info!(target: "presidium::p2p", "stub: stopped listening");
        Ok(())
    }

    async fn list_peers(&self) -> Result<Vec<Peer>, DomainError> {
        let peers = self.peers.read().await;
        Ok(peers.values().cloned().collect())
    }

    async fn connect_to_peer(&self, user_id: &UserId) -> Result<PeerState, DomainError> {
        let mut peers = self.peers.write().await;
        let key = user_id.to_hex();
        let peer = peers.entry(key).or_insert_with(|| Peer {
            user_id: user_id.clone(),
            addresses: vec![],
            state: PeerState::Disconnected,
            is_relay: false,
            last_seen_ms: 0,
        });
        peer.set_state(PeerState::Connected);
        drop(peers);
        tracing::debug!(target: "presidium::p2p", "stub: connected to {}", user_id);
        Ok(PeerState::Connected)
    }

    async fn disconnect_from_peer(&self, user_id: &UserId) -> Result<(), DomainError> {
        if let Some(peer) = self.peers.write().await.get_mut(&user_id.to_hex()) {
            peer.set_state(PeerState::Disconnected);
        }
        Ok(())
    }
}

#[async_trait]
impl DhtPort for StubP2pAdapter {
    async fn bootstrap(&self, _addrs: &[Multiaddr]) -> Result<(), DomainError> {
        tracing::info!(target: "presidium::p2p", "stub: DHT bootstrap");
        Ok(())
    }

    async fn lookup_peer(&self, _user_id: &UserId) -> Result<Vec<Multiaddr>, DomainError> {
        Ok(vec![])
    }

    async fn provide_address(&self, _addr: &Multiaddr) -> Result<(), DomainError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_stub_p2p_lifecycle() {
        let adapter = StubP2pAdapter::new();
        adapter
            .start_listening("/ip4/0.0.0.0/tcp/0")
            .await
            .expect("listen");

        let user = UserId::new([1u8; 32]);
        let state = adapter.connect_to_peer(&user).await.expect("connect");
        assert_eq!(state, PeerState::Connected);

        let peers = adapter.list_peers().await.expect("list");
        assert_eq!(peers.len(), 1);

        adapter
            .disconnect_from_peer(&user)
            .await
            .expect("disconnect");
        adapter.stop_listening().await.expect("stop");
    }

    #[tokio::test]
    async fn test_stub_dht_bootstrap() {
        let adapter = StubP2pAdapter::new();
        let addr = Multiaddr::new("/ip4/1.2.3.4/tcp/4001").expect("addr");
        adapter.bootstrap(&[addr]).await.expect("bootstrap");
    }
}
