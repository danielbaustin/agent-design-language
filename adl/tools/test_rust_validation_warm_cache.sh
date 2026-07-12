#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
HELPER="$ROOT_DIR/adl/tools/rust_validation_warm_cache.sh"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

assert_json_field() {
  local file="$1"
  local expr="$2"
  python3 - "$file" "$expr" <<'PY'
import json
import sys
payload = json.load(open(sys.argv[1]))
if not eval(sys.argv[2], {"payload": payload}):
    raise SystemExit(f"assertion failed: {sys.argv[2]} payload={payload}")
PY
}

missing_out="$TMP/missing.json"
ADL_RUST_WARM_CACHE_SOURCE_TARGET="$TMP/missing-target" \
ADL_RUST_WARM_CACHE_DEST_TARGET="$TMP/dest-target" \
ADL_RUST_WARM_CACHE_OUTPUT="$missing_out" \
  bash "$HELPER" >/dev/null
assert_json_field "$missing_out" 'payload["status"] == "skipped"'
assert_json_field "$missing_out" 'payload["reason"] == "source target cache missing"'
assert_json_field "$missing_out" 'payload["validation_proof"] is False'

same_target="$TMP/same-target"
mkdir -p "$same_target/debug/deps"
same_out="$TMP/same.json"
ADL_RUST_WARM_CACHE_SOURCE_TARGET="$same_target" \
ADL_RUST_WARM_CACHE_DEST_TARGET="$same_target" \
ADL_RUST_WARM_CACHE_OUTPUT="$same_out" \
  bash "$HELPER" >/dev/null
assert_json_field "$same_out" 'payload["status"] == "skipped"'
assert_json_field "$same_out" 'payload["reason"] == "source target is destination target"'

cross_device_dest_parent="${ADL_WARM_CACHE_CROSS_DEVICE_DEST_PARENT:-/Volumes/FastWork}"
if [ -d "$cross_device_dest_parent" ] && python3 - "$TMP" "$cross_device_dest_parent" <<'PY'
import os
import sys

source, dest = sys.argv[1], sys.argv[2]
raise SystemExit(0 if os.stat(source).st_dev != os.stat(dest).st_dev else 1)
PY
then
  cross_source="$TMP/cross-source"
  cross_dest="$cross_device_dest_parent/adl-warm-cache-cross-device-test"
  cross_out="$TMP/cross-device.json"
  mkdir -p "$cross_source/debug/deps" "$cross_dest"
  ADL_RUST_WARM_CACHE_SOURCE_TARGET="$cross_source" \
  ADL_RUST_WARM_CACHE_DEST_TARGET="$cross_dest" \
  ADL_RUST_WARM_CACHE_OUTPUT="$cross_out" \
    bash "$HELPER" >/dev/null
  assert_json_field "$cross_out" 'payload["status"] == "skipped"'
  assert_json_field "$cross_out" 'payload["reason"] == "source and destination target are on different filesystems; hardlink warm cache skipped"'
  rm -rf "$cross_dest"
fi

if ! grep -Fq '*/.worktrees/adl-wp-*)' "$HELPER"; then
  echo "expected helper to detect ADL issue worktrees for primary-checkout warm source discovery" >&2
  exit 1
fi

for runner in \
  "$ROOT_DIR/adl/tools/run_authoritative_coverage_lane.sh" \
  "$ROOT_DIR/adl/tools/run_pr_fast_coverage_lane.sh" \
  "$ROOT_DIR/adl/tools/run_pr_fast_test_lane.sh" \
  "$ROOT_DIR/adl/tools/run_owner_validation_lane.sh"
do
  if ! grep -Fq 'rust_validation_warm_cache.sh' "$runner"; then
    echo "expected runner to invoke shared warm-cache helper: $runner" >&2
    exit 1
  fi
done

owner_plan="$(bash "$ROOT_DIR/adl/tools/run_owner_validation_lane.sh" csdlc --print-plan)"
if grep -Fq '"status"' <<<"$owner_plan"; then
  echo "owner validation --print-plan must not execute warm-cache helper" >&2
  exit 1
fi

echo "PASS test_rust_validation_warm_cache"
