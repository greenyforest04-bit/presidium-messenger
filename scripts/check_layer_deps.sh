#!/usr/bin/env bash
# check_layer_deps.sh — Verify Clean Architecture layer dependency rules
#
# The Dependency Rule: source code dependencies must point inward only.
# domain → (nothing)
# application → domain
# infrastructure → application, domain
# interfaces → application, infrastructure
#
# This script checks that:
# 1. domain/ modules do NOT import from application/ or infrastructure/
# 2. application/ modules do NOT import from infrastructure/
#
# Usage: ./scripts/check_layer_deps.sh [--fix]
# Exit code: 0 = pass, 1 = violation found

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
CRATES=("presidium-core" "presidium-crypto" "presidium-p2p" "presidium-storage" "presidium-llm" "presidium-messaging" "presidium-bridge")

VIOLATIONS=0

for crate in "${CRATES[@]}"; do
    CRATE_DIR="$PROJECT_ROOT/$crate"
    [ -d "$CRATE_DIR" ] || continue

    DOMAIN_DIR="$CRATE_DIR/src/domain"
    [ -d "$DOMAIN_DIR" ] || continue

    # Check 1: domain/ should NOT import from application/ or infrastructure/
    if [ -d "$DOMAIN_DIR" ]; then
        while IFS= read -r -d '' file; do
            # Check for imports from same-crate application or infrastructure
            BASENAME=$(basename "$file")
            # Skip mod.rs files which just declare submodules
            if [ "$BASENAME" = "mod.rs" ]; then
                continue
            fi

            CRATE_NAME=$(echo "$crate" | tr '-' '_')

            # Check for `use crate::application` or `use crate::infrastructure` in domain files
            if rg -n "use crate::(application|infrastructure)" "$file" 2>/dev/null; then
                echo "VIOLATION: $file imports from application/ or infrastructure/ (Dependency Rule)"
                VIOLATIONS=$((VIOLATIONS + 1))
            fi

            # Also check for external crate imports that reference presidium application/infrastructure
            if rg -n "use ${CRATE_NAME}::(application|infrastructure)" "$file" 2>/dev/null; then
                echo "VIOLATION: $file imports from ${CRATE_NAME}::application or infrastructure (Dependency Rule)"
                VIOLATIONS=$((VIOLATIONS + 1))
            fi
        done < <(find "$DOMAIN_DIR" -name "*.rs" -print0)
    fi

    # Check 2: application/ should NOT import from infrastructure/
    APP_DIR="$CRATE_DIR/src/application"
    if [ -d "$APP_DIR" ]; then
        while IFS= read -r -d '' file; do
            BASENAME=$(basename "$file")
            if [ "$BASENAME" = "mod.rs" ]; then
                continue
            fi

            if rg -n "use crate::infrastructure" "$file" 2>/dev/null; then
                echo "VIOLATION: $file imports from infrastructure/ (Dependency Rule: application cannot depend on infrastructure)"
                VIOLATIONS=$((VIOLATIONS + 1))
            fi
        done < <(find "$APP_DIR" -name "*.rs" -print0)
    fi
done

if [ "$VIOLATIONS" -gt 0 ]; then
    echo ""
    echo "Found $VIOLATIONS layer dependency violation(s)."
    echo "See: docs/adr/0003-clean-architecture-layers.md"
    exit 1
fi

echo "All layer dependency checks passed."
exit 0
