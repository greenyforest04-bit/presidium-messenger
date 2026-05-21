//! # P2P Port Definitions
//!
//! Extended port traits specific to the P2P subsystem.

use crate::domain::peer::{Peer, PeerState};
use async_trait::async_trait;
use presidium_core::domain::entities::UserId;
use presidium_core::domain::errors::DomainError;
use presidium_core::domain::value_objects::Multiaddr;

/// Port for peer discovery and management.
#[async_trait]
pub trait PeerDiscoveryPort: Send + Sync {
    /// Starts listening for incoming connections on the given address.
    async fn start_listening(&self, addr: &str) -> Result<(), DomainError>;

    /// Stops listening and gracefully closes all connections.
    async fn stop_listening(&self) -> Result<(), DomainError>;

    /// Returns a list of all known peers.
    async fn list_peers(&self) -> Result<Vec<Peer>, DomainError>;

    /// Attempts to connect to a peer by their user ID.
    async fn connect_to_peer(&self, user_id: &UserId) -> Result<PeerState, DomainError>;

    /// Disconnects from a specific peer.
    async fn disconnect_from_peer(&self, user_id: &UserId) -> Result<(), DomainError>;
}

/// Port for DHT operations (Kademlia).
#[async_trait]
pub trait DhtPort: Send + Sync {
    /// Bootstraps the DHT with the given bootstrap nodes.
    async fn bootstrap(&self, bootstrap_addrs: &[Multiaddr]) -> Result<(), DomainError>;

    /// Looks up a peer's addresses by their user ID in the DHT.
    async fn lookup_peer(&self, user_id: &UserId) -> Result<Vec<Multiaddr>, DomainError>;

    /// Registers our own address in the DHT for other peers to discover us.
    async fn provide_address(&self, addr: &Multiaddr) -> Result<(), DomainError>;
}

#[cfg(test)]
mod tests {
    // Port trait definitions — compilation test only
    fn _assert_send_sync<T: Send + Sync>() {}

    #[test]
    fn test_port_traits_are_object_safe() {
        // Verify the traits can be used as trait objects
        fn _use_as_dyn(_: &dyn crate::application::ports::PeerDiscoveryPort) {}
        fn _use_dht_as_dyn(_: &dyn crate::application::ports::DhtPort) {}
    }
}
