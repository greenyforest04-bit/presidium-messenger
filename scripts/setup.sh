#!/usr/bin/env bash
set -euo pipefail

# Presidium Messenger — Development Environment Setup
# Run this script once after cloning the repository.

echo "=== Presidium Messenger — Setup ==="

# Check Rust installation
if ! command -v rustc &>/dev/null; then
    echo "[ERROR] Rust is not installed. Install it from https://rustup.rs/"
    exit 1
fi

echo "[OK] Rust $(rustc --version)"

# Check cargo components
for component in clippy rustfmt; do
    if ! rustup component list | grep -q "$component (installed)"; then
        echo "[SETUP] Installing $component..."
        rustup component add "$component"
    fi
done
echo "[OK] All components installed"

# Install cargo tools
CARGO_TOOLS=("cargo-deny" "cargo-audit" "cargo-nextest")
for tool in "${CARGO_TOOLS[@]}"; do
    if ! command -v "$tool" &>/dev/null; then
        echo "[SETUP] Installing $tool..."
        cargo install "$tool"
    else
        echo "[OK] $tool already installed"
    fi
done

# Create data directory
mkdir -p data
echo "[OK] Created ./data directory"

# Verify workspace builds
echo ""
echo "=== Building workspace ==="
cargo check --workspace
echo "[OK] Workspace compiles successfully"

# Run tests
echo ""
echo "=== Running tests ==="
cargo test --workspace
echo "[OK] All tests pass"

echo ""
echo "=== Setup complete! ==="
echo "Run 'cargo clippy --workspace -- -D warnings' for linting"
echo "Run 'cargo doc --workspace --no-deps' for documentation"
