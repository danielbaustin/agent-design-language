#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
ARTIFACT_ROOT="$ROOT_DIR/artifacts/test_gemma4_issue_clerk"
VALID_FIXTURE="$ROOT_DIR/demos/fixtures/gemma4_issue_clerk_demo/valid_response.json"
INVALID_FIXTURE="$ROOT_DIR/demos/fixtures/gemma4_issue_clerk_demo/invalid_response.json"

rm -rf "$ARTIFACT_ROOT"
bash "$ROOT_DIR/adl/tools/demo_v089_gemma4_issue_clerk.sh" --artifact-root "$ARTIFACT_ROOT" --dry-run >/dev/null
python3 - "$ARTIFACT_ROOT/demo_manifest.json" <<'PY'
import json
import sys

with open(sys.argv[1], "r", encoding="utf-8") as fh:
    manifest = json.load(fh)
assert manifest["disposition"] == "dry_run_prepared"
assert manifest["artifacts"]["issue_packet"]
assert manifest["artifacts"]["model_prompt"]
PY

rm -rf "$ARTIFACT_ROOT"
ADL_GEMMA4_RESPONSE_FILE="$VALID_FIXTURE" \
  bash "$ROOT_DIR/adl/tools/demo_v089_gemma4_issue_clerk.sh" --artifact-root "$ARTIFACT_ROOT" >/dev/null
python3 - "$ARTIFACT_ROOT/demo_manifest.json" "$ARTIFACT_ROOT/materialized_issue_body.md" <<'PY'
import json
import sys

with open(sys.argv[1], "r", encoding="utf-8") as fh:
    manifest = json.load(fh)
assert manifest["disposition"] == "accepted"
assert manifest["artifacts"]["validated_issue_proposal"]

with open(sys.argv[2], "r", encoding="utf-8") as fh:
    body = fh.read()
assert "## Summary" in body
assert "release-surface consistency" in body
PY

rm -rf "$ARTIFACT_ROOT"
ADL_GEMMA4_RESPONSE_FILE="$INVALID_FIXTURE" \
  bash "$ROOT_DIR/adl/tools/demo_v089_gemma4_issue_clerk.sh" --artifact-root "$ARTIFACT_ROOT" >/dev/null
python3 - "$ARTIFACT_ROOT/demo_manifest.json" <<'PY'
import json
import sys

with open(sys.argv[1], "r", encoding="utf-8") as fh:
    manifest = json.load(fh)
assert manifest["disposition"] == "rejected"
assert manifest["artifacts"]["rejection_reason"]
assert manifest["artifacts"]["materialized_issue_body"] is None
PY

echo "demo_v089_gemma4_issue_clerk: ok"
