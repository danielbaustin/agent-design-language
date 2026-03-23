#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_ROOT="${1:-$ROOT/.adl/reports/demo-experiment-prioritization}"
RUNS_DIR="$OUT_ROOT/runs"
RUN_ID="review-godel-priority-001"

echo "[experiment-prioritization-demo] root=$ROOT"
echo "[experiment-prioritization-demo] out=$OUT_ROOT"

rm -rf "$OUT_ROOT"
mkdir -p "$RUNS_DIR"

echo "[experiment-prioritization-demo] step 1: run deterministic godel prioritization flow"
cargo run --manifest-path "$ROOT/adl/Cargo.toml" --bin adl -- godel run \
  --run-id "$RUN_ID" \
  --workflow-id wf-godel-loop \
  --failure-code tool_failure \
  --failure-summary "deterministic parse failure" \
  --evidence-ref runs/source-run/run_status.json \
  --evidence-ref runs/source-run/logs/activation_log.json \
  --runs-dir "$RUNS_DIR"

echo "[experiment-prioritization-demo] step 2: inspect ranked experiment output"
cargo run --manifest-path "$ROOT/adl/Cargo.toml" --bin adl -- godel inspect \
  --run-id "$RUN_ID" \
  --runs-dir "$RUNS_DIR"

PRIORITY_PATH="$RUNS_DIR/$RUN_ID/godel/godel_experiment_priority.v1.json"
[[ -f "$PRIORITY_PATH" ]] || {
  echo "[experiment-prioritization-demo] missing $PRIORITY_PATH" >&2
  exit 1
}

echo "[experiment-prioritization-demo] persisted prioritization artifact:"
cat "$PRIORITY_PATH"
echo
echo "[experiment-prioritization-demo] PASS"
