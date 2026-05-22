# Presidium Messenger — Project Status

## Overview

Decentralized, E2EE, P2P messenger with on-device LLM moderation and "Sarcophagus" mechanism.

## Progress

### Day 1: Workspace Setup ✅
- Cargo workspace with 7 crates
- CI/CD pipeline (GitHub Actions)
- Quality tools: rustfmt, clippy, cargo-deny, cargo-audit
- Configuration: figment (presidium.toml / env vars)
- Observability: tracing-subscriber with EnvFilter

### Day 2: Hexagonal Ports & Stub Adapters ✅
- 5 port traits defined: E2EECryptoPort, P2PPort, MessageStoragePort, LLMPort, ModerationPort
- 5 stub adapters: LibSignalCryptoAdapter, Libp2pNetworkAdapter, RedbStorageAdapter, CandleLlmAdapter, LocalModerationAdapter
- Common types in presidium-core/domain (UserId, DeviceId, MessageId, SessionId, Timestamp, PreKeyBundle)
- Domain errors in presidium-core/domain/errors.rs
- ADR 0002 created

### Day 3: Clean Architecture Layers Setup ✅
- Each crate organized into domain/application/infrastructure/interfaces layers
- Domain events (MessageSent, MessageDelivered) in presidium-core
- Message entity and MessageStatus in presidium-messaging/domain
- SendMessageUseCase (first working use case) with full orchestration flow
- DI factory in presidium-messaging/infrastructure/di.rs
- Unit tests with mockall (5 test cases: success, moderation block, crypto/network/storage failure)
- Layer dependency check script (scripts/check_layer_deps.sh) integrated into CI
- ADR 0003 created

## Next Steps

### Day 4: Domain-Driven Design — Entities, Value Objects and Aggregates
- Deepen domain model for messages and sessions
- Define ChatAggregate with invariants
- Add domain services for session management
