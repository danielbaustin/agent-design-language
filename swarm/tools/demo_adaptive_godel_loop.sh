#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_ROOT="${1:-$ROOT/.adl/reports/demo-adaptive-godel-loop}"
RUNS_DIR="$OUT_ROOT/runs"
RUN_ID="review-godel-policy-001"

echo "[adaptive-godel-loop-demo] root=$ROOT"
echo "[adaptive-godel-loop-demo] out=$OUT_ROOT"

rm -rf "$OUT_ROOT"
mkdir -p "$RUNS_DIR"

echo "[adaptive-godel-loop-demo] step 1: run deterministic bounded godel loop"
cargo run --manifest-path "$ROOT/swarm/Cargo.toml" --bin adl -- godel run \
  --run-id "$RUN_ID" \
  --workflow-id wf-godel-loop \
  --failure-code tool_failure \
  --failure-summary "deterministic parse failure" \
  --evidence-ref runs/source-run/run_status.json \
  --evidence-ref runs/source-run/logs/activation_log.json \
  --runs-dir "$RUNS_DIR"

echo "[adaptive-godel-loop-demo] step 2: inspect persisted policy-learning surfaces"
cargo run --manifest-path "$ROOT/swarm/Cargo.toml" --bin adl -- godel inspect \
  --run-id "$RUN_ID" \
  --runs-dir "$RUNS_DIR"

POLICY_PATH="$RUNS_DIR/$RUN_ID/godel/godel_policy.v1.json"
COMPARISON_PATH="$RUNS_DIR/$RUN_ID/godel/godel_policy_comparison.v1.json"
[[ -f "$POLICY_PATH" ]] || {
  echo "[adaptive-godel-loop-demo] missing $POLICY_PATH" >&2
  exit 1
}
[[ -f "$COMPARISON_PATH" ]] || {
  echo "[adaptive-godel-loop-demo] missing $COMPARISON_PATH" >&2
  exit 1
}

echo "[adaptive-godel-loop-demo] persisted policy artifact:"
cat "$POLICY_PATH"
echo
echo "[adaptive-godel-loop-demo] persisted policy comparison artifact:"
cat "$COMPARISON_PATH"
echo
echo "[adaptive-godel-loop-demo] PASS"
