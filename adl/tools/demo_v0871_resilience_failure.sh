#!/usr/bin/env bash
set -euo pipefail

source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/v0871_demo_common.sh"

ROOT_DIR="$(v0871_demo_repo_root)"
OUT_DIR="${1:-$ROOT_DIR/artifacts/v0871/resilience_failure}"
RUN_ID="failure-missing-file-demo"
STEP_OUT="$OUT_DIR/out"
LOG_FILE="$OUT_DIR/run_log.txt"
STATUS_FILE="$OUT_DIR/runtime/runs/$RUN_ID/run_status.json"
SUMMARY_FILE="$OUT_DIR/runtime/runs/$RUN_ID/run_summary.json"
TRACE_FILE="$OUT_DIR/runtime/runs/$RUN_ID/logs/trace_v1.json"
FAILURE_SUMMARY="$OUT_DIR/failure_summary.json"

rm -rf "$OUT_DIR"
v0871_demo_run_mock_workflow_expect_failure "$OUT_DIR" "adl/examples/failure-missing-file.adl.yaml" "$STEP_OUT" "$LOG_FILE"

python3 - "$STATUS_FILE" "$SUMMARY_FILE" "$FAILURE_SUMMARY" <<'PY'
import json
import sys

status_path, summary_path, out_path = sys.argv[1:4]
with open(status_path, encoding="utf-8") as fh:
    status = json.load(fh)
with open(summary_path, encoding="utf-8") as fh:
    summary = json.load(fh)

payload = {
    "demo_id": "D4",
    "run_id": status.get("run_id"),
    "overall_status": status.get("overall_status"),
    "failure_kind": status.get("failure_kind"),
    "resilience_classification": status.get("resilience_classification"),
    "continuity_status": status.get("continuity_status"),
    "shepherd_decision": status.get("shepherd_decision"),
    "summary_status": summary.get("status"),
}

with open(out_path, "w", encoding="utf-8") as fh:
    json.dump(payload, fh, indent=2)
    fh.write("\n")
PY

SECONDARIES="$(printf '%s\n%s\n%s' \
  "$STATUS_FILE" \
  "$SUMMARY_FILE" \
  "$TRACE_FILE")"

v0871_demo_write_readme \
  "$OUT_DIR" \
  "v0.87.1 Demo D4 - Local Failure Handling" \
  $'bash adl/tools/demo_v0871_resilience_failure.sh' \
  "$FAILURE_SUMMARY" \
  "$SECONDARIES" \
  "failure_summary.json records a bounded failed run that preserves reviewable state instead of silently discarding the failed execution"

v0871_demo_print_proof_surfaces "$FAILURE_SUMMARY" "$SECONDARIES"
