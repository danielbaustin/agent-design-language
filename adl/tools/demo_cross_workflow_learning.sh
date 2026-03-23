#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_ROOT="${1:-$ROOT/.adl/reports/demo-cross-workflow-learning}"
RUNS_DIR="$OUT_ROOT/runs"
RUN_ID="review-godel-crossflow-001"

echo "[cross-workflow-learning-demo] root=$ROOT"
echo "[cross-workflow-learning-demo] out=$OUT_ROOT"

rm -rf "$OUT_ROOT"
mkdir -p "$RUNS_DIR"

echo "[cross-workflow-learning-demo] step 1: run deterministic godel learning flow"
cargo run --manifest-path "$ROOT/adl/Cargo.toml" --bin adl -- godel run \
  --run-id "$RUN_ID" \
  --workflow-id wf-godel-loop \
  --failure-code tool_failure \
  --failure-summary "deterministic parse failure" \
  --evidence-ref runs/source-run/run_status.json \
  --evidence-ref runs/source-run/logs/activation_log.json \
  --runs-dir "$RUNS_DIR"

echo "[cross-workflow-learning-demo] step 2: inspect downstream learning decision"
cargo run --manifest-path "$ROOT/adl/Cargo.toml" --bin adl -- godel inspect \
  --run-id "$RUN_ID" \
  --runs-dir "$RUNS_DIR"

CROSS_PATH="$RUNS_DIR/$RUN_ID/godel/godel_cross_workflow_learning.v1.json"
[[ -f "$CROSS_PATH" ]] || {
  echo "[cross-workflow-learning-demo] missing $CROSS_PATH" >&2
  exit 1
}

echo "[cross-workflow-learning-demo] persisted cross-workflow learning artifact:"
cat "$CROSS_PATH"
echo
echo "[cross-workflow-learning-demo] PASS"
