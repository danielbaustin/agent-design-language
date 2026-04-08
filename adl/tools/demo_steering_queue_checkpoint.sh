#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_ROOT="${1:-$ROOT/.adl/reports/demo-steering-queue-checkpoint}"
OUT_DIR="$OUT_ROOT/out"
RUN_ID="v0-85-hitl-steering-demo"
RUN_JSON="$ROOT/.adl/runs/$RUN_ID/run.json"
PAUSE_STATE="$ROOT/.adl/runs/$RUN_ID/pause_state.json"
STEER_PATH="$OUT_ROOT/steer.json"

echo "[steering-demo] root=$ROOT"
echo "[steering-demo] out=$OUT_ROOT"

rm -rf "$OUT_ROOT" "$ROOT/.adl/runs/$RUN_ID"
mkdir -p "$OUT_DIR"
cp "$ROOT/demos/v0.85/steering_queue_checkpoint_patch.json" "$STEER_PATH"

echo "[steering-demo] step 1: execute until the checkpoint boundary"
ADL_OLLAMA_BIN="$ROOT/adl/tools/mock_ollama_v0_4.sh" \
cargo run --manifest-path "$ROOT/adl/Cargo.toml" --bin adl -- \
  "$ROOT/adl/examples/v0-85-hitl-steering-demo.adl.yaml" \
  --run \
  --trace \
  --allow-unsigned \
  --out "$OUT_DIR"

[[ -f "$PAUSE_STATE" ]] || {
  echo "[steering-demo] missing $PAUSE_STATE" >&2
  exit 1
}
[[ -f "$RUN_JSON" ]] || {
  echo "[steering-demo] missing $RUN_JSON" >&2
  exit 1
}

echo "[steering-demo] step 2: resume with explicit steering patch"
ADL_OLLAMA_BIN="$ROOT/adl/tools/mock_ollama_v0_4.sh" \
cargo run --manifest-path "$ROOT/adl/Cargo.toml" --bin adl -- \
  "$ROOT/adl/examples/v0-85-hitl-steering-demo.adl.yaml" \
  --run \
  --resume "$RUN_JSON" \
  --steer "$STEER_PATH" \
  --allow-unsigned \
  --out "$OUT_DIR"

[[ -f "$OUT_DIR/s2.txt" ]] || {
  echo "[steering-demo] missing resumed output $OUT_DIR/s2.txt" >&2
  exit 1
}

grep -q "steered-topic" "$OUT_DIR/s2.txt" || {
  echo "[steering-demo] resumed output does not contain steered-topic" >&2
  exit 1
}

python3 - "$PAUSE_STATE" "$RUN_JSON" <<'PY'
import json
import sys
from pathlib import Path

pause_state = json.loads(Path(sys.argv[1]).read_text())
run_json = json.loads(Path(sys.argv[2]).read_text())

assert pause_state["status"] == "paused", pause_state["status"]
assert pause_state["pause"]["paused_step_id"] == "s1", pause_state["pause"]["paused_step_id"]
assert "s2" in pause_state["pause"]["remaining_step_ids"], pause_state["pause"]["remaining_step_ids"]

history = run_json.get("steering_history", [])
assert history, "missing steering_history"
assert history[0]["apply_at"] == "resume_boundary", history[0]
assert "inputs.topic" in history[0]["set_state_keys"], history[0]
PY

echo "[steering-demo] pause_state.json:"
cat "$PAUSE_STATE"
echo
echo "[steering-demo] steer.json:"
cat "$STEER_PATH"
echo
echo "[steering-demo] resumed s2.txt:"
cat "$OUT_DIR/s2.txt"
echo
echo "[steering-demo] PASS"
