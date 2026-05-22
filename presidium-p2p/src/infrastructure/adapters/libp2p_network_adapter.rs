use crate::application::ports::{NetworkError, P2PPort};
use async_trait::async_trait;
use presidium_core::domain::{DeviceId, UserId};

/// Stub adapter — will be replaced by libp2p integration
/// TODO(Day 5+): Implement real libp2p networking with Kademlia, GossipSub, Circuit Relay v2
pub struct Libp2pNetworkAdapter;

impl Default for Libp2pNetworkAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl Libp2pNetworkAdapter {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl P2PPort for Libp2pNetworkAdapter {
    async fn send_p2p(
        &self, _target_device: &DeviceId, _data: Vec<u8>,
    ) -> Result<(), NetworkError> {
        unimplemented!("TODO: Implement real P2P send via libp2p")
    }

    async fn receive_p2p(&self) -> Result<Option<Vec<u8>>, NetworkError> {
        unimplemented!("TODO: Implement real P2P receive via libp2p")
    }

    async fn publish_pre_keys(
        &self, _user_id: &UserId, _device_id: &DeviceId, _bundle: &[u8],
    ) -> Result<(), NetworkError> {
        unimplemented!("TODO: Implement pre-key publishing via GossipSub/DHT")
    }

    async fn fetch_pre_keys(
        &self, _user_id: &UserId, _device_id: &DeviceId,
    ) -> Result<Vec<u8>, NetworkError> {
        unimplemented!("TODO: Implement pre-key fetching via Kademlia DHT")
    }

    async fn subscribe_topic(&self, _topic: &str) -> Result<(), NetworkError> {
        unimplemented!("TODO: Implement GossipSub topic subscription")
    }
}
