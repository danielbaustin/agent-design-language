#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TMP_DIR="$(mktemp -d "${TMPDIR:-/tmp}/adl-demo-v0871-runtime-rows.XXXXXX")"
trap 'rm -rf "$TMP_DIR"' EXIT

bash "$ROOT_DIR/adl/tools/demo_v0871_runtime_environment.sh" "$TMP_DIR/d1" >/dev/null
bash "$ROOT_DIR/adl/tools/demo_v0871_lifecycle.sh" "$TMP_DIR/d2" >/dev/null
bash "$ROOT_DIR/adl/tools/demo_v0871_trace_runtime.sh" "$TMP_DIR/d3" >/dev/null
bash "$ROOT_DIR/adl/tools/demo_v0871_resilience_failure.sh" "$TMP_DIR/d4" >/dev/null
bash "$ROOT_DIR/adl/tools/demo_v0871_shepherd_recovery.sh" "$TMP_DIR/d4a" >/dev/null
bash "$ROOT_DIR/adl/tools/demo_v0871_restartability.sh" "$TMP_DIR/d5" >/dev/null

test -f "$TMP_DIR/d1/runtime/runtime_environment.json"
grep -q '"schema_version": "runtime_environment.v1"' "$TMP_DIR/d1/runtime/runtime_environment.json"

test -f "$TMP_DIR/d2/lifecycle_summary.json"
grep -q '"phase_order_valid": true' "$TMP_DIR/d2/lifecycle_summary.json"

test -f "$TMP_DIR/d3/trace_bundle_manifest.json"
grep -q '"demo_id": "D3"' "$TMP_DIR/d3/trace_bundle_manifest.json"

test -f "$TMP_DIR/d4/failure_summary.json"
grep -q '"overall_status": "failed"' "$TMP_DIR/d4/failure_summary.json"
grep -q '"resilience_classification": "crash"' "$TMP_DIR/d4/failure_summary.json"

test -f "$TMP_DIR/d4a/shepherd_recovery_summary.json"
grep -q '"continuity_status": "resume_ready"' "$TMP_DIR/d4a/shepherd_recovery_summary.json"

test -f "$TMP_DIR/d5/restartability_summary.json"
grep -q '"resume_guard": "execution_plan_hash_match_required"' "$TMP_DIR/d5/restartability_summary.json"

echo "demo_v0871_runtime_rows: ok"
