#!/usr/bin/env bash
set -euo pipefail

source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/v0871_demo_common.sh"

ROOT_DIR="$(v0871_demo_repo_root)"
OUT_DIR="${1:-$ROOT_DIR/artifacts/v0871/shepherd_recovery}"
RUN_ID="v0-6-hitl-pause-demo"
STATUS_FILE="$OUT_DIR/runtime/runs/$RUN_ID/run_status.json"
PAUSE_FILE="$OUT_DIR/runtime/runs/$RUN_ID/pause_state.json"
TRACE_FILE="$OUT_DIR/runtime/runs/$RUN_ID/logs/trace_v1.json"
SUMMARY_FILE="$OUT_DIR/shepherd_recovery_summary.json"

rm -rf "$OUT_DIR"
bash "$ROOT_DIR/adl/tools/demo_v0871_runtime_state.sh" "$OUT_DIR" >/dev/null

python3 - "$STATUS_FILE" "$PAUSE_FILE" "$SUMMARY_FILE" <<'PY'
import json
import sys

status_path, pause_path, out_path = sys.argv[1:4]
with open(status_path, encoding="utf-8") as fh:
    status = json.load(fh)
with open(pause_path, encoding="utf-8") as fh:
    pause = json.load(fh)

payload = {
    "demo_id": "D4A",
    "run_id": status.get("run_id"),
    "resilience_classification": status.get("resilience_classification"),
    "continuity_status": status.get("continuity_status"),
    "preservation_status": status.get("preservation_status"),
    "shepherd_decision": status.get("shepherd_decision"),
    "cleanup_disposition": status.get("cleanup_disposition"),
    "pause_status": pause.get("status"),
}

with open(out_path, "w", encoding="utf-8") as fh:
    json.dump(payload, fh, indent=2)
    fh.write("\n")
PY

SECONDARIES="$(printf '%s\n%s\n%s' \
  "$STATUS_FILE" \
  "$PAUSE_FILE" \
  "$TRACE_FILE")"

v0871_demo_write_readme \
  "$OUT_DIR" \
  "v0.87.1 Demo D4A - Shepherd Preservation And Recovery" \
  $'bash adl/tools/demo_v0871_shepherd_recovery.sh' \
  "$SUMMARY_FILE" \
  "$SECONDARIES" \
  "the paused run remains resume-ready with preserved pause_state.json and an explicit shepherd decision instead of being treated as disposable failure"

v0871_demo_print_proof_surfaces "$SUMMARY_FILE" "$SECONDARIES"
