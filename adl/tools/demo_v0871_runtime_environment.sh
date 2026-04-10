#!/usr/bin/env bash
set -euo pipefail

source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/v0871_demo_common.sh"

ROOT_DIR="$(v0871_demo_repo_root)"
OUT_DIR="${1:-$ROOT_DIR/artifacts/v0871/runtime_environment}"
RUN_ID="v0-4-demo-deterministic-replay"
STEP_OUT="$OUT_DIR/out"
LOG_FILE="$OUT_DIR/run_log.txt"
RUNTIME_ROOT="$OUT_DIR/runtime"
RUNS_ROOT="$RUNTIME_ROOT/runs"

rm -rf "$OUT_DIR"
v0871_demo_run_mock_workflow "$OUT_DIR" "adl/examples/v0-4-demo-deterministic-replay.adl.yaml" "$STEP_OUT" "$LOG_FILE"

SECONDARIES="$(printf '%s\n%s\n%s\n%s' \
  "$RUNS_ROOT/$RUN_ID/run_manifest.json" \
  "$RUNS_ROOT/$RUN_ID/run_summary.json" \
  "$RUNS_ROOT/$RUN_ID/run_status.json" \
  "$RUNS_ROOT/$RUN_ID/logs/trace_v1.json")"

v0871_demo_write_readme \
  "$OUT_DIR" \
  "v0.87.1 Demo D1 - Runtime Environment Bring-Up" \
  $'bash adl/tools/demo_v0871_runtime_environment.sh' \
  "$RUNTIME_ROOT/runtime_environment.json" \
  "$SECONDARIES" \
  "runtime_environment.json exists, the canonical run root is created, and the deterministic replay run emits its run manifest and summary under the declared runtime root"

v0871_demo_print_proof_surfaces \
  "$RUNTIME_ROOT/runtime_environment.json" \
  "$SECONDARIES"
