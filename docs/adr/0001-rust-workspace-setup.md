# ADR 0001: Rust Workspace Setup

## Status

Accepted

## Date

2026-05-20

## Context

Presidium Messenger requires a modular architecture with clear crate boundaries for E2EE crypto, P2P networking, storage, LLM inference, messaging logic, and mobile bridge.

## Decision

We use a Cargo workspace with 7 crates:
- `presidium-core` — shared domain types, errors, and the ModerationPort
- `presidium-crypto` — E2EE crypto port and adapters (libsignal-protocol-rust)
- `presidium-p2p` — P2P networking port and adapters (libp2p)
- `presidium-storage` — message storage port and adapters (redb)
- `presidium-llm` — on-device LLM inference port and adapters (candle.rs)
- `presidium-messaging` — messaging use cases and orchestration
- `presidium-bridge` — UniFFI bindings for Kotlin Multiplatform

## Consequences

- Clear separation of concerns at the crate level
- Each crate can be independently tested and versioned
- Workspace-level dependency management ensures version consistency
