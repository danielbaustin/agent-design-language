#!/usr/bin/env bash
set -euo pipefail

source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/v0871_demo_common.sh"

ROOT_DIR="$(v0871_demo_repo_root)"
OUT_DIR="${1:-$ROOT_DIR/artifacts/v0871/lifecycle}"
RUN_ID="v0-4-demo-deterministic-replay"
STEP_OUT="$OUT_DIR/out"
LOG_FILE="$OUT_DIR/run_log.txt"
TRACE_FILE="$OUT_DIR/runtime/runs/$RUN_ID/logs/trace_v1.json"
SUMMARY_FILE="$OUT_DIR/lifecycle_summary.json"

rm -rf "$OUT_DIR"
v0871_demo_run_mock_workflow "$OUT_DIR" "adl/examples/v0-4-demo-deterministic-replay.adl.yaml" "$STEP_OUT" "$LOG_FILE"

python3 - "$TRACE_FILE" "$SUMMARY_FILE" <<'PY'
import json
import sys

trace_path, summary_path = sys.argv[1:3]
with open(trace_path, encoding="utf-8") as fh:
    trace = json.load(fh)

phases = []
boundaries = []
for event in trace.get("events", []):
    kind = event.get("event_type")
    decision = event.get("decision_context") or {}
    if kind == "LIFECYCLE_PHASE":
        phases.append(decision.get("outcome"))
    elif kind == "EXECUTION_BOUNDARY":
        boundaries.append({
            "boundary": (event.get("scope") or {}).get("name"),
            "outcome": decision.get("outcome"),
        })

payload = {
    "demo_id": "D2",
    "trace_path": trace_path,
    "phases": phases,
    "boundary_outcomes": boundaries,
    "phase_order_valid": phases == ["init", "execute", "complete", "teardown"],
}

with open(summary_path, "w", encoding="utf-8") as fh:
    json.dump(payload, fh, indent=2)
    fh.write("\n")
PY

SECONDARIES="$(printf '%s\n%s\n%s' \
  "$TRACE_FILE" \
  "$OUT_DIR/runtime/runs/$RUN_ID/run_summary.json" \
  "$LOG_FILE")"

v0871_demo_write_readme \
  "$OUT_DIR" \
  "v0.87.1 Demo D2 - Lifecycle Phases And Boundaries" \
  $'bash adl/tools/demo_v0871_lifecycle.sh' \
  "$SUMMARY_FILE" \
  "$SECONDARIES" \
  "lifecycle_summary.json records the explicit init -> execute -> complete -> teardown phase order from the emitted runtime trace"

v0871_demo_print_proof_surfaces "$SUMMARY_FILE" "$SECONDARIES"
