#!/usr/bin/env bash
# Feature 003: automated tier + manual GUI performance checklist.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
RESULTS="specs/003-gui-performance-validation/validation-results.md"

echo "=== Feature 003: GUI performance validation ==="
echo ""

echo "--- Step 1: Automated tier (FR-005) ---"
./scripts/validate-feature-001.sh
cargo test sc003_filter_10k_under_200ms --quiet
echo ""

echo "--- Step 2: Fixture generator ---"
echo "Run: FIXTURE=\$(./scripts/generate-perf-fixture.sh 10000)"
echo ""

echo "--- Step 3: Manual GUI protocol ---"
echo "See: docs/UI-UX-REDESIGN.md (Sections 6 U0/U4)"
echo "  - V1 layout spot check"
echo "  - same-build, same-fixture rapid-scroll interaction; report observations without invented numeric threshold"
echo "  - RSS samples: ./scripts/sample-rss.sh; compare with inherited <150MB target when reproducible"
echo ""

if [[ ! -f "$RESULTS" ]]; then
  cat >"$RESULTS" <<'EOF'
# GUI Performance Validation Results

**Run ID**: pending
**Date**: pending
**Tester**: pending
**Environment**: pending

## Automated tier (FR-005)

| Check | Result |
|-------|--------|
| validate-feature-001.sh | pass |
| cargo test sc003_filter_10k_under_200ms | pass |

## Manual tier (SC-002, SC-004)

| Metric | Value | Threshold | Verdict |
|--------|-------|-----------|---------|
| Scroll smoothness (5s rapid drag) | pending | no freeze >500ms | pending |
| RSS peak (MB) | pending | <150 | pending |
| Images loaded | pending | ≥10000 | pending |

## Fixture

- Path: pending
- Generator: `scripts/generate-perf-fixture.sh`

## Gap audit update

- [ ] Updated 001 gap-audit SC-002 validated: pending
- [ ] Updated 001 gap-audit SC-004 validated: pending

## Notes

Complete the manual protocol in active UX plan U4, then fill this file.
EOF
  echo "Created stub: $RESULTS"
fi

echo ""
echo "=== Automated tier passed. Complete the manual protocol in active UX plan U4 ==="