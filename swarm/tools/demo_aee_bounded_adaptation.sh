#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_ROOT="${1:-$ROOT/.adl/reports/demo-aee-bounded-adaptation}"
STATE_ROOT="$OUT_ROOT/state"

echo "[aee-demo] root=$ROOT"
echo "[aee-demo] out=$OUT_ROOT"

rm -rf \
  "$ROOT/.adl/runs/v0-3-aee-recovery-initial" \
  "$ROOT/.adl/runs/v0-3-aee-recovery-adapted" \
  "$OUT_ROOT"
mkdir -p "$OUT_ROOT" "$STATE_ROOT"

echo "[aee-demo] step 1: run initial bounded recovery case (expected failure)"
set +e
ADL_OLLAMA_BIN="$ROOT/swarm/tools/mock_ollama_fail_once.sh" \
ADL_AEE_DEMO_STATE_DIR="$STATE_ROOT/initial" \
cargo run --manifest-path "$ROOT/swarm/Cargo.toml" --bin adl -- \
  "$ROOT/swarm/examples/v0-3-aee-recovery-initial.adl.yaml" \
  --run \
  --trace \
  --out "$OUT_ROOT/initial"
rc=$?
set -e
if [[ "$rc" -eq 0 ]]; then
  echo "[aee-demo] expected initial run to fail, but it succeeded" >&2
  exit 1
fi

decision_path="$ROOT/.adl/runs/v0-3-aee-recovery-initial/learning/aee_decision.json"
suggestions_path="$ROOT/.adl/runs/v0-3-aee-recovery-initial/learning/suggestions.json"
affect_path="$ROOT/.adl/runs/v0-3-aee-recovery-initial/learning/affect_state.v1.json"
[[ -f "$decision_path" ]] || { echo "[aee-demo] missing $decision_path" >&2; exit 1; }
[[ -f "$suggestions_path" ]] || { echo "[aee-demo] missing $suggestions_path" >&2; exit 1; }
[[ -f "$affect_path" ]] || { echo "[aee-demo] missing $affect_path" >&2; exit 1; }

echo "[aee-demo] initial affect state artifact:"
cat "$affect_path"
echo "[aee-demo] initial AEE decision artifact:"
cat "$decision_path"

echo "[aee-demo] step 2: rerun with bounded overlay aligned to the emitted decision"
ADL_OLLAMA_BIN="$ROOT/swarm/tools/mock_ollama_fail_once.sh" \
ADL_AEE_DEMO_STATE_DIR="$STATE_ROOT/adapted" \
cargo run --manifest-path "$ROOT/swarm/Cargo.toml" --bin adl -- \
  "$ROOT/swarm/examples/v0-3-aee-recovery-adapted.adl.yaml" \
  --run \
  --trace \
  --overlay "$ROOT/demos/aee-recovery/retry-budget.overlay.json" \
  --out "$OUT_ROOT/adapted"

adapted_summary="$ROOT/.adl/runs/v0-3-aee-recovery-adapted/run_summary.json"
adapted_decision="$ROOT/.adl/runs/v0-3-aee-recovery-adapted/learning/aee_decision.json"
adapted_affect="$ROOT/.adl/runs/v0-3-aee-recovery-adapted/learning/affect_state.v1.json"
[[ -f "$adapted_summary" ]] || { echo "[aee-demo] missing $adapted_summary" >&2; exit 1; }
[[ -f "$adapted_decision" ]] || { echo "[aee-demo] missing $adapted_decision" >&2; exit 1; }
[[ -f "$adapted_affect" ]] || { echo "[aee-demo] missing $adapted_affect" >&2; exit 1; }

echo "[aee-demo] adapted run summary:"
cat "$adapted_summary"
echo "[aee-demo] adapted affect state artifact:"
cat "$adapted_affect"
echo "[aee-demo] adapted AEE decision artifact:"
cat "$adapted_decision"
echo "[aee-demo] PASS"
