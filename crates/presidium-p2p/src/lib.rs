//! # Presidium P2P
//!
//! Peer-to-peer networking subsystem for Presidium Messenger.
//!
//! This crate wraps the `libp2p` stack and exposes a clean,
//! domain-friendly API for:
//! - Peer discovery (Kademlia DHT, mDNS)
//! - Message routing (GossipSub, direct messages)
//! - NAT traversal (Circuit Relay v2, hole punching)
//! - Transport protocols (QUIC, WebRTC)
//!
//! ## Status
//!
//! Day 1 skeleton — all implementations are stubs awaiting
//! actual libp2p integration in subsequent development days.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod application;
pub mod domain;
pub mod infrastructure;
