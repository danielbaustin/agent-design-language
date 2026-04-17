#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
PROFILE_DOC="$ROOT_DIR/docs/milestones/v0.90/milestone_compression/FINISH_VALIDATION_PROFILES_v0.90.md"
README_DOC="$ROOT_DIR/docs/milestones/v0.90/milestone_compression/README.md"
WORKFLOW_DOC="$ROOT_DIR/docs/default_workflow.md"

fail() {
  echo "FAIL: $1" >&2
  exit 1
}

require_text() {
  local needle="$1"
  local path="$2"
  grep -Fq "$needle" "$path" || fail "missing required text in ${path#$ROOT_DIR/}: $needle"
}

[[ -f "$PROFILE_DOC" ]] || fail "missing finish validation profile doc"

require_text "Focused local validation is not full local validation." "$PROFILE_DOC"
require_text "FOCUSED_LOCAL_CI_GATED" "$PROFILE_DOC"
require_text "CI requirement: required before merge" "$PROFILE_DOC"
require_text "full local validation: not run" "$PROFILE_DOC"
require_text "Use full local validation for runtime, schema, security, release, broad tooling" "$WORKFLOW_DOC"
require_text "FINISH_VALIDATION_PROFILES_v0.90.md" "$README_DOC"

if grep -Eiq 'focused local validation (is|equals|counts as) full local validation' "$PROFILE_DOC" "$README_DOC" "$WORKFLOW_DOC"; then
  fail "focused validation must not be described as full local validation"
fi

echo "compression finish profile checks: ok"
