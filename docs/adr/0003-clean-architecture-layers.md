# ADR 0003: Clean Architecture Layers

## Status

Accepted

## Date

2026-05-22

## Context

With ports and stub adapters defined (ADR 0002), we need a formal layer structure within each crate to enforce the Dependency Rule: source code dependencies must point inward only. Without explicit layering, the risk of infrastructure leaking into domain logic grows as the codebase expands.

Each crate in the Presidium Messenger workspace needs clear separation between:

- **Domain**: business entities, value objects, domain events, and domain-specific errors
- **Application**: use cases (interactors), port traits (output boundaries), application services
- **Infrastructure**: concrete adapter implementations for ports, external system integrations
- **Interfaces** (optional): FFI, HTTP, CLI entry points for external consumers

## Decision

We adopt Clean Architecture with the following layer organization for every crate:

```
src/
├── domain/               # Pure business logic, no external dependencies
│   ├── entities.rs       # Entities (Message, User, Session)
│   ├── value_objects.rs  # Value objects (UserId, MessageId, etc.)
│   ├── aggregates.rs     # Aggregates (ChatAggregate)
│   ├── events.rs         # Domain events (MessageSent, MessageDelivered)
│   └── errors.rs         # Domain-specific errors
├── application/          # Use cases and port definitions
│   ├── ports/            # Output port traits (implemented by infrastructure)
│   ├── use_cases/        # Interactors (SendMessageUseCase, etc.)
│   └── services/         # Application-level services
├── infrastructure/       # External system adapters
│   ├── adapters/         # Concrete port implementations
│   └── di.rs             # Dependency injection factory
└── interfaces/           # Optional: API boundaries (FFI, HTTP, CLI)
```

### Dependency Rules

1. **domain** may NOT import from application, infrastructure, or interfaces
2. **application** may import from domain and define port traits
3. **infrastructure** may import from application and domain
4. **interfaces** may import from application and infrastructure

### SendMessageUseCase as Reference Implementation

The `SendMessageInteractor<C, P, S, M>` in `presidium-messaging` demonstrates the pattern:

- Generic over port traits (`E2EECryptoPort`, `P2PPort`, `MessageStoragePort`, `ModerationPort`)
- Constructor injection via `new(crypto, p2p, storage, moderation)`
- Orchestrates: moderation check, session establishment, encryption, storage, P2P send
- Fully testable with mockall mocks in integration tests

### Dependency Injection

We use constructor injection (not service locator or global state). The `infrastructure/di.rs` module wires concrete adapters to use case interactors. This approach:

- Makes dependencies explicit at compile time
- Enables easy mocking for unit tests
- Allows swapping adapters without modifying use case logic
- Will evolve toward a builder pattern as the application grows

## Consequences

### Positive

- **Testability**: Each layer can be tested in isolation using mocks for its dependencies
- **Substitutability**: Adapters can be replaced without touching domain or application logic
- **Enforceability**: The layer dependency check script (`scripts/check_layer_deps.sh`) prevents violations in CI
- **Clarity**: New developers can locate code by function (domain logic vs. infrastructure integration)
- **E2EE integrity**: Domain layer has zero dependency on infrastructure, ensuring crypto logic cannot be bypassed

### Negative

- **Boilerplate**: More files and modules than a flat structure
- **Indirection**: Port traits add an abstraction layer; simple operations require trait definitions and adapter implementations
- **Learning curve**: Developers unfamiliar with Clean Architecture may need guidance

### Mitigation

- ADR 0002 already established the port/adapter pattern; this ADR extends it with explicit layers
- CI enforcement prevents accidental layer violations
- The SendMessageUseCase serves as a reference implementation for all future use cases
