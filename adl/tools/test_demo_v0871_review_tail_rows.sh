#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TMP_DIR="$(mktemp -d "${TMPDIR:-/tmp}/adl-demo-v0871-review-rows.XXXXXX")"
trap 'rm -rf "$TMP_DIR"' EXIT

bash "$ROOT_DIR/adl/tools/demo_v0871_integrated_runtime.sh" "$TMP_DIR/d9" >/dev/null
bash "$ROOT_DIR/adl/tools/demo_v0871_docs_review.sh" "$TMP_DIR/d10" >/dev/null
bash "$ROOT_DIR/adl/tools/demo_v0871_quality_gate.sh" "$TMP_DIR/d11" >/dev/null
bash "$ROOT_DIR/adl/tools/demo_v0871_release_review_package.sh" "$TMP_DIR/d12" >/dev/null

test -f "$TMP_DIR/d9/demo_manifest.json"
grep -q '"demo_id": "D9"' "$TMP_DIR/d9/demo_manifest.json"
grep -q '"demo_id": "D8"' "$TMP_DIR/d9/demo_manifest.json"

test -f "$TMP_DIR/d10/docs_review_manifest.json"
grep -q '"demo_id": "D10"' "$TMP_DIR/d10/docs_review_manifest.json"
grep -q 'ADL_RUNTIME_ENVIRONMENT.md' "$TMP_DIR/d10/docs_review_manifest.json"

test -f "$TMP_DIR/d11/quality_gate_record.json"
grep -q '"demo_id":"D11"' "$TMP_DIR/d11/quality_gate_record.json"
grep -q '"fmt":{"status":"PASS"' "$TMP_DIR/d11/quality_gate_record.json"

test -f "$TMP_DIR/d12/release_review_package_manifest.json"
grep -q '"demo_id": "D12"' "$TMP_DIR/d12/release_review_package_manifest.json"
grep -q 'RELEASE_PLAN_v0.87.1.md' "$TMP_DIR/d12/release_review_package_manifest.json"

echo "demo_v0871_review_tail_rows: ok"
