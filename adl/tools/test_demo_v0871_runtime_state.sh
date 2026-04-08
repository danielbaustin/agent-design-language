#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TMP_DIR="$(mktemp -d "${TMPDIR:-/tmp}/adl-demo-v0871-runtime-state.XXXXXX")"
trap 'rm -rf "$TMP_DIR"' EXIT

bash "$ROOT_DIR/adl/tools/demo_v0871_runtime_state.sh" "$TMP_DIR/artifacts"

PAUSED_RUN="$TMP_DIR/artifacts/runtime/runs/v0-6-hitl-pause-demo"
COMPLETE_RUN="$TMP_DIR/artifacts/runtime/runs/v0-6-hitl-no-pause-demo"

test -f "$TMP_DIR/artifacts/README.md"
test -f "$TMP_DIR/artifacts/runtime/runtime_environment.json"
test -f "$PAUSED_RUN/run_status.json"
test -f "$PAUSED_RUN/pause_state.json"
test -f "$PAUSED_RUN/logs/trace_v1.json"
test -f "$COMPLETE_RUN/run_status.json"
test ! -f "$COMPLETE_RUN/pause_state.json"
test -f "$COMPLETE_RUN/logs/trace_v1.json"

grep -q '"persistence_mode": "checkpoint_resume_state"' "$PAUSED_RUN/run_status.json"
grep -q '"cleanup_disposition": "retain_pause_state"' "$PAUSED_RUN/run_status.json"
grep -q '"resume_guard": "execution_plan_hash_match_required"' "$PAUSED_RUN/run_status.json"
grep -q '"pause_state.json"' "$PAUSED_RUN/run_status.json"

grep -q '"persistence_mode": "completed_run_record"' "$COMPLETE_RUN/run_status.json"
grep -q '"cleanup_disposition": "no_resume_state_retained"' "$COMPLETE_RUN/run_status.json"
grep -q '"resume_guard": "not_applicable"' "$COMPLETE_RUN/run_status.json"

grep -q 'paused run status' "$TMP_DIR/artifacts/README.md"
grep -q 'completed run status' "$TMP_DIR/artifacts/README.md"
