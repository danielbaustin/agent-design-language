#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_ROOT="${1:-$ROOT/.adl/reports/demo-affect-godel-vertical-slice}"
GODEL_RUNS_DIR="$OUT_ROOT/runs"
GODEL_RUN_ID="review-godel-affect-001"

echo "[affect-godel-demo] root=$ROOT"
echo "[affect-godel-demo] out=$OUT_ROOT"

rm -rf "$OUT_ROOT"
mkdir -p "$GODEL_RUNS_DIR"

echo "[affect-godel-demo] step 1: refresh affect and reasoning graph artifacts"
"$ROOT/swarm/tools/demo_reasoning_graph_affect.sh" "$OUT_ROOT/aee"

echo "[affect-godel-demo] step 2: run deterministic godel stage loop"
cargo run --manifest-path "$ROOT/swarm/Cargo.toml" --bin adl -- godel run \
  --run-id "$GODEL_RUN_ID" \
  --workflow-id wf-godel-loop \
  --failure-code tool_failure \
  --failure-summary "deterministic parse failure" \
  --evidence-ref runs/source-run/run_status.json \
  --evidence-ref runs/source-run/logs/activation_log.json \
  --runs-dir "$GODEL_RUNS_DIR"

echo "[affect-godel-demo] step 3: derive affect-plus-godel vertical slice artifact"
cargo run --manifest-path "$ROOT/swarm/Cargo.toml" --bin adl -- godel affect-slice \
  --initial-run-id v0-3-aee-recovery-initial \
  --adapted-run-id v0-3-aee-recovery-adapted \
  --godel-run-id "$GODEL_RUN_ID" \
  --aee-runs-dir "$ROOT/.adl/runs" \
  --godel-runs-dir "$GODEL_RUNS_DIR"

SLICE_PATH="$GODEL_RUNS_DIR/$GODEL_RUN_ID/godel/godel_affect_vertical_slice.v1.json"
[[ -f "$SLICE_PATH" ]] || {
  echo "[affect-godel-demo] missing $SLICE_PATH" >&2
  exit 1
}

echo "[affect-godel-demo] persisted affect-plus-godel vertical slice artifact:"
cat "$SLICE_PATH"
echo
echo "[affect-godel-demo] PASS"
