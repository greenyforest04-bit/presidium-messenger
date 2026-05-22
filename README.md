# Presidium Messenger

<<<<<<< HEAD
Decentralized, end-to-end encrypted (E2EE) peer-to-peer messenger of the next generation.

## Overview

Presidium Messenger is a privacy-first messaging platform built entirely in Rust. It features:

- **E2EE via Signal Protocol** — PQXDH + Double Ratchet with post-quantum support
- **P2P networking** — libp2p stack (Kademlia DHT, GossipSub, Circuit Relay v2)
- **On-device LLM** — Local content moderation and AI assistant (Gemma-2B / Phi-3)
- **Every device strengthens the network** — More relays, better discovery, distributed compute
- **Mobile-first** — Kotlin Multiplatform + Jetpack Compose via UniFFI

## Architecture

This project uses **Hexagonal Architecture** (Ports & Adapters) with **Clean Architecture**
and **Domain-Driven Design**. See [`docs/adr/`](docs/adr/) for all Architecture Decision Records.

### Crate Structure

| Crate | Purpose |
|-------|---------|
| `presidium-core` | Domain entities, application ports, configuration, observability |
| `presidium-crypto` | E2EE cryptography (Signal Protocol, key management) |
| `presidium-p2p` | P2P networking (libp2p, peer discovery, DHT) |
| `presidium-storage` | Persistent storage (encrypted, ACID-compliant) |
| `presidium-llm` | On-device LLM inference (moderation, assistant) |
| `presidium-messaging` | Messaging use cases (orchestration layer) |
| `presidium-bridge` | UniFFI bindings for Kotlin/Swift interop |

## Getting Started

### Prerequisites

- **Rust** stable 1.81+ (install via [rustup](https://rustup.rs/))
- **Cargo** components: `rustfmt`, `clippy`
- **Optional tools**: `cargo-deny`, `cargo-audit`

### Quick Start

```bash
# Clone the repository
git clone https://github.com/greenyforest04-bit/presidium-messenger.git
cd presidium-messenger

# Run the setup script (installs tools, verifies build)
bash scripts/setup.sh

# Or manually:
cargo build --workspace
cargo test --workspace
```

### Development Commands

```bash
# Check compilation
cargo check --workspace

# Run all tests
cargo test --workspace

# Format code
cargo fmt --all

# Lint with Clippy (strict mode)
cargo clippy --workspace -- -D warnings

# Generate documentation
cargo doc --workspace --no-deps

# Run all CI checks locally
bash scripts/ci-checks.sh
```

### Adding a New Crate

1. Create `crates/presidium-<name>/Cargo.toml`
2. Add `"crates/presidium-<name>"` to `workspace.members` in root `Cargo.toml`
3. Follow the Hexagonal structure: `src/{domain,application,infrastructure}/`
4. Add the crate as a dependency to other crates as needed

## Project Status

See [`docs/project-status.md`](docs/project-status.md) for the current development progress.

## License

AGPL-3.0-or-later — see [LICENSE](LICENSE) for details.
=======
> Decentralized, End-to-End Encrypted, Peer-to-Peer messenger with on-device LLM moderation

## Architecture

The project follows **Clean Architecture** with **Hexagonal (Ports & Adapters)** pattern and **Domain-Driven Design** principles.

### Layer Organization (per crate)

```
src/
├── domain/               # Entities, value objects, aggregates, events, errors
├── application/          # Use cases (interactors), port traits, services
├── infrastructure/       # Concrete adapter implementations, DI factory
└── interfaces/           # FFI, HTTP, CLI entry points (optional)
```

**Dependency Rule**: dependencies point inward only. `domain` has zero external dependencies. `application` depends on `domain`. `infrastructure` depends on `application` and `domain`.

### Workspace Crates

| Crate | Purpose |
|-------|---------|
| `presidium-core` | Shared domain types, ModerationPort, domain events |
| `presidium-crypto` | E2EE port (PQXDH + Double Ratchet) |
| `presidium-p2p` | P2P networking port (libp2p) |
| `presidium-storage` | Message storage port (redb) |
| `presidium-llm` | On-device LLM inference port (candle.rs) |
| `presidium-messaging` | Use cases and orchestration |
| `presidium-bridge` | UniFFI bindings for Kotlin Multiplatform |

## Development

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
./scripts/check_layer_deps.sh
```

## License

AGPL-3.0-or-later
>>>>>>> 9073fbc (feat(architecture): implement clean architecture layers with SendMessageUseCase)
