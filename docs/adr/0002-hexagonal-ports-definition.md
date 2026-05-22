# ADR 0002: Hexagonal Ports Definition

## Status

Accepted

## Date

2026-05-21

## Context

To enable testability and infrastructure independence, we need to define port traits (interfaces) that decouple business logic from external system implementations.

## Decision

We define 5 port traits using the Hexagonal Architecture (Ports & Adapters) pattern:

1. **E2EECryptoPort** (`presidium-crypto`) — PQXDH session establishment, Double Ratchet encrypt/decrypt
2. **P2PPort** (`presidium-p2p`) — P2P messaging, pre-key publishing/fetching, topic subscription
3. **MessageStoragePort** (`presidium-storage`) — message persistence, delivery/read status tracking
4. **LLMPort** (`presidium-llm`) — on-device model loading and inference
5. **ModerationPort** (`presidium-core`) — local content moderation and sarcophagus creation

Each port has a corresponding stub adapter with `unimplemented!()` methods, annotated with TODO comments for future implementation.

## Consequences

- Use cases depend on trait abstractions, not concrete implementations
- Mock implementations (mockall) enable isolated unit testing
- Real adapters can be developed incrementally without affecting business logic
