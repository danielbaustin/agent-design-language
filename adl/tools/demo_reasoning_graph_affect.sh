#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_ROOT="${1:-$ROOT/.adl/reports/demo-reasoning-graph-affect}"
RUNS_ROOT="${ADL_RUNS_ROOT:-$ROOT/.adl/runs}"

echo "[reasoning-graph-demo] running affect-linked reasoning graph demo"
"$ROOT/adl/tools/demo_affect_engine.sh" "$OUT_ROOT"

echo "[reasoning-graph-demo] initial reasoning graph artifact:"
cat "$RUNS_ROOT/v0-3-aee-recovery-initial/learning/reasoning_graph.v1.json"

echo "[reasoning-graph-demo] adapted reasoning graph artifact:"
cat "$RUNS_ROOT/v0-3-aee-recovery-adapted/learning/reasoning_graph.v1.json"

echo "[reasoning-graph-demo] PASS"
