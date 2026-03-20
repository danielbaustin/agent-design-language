#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_ROOT="${1:-$ROOT/.adl/reports/demo-promotion-eval-loop}"
RUNS_DIR="$OUT_ROOT/runs"
RUN_ID="review-godel-promotion-001"

echo "[promotion-eval-loop-demo] root=$ROOT"
echo "[promotion-eval-loop-demo] out=$OUT_ROOT"

rm -rf "$OUT_ROOT"
mkdir -p "$RUNS_DIR"

echo "[promotion-eval-loop-demo] step 1: run deterministic promotion/eval flow"
cargo run --manifest-path "$ROOT/swarm/Cargo.toml" --bin adl -- godel run \
  --run-id "$RUN_ID" \
  --workflow-id wf-godel-loop \
  --failure-code tool_failure \
  --failure-summary "deterministic parse failure" \
  --evidence-ref runs/source-run/run_status.json \
  --evidence-ref runs/source-run/logs/activation_log.json \
  --runs-dir "$RUNS_DIR"

echo "[promotion-eval-loop-demo] step 2: inspect evaluation and promotion outputs"
cargo run --manifest-path "$ROOT/swarm/Cargo.toml" --bin adl -- godel inspect \
  --run-id "$RUN_ID" \
  --runs-dir "$RUNS_DIR"

EVAL_PATH="$RUNS_DIR/$RUN_ID/godel/godel_eval_report.v1.json"
PROMOTION_PATH="$RUNS_DIR/$RUN_ID/godel/godel_promotion_decision.v1.json"
[[ -f "$EVAL_PATH" ]] || {
  echo "[promotion-eval-loop-demo] missing $EVAL_PATH" >&2
  exit 1
}
[[ -f "$PROMOTION_PATH" ]] || {
  echo "[promotion-eval-loop-demo] missing $PROMOTION_PATH" >&2
  exit 1
}

echo "[promotion-eval-loop-demo] persisted evaluation artifact:"
cat "$EVAL_PATH"
echo
echo "[promotion-eval-loop-demo] persisted promotion decision artifact:"
cat "$PROMOTION_PATH"
echo
echo "[promotion-eval-loop-demo] PASS"
