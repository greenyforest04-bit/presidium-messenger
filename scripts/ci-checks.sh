#!/usr/bin/env bash
set -euo pipefail

# Presidium Messenger — CI Checks (local equivalent of GitHub Actions)
# Run this before pushing to catch issues early.

echo "=== Presidium Messenger — CI Checks ==="

echo ""
echo "--- [1/5] Formatting ---"
cargo fmt --all -- --check
echo "[PASS]"

echo ""
echo "--- [2/5] Clippy ---"
cargo clippy --workspace -- -D warnings
echo "[PASS]"

echo ""
echo "--- [3/5] Tests ---"
cargo test --workspace
echo "[PASS]"

echo ""
echo "--- [4/5] Documentation ---"
cargo doc --workspace --no-deps
echo "[PASS]"

echo ""
echo "--- [5/5] Build (release) ---"
cargo build --workspace --release
echo "[PASS]"

echo ""
echo "=== All CI checks passed! ==="
