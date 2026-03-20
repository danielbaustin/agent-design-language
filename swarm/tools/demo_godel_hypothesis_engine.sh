#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_ROOT="${1:-$ROOT/.adl/reports/demo-godel-hypothesis-engine}"
RUNS_DIR="$OUT_ROOT/runs"
RUN_ID="review-godel-cli-001"

echo "[godel-hypothesis-demo] root=$ROOT"
echo "[godel-hypothesis-demo] out=$OUT_ROOT"

rm -rf "$OUT_ROOT"
mkdir -p "$RUNS_DIR"

echo "[godel-hypothesis-demo] step 1: generate deterministic hypothesis artifacts"
cargo run --manifest-path "$ROOT/swarm/Cargo.toml" --bin adl -- godel run \
  --run-id "$RUN_ID" \
  --workflow-id wf-godel-loop \
  --failure-code tool_failure \
  --failure-summary "deterministic parse failure" \
  --evidence-ref runs/source-run/run_status.json \
  --evidence-ref runs/source-run/logs/activation_log.json \
  --runs-dir "$RUNS_DIR"

echo "[godel-hypothesis-demo] step 2: inspect the persisted hypothesis surface"
cargo run --manifest-path "$ROOT/swarm/Cargo.toml" --bin adl -- godel inspect \
  --run-id "$RUN_ID" \
  --runs-dir "$RUNS_DIR"

HYPOTHESIS_PATH="$RUNS_DIR/$RUN_ID/godel/godel_hypothesis.v1.json"
[[ -f "$HYPOTHESIS_PATH" ]] || {
  echo "[godel-hypothesis-demo] missing $HYPOTHESIS_PATH" >&2
  exit 1
}

echo "[godel-hypothesis-demo] persisted hypothesis artifact:"
cat "$HYPOTHESIS_PATH"
echo "[godel-hypothesis-demo] PASS"
