#!/usr/bin/env bash
set -euo pipefail

source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/v0871_demo_common.sh"

ROOT_DIR="$(v0871_demo_repo_root)"
OUT_DIR="${1:-$ROOT_DIR/artifacts/v0871/restartability}"
PAUSED_RUN="v0-6-hitl-pause-demo"
COMPLETE_RUN="v0-6-hitl-no-pause-demo"
SUMMARY_FILE="$OUT_DIR/restartability_summary.json"

rm -rf "$OUT_DIR"
bash "$ROOT_DIR/adl/tools/demo_v0871_runtime_state.sh" "$OUT_DIR" >/dev/null

python3 - "$OUT_DIR/runtime/runs/$PAUSED_RUN/run_status.json" "$OUT_DIR/runtime/runs/$COMPLETE_RUN/run_status.json" "$SUMMARY_FILE" <<'PY'
import json
import sys

paused_path, complete_path, out_path = sys.argv[1:4]
with open(paused_path, encoding="utf-8") as fh:
    paused = json.load(fh)
with open(complete_path, encoding="utf-8") as fh:
    complete = json.load(fh)

payload = {
    "demo_id": "D5",
    "paused_run": {
        "run_id": paused.get("run_id"),
        "continuity_status": paused.get("continuity_status"),
        "persistence_mode": paused.get("persistence_mode"),
        "cleanup_disposition": paused.get("cleanup_disposition"),
        "resume_guard": paused.get("resume_guard"),
    },
    "completed_run": {
        "run_id": complete.get("run_id"),
        "continuity_status": complete.get("continuity_status"),
        "persistence_mode": complete.get("persistence_mode"),
        "cleanup_disposition": complete.get("cleanup_disposition"),
        "resume_guard": complete.get("resume_guard"),
    },
}

with open(out_path, "w", encoding="utf-8") as fh:
    json.dump(payload, fh, indent=2)
    fh.write("\n")
PY

SECONDARIES="$(printf '%s\n%s\n%s\n%s' \
  "$OUT_DIR/runtime/runs/$PAUSED_RUN/run_status.json" \
  "$OUT_DIR/runtime/runs/$PAUSED_RUN/pause_state.json" \
  "$OUT_DIR/runtime/runs/$COMPLETE_RUN/run_status.json" \
  "$OUT_DIR/runtime/runs/$COMPLETE_RUN/logs/trace_v1.json")"

v0871_demo_write_readme \
  "$OUT_DIR" \
  "v0.87.1 Demo D5 - Restartability And Recovery" \
  $'bash adl/tools/demo_v0871_restartability.sh' \
  "$SUMMARY_FILE" \
  "$SECONDARIES" \
  "restartability_summary.json compares the paused resume-ready path against the completed no-resume path using the runtime state contract"

v0871_demo_print_proof_surfaces "$SUMMARY_FILE" "$SECONDARIES"
