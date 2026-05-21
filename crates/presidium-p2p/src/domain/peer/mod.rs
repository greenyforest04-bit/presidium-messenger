//! # Peer Domain Types
//!
//! Represents peers in the Presidium P2P network.

use presidium_core::domain::entities::UserId;
use presidium_core::domain::value_objects::Multiaddr;
use serde::{Deserialize, Serialize};

/// Connection state of a peer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PeerState {
    /// Peer is discovered but not yet connected.
    Disconnected,
    /// Connection is being established.
    Connecting,
    /// Peer is actively connected.
    Connected,
    /// Peer connection is being gracefully closed.
    Disconnecting,
}

/// A peer in the Presidium P2P network.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Peer {
    /// The user identity of this peer.
    pub user_id: UserId,
    /// Known multiaddresses for this peer.
    pub addresses: Vec<Multiaddr>,
    /// Current connection state.
    pub state: PeerState,
    /// Whether this peer is acting as a relay node.
    pub is_relay: bool,
    /// Monotonic timestamp of last seen activity.
    pub last_seen_ms: u64,
}

impl Peer {
    /// Creates a new peer with the given identity and address.
    ///
    /// # Errors
    ///
    /// Returns an error if the address is invalid.
    pub fn new(user_id: UserId, address: &str) -> Result<Self, &'static str> {
        Ok(Self {
            user_id,
            addresses: vec![Multiaddr::new(address)?],
            state: PeerState::Disconnected,
            is_relay: false,
            last_seen_ms: 0,
        })
    }

    /// Adds an additional address for this peer.
    ///
    /// # Errors
    ///
    /// Returns an error if the address is invalid.
    pub fn add_address(&mut self, addr: &str) -> Result<(), &'static str> {
        let multiaddr = Multiaddr::new(addr)?;
        if !self.addresses.contains(&multiaddr) {
            self.addresses.push(multiaddr);
        }
        Ok(())
    }

    /// Updates the peer's connection state.
    pub fn set_state(&mut self, state: PeerState) {
        self.state = state;
        if state == PeerState::Connected {
            self.last_seen_ms = current_timestamp_ms();
        }
    }
}

/// Returns the current Unix timestamp in milliseconds.
fn current_timestamp_ms() -> u64 {
    #[allow(clippy::cast_possible_truncation)]
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_millis() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_peer_creation() {
        let user = UserId::new([1u8; 32]);
        let peer = Peer::new(user, "/ip4/127.0.0.1/tcp/4001").expect("valid addr");
        assert_eq!(peer.state, PeerState::Disconnected);
        assert_eq!(peer.addresses.len(), 1);
        assert!(!peer.is_relay);
    }

    #[test]
    fn test_peer_add_address() {
        let user = UserId::new([1u8; 32]);
        let mut peer = Peer::new(user, "/ip4/1.2.3.4/tcp/4001").expect("addr");
        peer.add_address("/ip6/::1/tcp/4001").expect("add addr");
        assert_eq!(peer.addresses.len(), 2);
    }

    #[test]
    fn test_peer_state_transitions() {
        let user = UserId::new([1u8; 32]);
        let mut peer = Peer::new(user, "/ip4/127.0.0.1/tcp/4001").expect("addr");
        assert_eq!(peer.state, PeerState::Disconnected);

        peer.set_state(PeerState::Connecting);
        assert_eq!(peer.state, PeerState::Connecting);

        peer.set_state(PeerState::Connected);
        assert_eq!(peer.state, PeerState::Connected);
        assert!(peer.last_seen_ms > 0);
    }

    #[test]
    fn test_peer_rejects_empty_address() {
        let user = UserId::new([1u8; 32]);
        assert!(Peer::new(user, "").is_err());
    }
}
