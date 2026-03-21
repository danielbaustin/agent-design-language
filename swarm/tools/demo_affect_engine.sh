#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_ROOT="${1:-$ROOT/.adl/reports/demo-affect-engine}"

echo "[affect-demo] running bounded affect engine demo"
"$ROOT/swarm/tools/demo_aee_bounded_adaptation.sh" "$OUT_ROOT"

echo "[affect-demo] primary proof artifacts:"
echo "  .adl/runs/v0-3-aee-recovery-initial/learning/affect_state.v1.json"
echo "  .adl/runs/v0-3-aee-recovery-initial/learning/aee_decision.json"
echo "  .adl/runs/v0-3-aee-recovery-adapted/learning/affect_state.v1.json"
echo "  .adl/runs/v0-3-aee-recovery-adapted/learning/aee_decision.json"

echo "[affect-demo] PASS"
