# ADR 0001: Cargo Workspace with Hexagonal Architecture

**Date:** 2026-05-22
**Status:** Accepted
**Deciders:** Presidium Core Team

## Context

Presidium Messenger is a complex, long-term project (365+ days) requiring a
decentralized E2EE P2P messenger with on-device LLM. The initial architecture
decision must support:

1. **Multi-platform targets** — Android (primary), iOS, Desktop (later).
2. **Strict domain isolation** — Crypto, P2P, storage, and LLM subsystems must
   evolve independently without coupling.
3. **Testability** — Every use case must be testable with mock adapters.
4. **Long-term maintainability** — New developers must understand the architecture
   within minutes.
5. **Performance** — Rust compilation times must stay manageable as the codebase
   grows.

### Considered Alternatives

| Approach | Pros | Cons |
|----------|------|------|
| **Single binary crate** | Simplest setup | No isolation, monolithic dependencies, slow compiles |
| **Multi-repo (one per crate)** | Full isolation | Cross-repo refactoring is painful, CI complexity |
| **Cargo workspace + flat modules** | Fast compiles | No architectural boundaries, "big ball of mud" risk |
| **Cargo workspace + Hexagonal** | Clean boundaries, testability, domain isolation | More boilerplate, learning curve |

## Decision

We adopt a **Cargo workspace** with **7 crates**, each following
**Hexagonal Architecture** (Ports & Adapters) layered on top of **Clean Architecture**
and **Domain-Driven Design** principles.

### Workspace Structure

```
presidium/
├── crates/
│   ├── presidium-core/         # Domain entities, ports, config, observability
│   ├── presidium-crypto/       # E2EE (Signal Protocol, PQXDH, Double Ratchet)
│   ├── presidium-p2p/          # P2P networking (libp2p: Kademlia, GossipSub)
│   ├── presidium-storage/      # Persistent storage (encrypted SQLite)
│   ├── presidium-llm/          # On-device LLM (GGUF inference, moderation)
│   ├── presidium-messaging/    # Messaging use cases (orchestration)
│   └── presidium-bridge/       # UniFFI bindings for Kotlin/Swift interop
```

### Layer Rules (within each crate)

```
domain/          → No external dependencies. Pure business logic.
application/     → Depends on domain only. Defines ports (traits) and use cases.
infrastructure/  → Depends on application ports. Implements adapters.
```

### Dependency Rule

Dependencies point **inward** — from infrastructure toward domain. The domain
layer never depends on infrastructure, frameworks, or external libraries.

## Consequences

### Positive

- **Clear boundaries** — Each crate has a well-defined responsibility.
- **Fast incremental builds** — Only changed crates need recompilation.
- **Testability** — Use cases can be tested with stub/mock adapters from Day 1.
- **Parallel development** — Teams can work on different crates simultaneously.
- **Platform flexibility** — `presidium-bridge` isolates all FFI concerns.

### Negative

- **Boilerplate** — Each new use case requires a port trait + adapter impl.
- **Initial complexity** — More files and directories than a flat structure.
- **Refactoring cost** — Moving types between layers requires careful migration.

### Mitigations

- Use `cargo alias` for common multi-crate operations.
- Document all port traits with examples to reduce onboarding friction.
- ADR process captures all future architectural changes.
