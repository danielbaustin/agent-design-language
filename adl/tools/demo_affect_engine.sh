#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_ROOT="${1:-$ROOT/.adl/reports/demo-affect-engine}"
RUNS_ROOT="${ADL_RUNS_ROOT:-$ROOT/.adl/runs}"

echo "[affect-demo] running bounded affect engine demo"
"$ROOT/adl/tools/demo_aee_bounded_adaptation.sh" "$OUT_ROOT"

echo "[affect-demo] primary proof artifacts:"
echo "  $RUNS_ROOT/v0-3-aee-recovery-initial/learning/affect_state.v1.json"
echo "  $RUNS_ROOT/v0-3-aee-recovery-initial/learning/aee_decision.json"
echo "  $RUNS_ROOT/v0-3-aee-recovery-adapted/learning/affect_state.v1.json"
echo "  $RUNS_ROOT/v0-3-aee-recovery-adapted/learning/aee_decision.json"

echo "[affect-demo] PASS"
