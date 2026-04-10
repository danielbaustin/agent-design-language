#!/usr/bin/env bash
set -euo pipefail

source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/v0871_demo_common.sh"

ROOT_DIR="$(v0871_demo_repo_root)"
OUT_DIR="${1:-$ROOT_DIR/artifacts/v0871/trace_runtime}"
RUN_ID="v0-4-demo-deterministic-replay"
STEP_OUT="$OUT_DIR/out"
LOG_FILE="$OUT_DIR/run_log.txt"
TRACE_FILE="$OUT_DIR/runtime/runs/$RUN_ID/logs/trace_v1.json"
SUMMARY_FILE="$OUT_DIR/runtime/runs/$RUN_ID/run_summary.json"
MANIFEST_FILE="$OUT_DIR/trace_bundle_manifest.json"

rm -rf "$OUT_DIR"
v0871_demo_run_mock_workflow "$OUT_DIR" "adl/examples/v0-4-demo-deterministic-replay.adl.yaml" "$STEP_OUT" "$LOG_FILE"
v0871_demo_archive_trace "$OUT_DIR" "$RUN_ID"

python3 - "$ROOT_DIR" "$OUT_DIR" "$RUN_ID" "$TRACE_FILE" "$SUMMARY_FILE" "$MANIFEST_FILE" <<'PY'
import json
import os
import sys

root_dir, out_dir, run_id, trace_file, summary_file, manifest_file = sys.argv[1:7]
archive_run = os.path.join(root_dir, ".adl", "trace-archive", "milestones", "v0.87.1", "runs", run_id)

with open(trace_file, encoding="utf-8") as fh:
    trace = json.load(fh)

payload = {
    "demo_id": "D3",
    "trace_bundle_version": "adl.v0871.trace_runtime.v1",
    "run_id": run_id,
    "trace_path": os.path.relpath(trace_file, root_dir),
    "run_summary_path": os.path.relpath(summary_file, root_dir),
    "archive_run_root": os.path.relpath(archive_run, root_dir),
    "event_types": [event.get("event_type") for event in trace.get("events", [])],
    "event_count": len(trace.get("events", [])),
}

with open(manifest_file, "w", encoding="utf-8") as fh:
    json.dump(payload, fh, indent=2)
    fh.write("\n")
PY

ARCHIVE_RUN=".adl/trace-archive/milestones/v0.87.1/runs/$RUN_ID"
SECONDARIES="$(printf '%s\n%s\n%s\n%s' \
  "$TRACE_FILE" \
  "$SUMMARY_FILE" \
  "$ARCHIVE_RUN/run_manifest.json" \
  "$ARCHIVE_RUN/logs/trace_v1.json")"

v0871_demo_write_readme \
  "$OUT_DIR" \
  "v0.87.1 Demo D3 - Trace-Aligned Runtime Execution" \
  $'bash adl/tools/demo_v0871_trace_runtime.sh' \
  "$MANIFEST_FILE" \
  "$SECONDARIES" \
  "trace_bundle_manifest.json links the emitted trace, run summary, and archived trace-bundle root for one bounded runtime run"

v0871_demo_print_proof_surfaces "$MANIFEST_FILE" "$SECONDARIES"
