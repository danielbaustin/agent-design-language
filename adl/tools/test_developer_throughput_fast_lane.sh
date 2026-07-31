#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
POLICY="$ROOT/docs/tooling/DEVELOPER_THROUGHPUT_FAST_LANE.md"
ROUTING="$ROOT/docs/tooling/VALIDATION_PLATFORM_ROUTING.md"

test -s "$POLICY"
test -s "$ROUTING"

grep -F "Proportional issue classes" "$POLICY" >/dev/null
grep -F "FastWork-required mode" "$POLICY" >/dev/null
grep -F "changed-state-only PR watching" "$POLICY" >/dev/null
grep -F "Do not wait on GitHub when no action is possible" "$POLICY" >/dev/null
grep -F "typed C-SDLC v2 remains the lifecycle authority" "$POLICY" >/dev/null
grep -F "docs/architecture/VALIDATION_LANE_SELECTOR.md" "$POLICY" >/dev/null
grep -F "Do not silently fall back to the local disk" "$POLICY" >/dev/null
grep -F "selector output is \`escalated\` or \`release_gate_required\`" "$POLICY" >/dev/null

grep -F "DEVELOPER_THROUGHPUT_FAST_LANE.md" "$ROUTING" >/dev/null
