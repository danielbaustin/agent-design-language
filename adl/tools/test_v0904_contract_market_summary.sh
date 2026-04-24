#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TMP_DIR="$(mktemp -d "$ROOT_DIR/.tmp-contract-market-summary.XXXXXX")"
RUNNER_OUT_REL="$(basename "$TMP_DIR")/runner"
RENDERED_ONE_REL="$(basename "$TMP_DIR")/rendered_one.md"
RENDERED_TWO_REL="$(basename "$TMP_DIR")/rendered_two.md"
TRAVERSAL_RENDER_REL="../summary-escape-attempt.md"
RUNNER_OUT="$ROOT_DIR/$RUNNER_OUT_REL"
RENDERED_ONE="$ROOT_DIR/$RENDERED_ONE_REL"
RENDERED_TWO="$ROOT_DIR/$RENDERED_TWO_REL"
ABSOLUTE_RENDER="/tmp/adl-contract-market-summary-absolute.md"
MISSING_RENDER_REL="$(basename "$TMP_DIR")/rendered_missing.md"
INVALID_REVIEW_REL="$(basename "$TMP_DIR")/invalid_review_bundle.json"
INVALID_RENDER_REL="$(basename "$TMP_DIR")/rendered_invalid.md"
RUNNER_OUT="$ROOT_DIR/$RUNNER_OUT_REL"
RENDERED_ONE="$ROOT_DIR/$RENDERED_ONE_REL"
RENDERED_TWO="$ROOT_DIR/$RENDERED_TWO_REL"
MISSING_RENDER="$ROOT_DIR/$MISSING_RENDER_REL"
INVALID_REVIEW="$ROOT_DIR/$INVALID_REVIEW_REL"
INVALID_RENDER="$ROOT_DIR/$INVALID_RENDER_REL"
trap 'rm -rf "$TMP_DIR"' EXIT

cd "$ROOT_DIR"

python3 adl/tools/run_v0904_contract_market_runner.py --out "$RUNNER_OUT_REL"
python3 adl/tools/render_v0904_contract_market_summary.py \
  --review-bundle "$RUNNER_OUT_REL/review_bundle.json" \
  --out "$RENDERED_ONE_REL"
python3 adl/tools/render_v0904_contract_market_summary.py \
  --review-bundle "$RUNNER_OUT_REL/review_bundle.json" \
  --out "$RENDERED_TWO_REL"

diff -u "$RENDERED_ONE" "$RENDERED_TWO"
diff -u "$RENDERED_ONE" "demos/fixtures/contract_market/review_summary_example.md"

python3 - "$RENDERED_ONE" <<'PY'
from pathlib import Path
import sys

text = Path(sys.argv[1]).read_text()

required_headings = [
    "## Scope",
    "## Participants",
    "## Authority Basis",
    "## Bid Comparison",
    "## Selection Rationale",
    "## Delegation",
    "## Artifacts",
    "## Trace",
    "## Validation",
    "## Tool Requirements",
    "## Caveats",
    "## Residual Risk",
]
for heading in required_headings:
    assert heading in text

assert "Proof:" in text
assert "Judgment:" in text
assert "Non-claims:" in text
assert "Residual risk:" in text
assert "Governed tool execution is deferred to v0.90.5." in text
assert "/Users/" not in text
assert "/private/" not in text
assert "/var/" not in text
assert "file://" not in text
PY

rm -f "$ABSOLUTE_RENDER"
set +e
ABSOLUTE_OUTPUT="$(
  python3 adl/tools/render_v0904_contract_market_summary.py \
    --review-bundle "$RUNNER_OUT_REL/review_bundle.json" \
    --out "$ABSOLUTE_RENDER" \
    2>&1
)"
ABSOLUTE_STATUS=$?
set -e

test "$ABSOLUTE_STATUS" -eq 1
[[ "$ABSOLUTE_OUTPUT" == *"contract_market_summary: fail [absolute_path_forbidden]"* ]]
[[ "$ABSOLUTE_OUTPUT" != *"Traceback (most recent call last)"* ]]
test ! -e "$ABSOLUTE_RENDER"

set +e
TRAVERSAL_OUTPUT="$(
  python3 adl/tools/render_v0904_contract_market_summary.py \
    --review-bundle "$RUNNER_OUT_REL/review_bundle.json" \
    --out "$TRAVERSAL_RENDER_REL" \
    2>&1
)"
TRAVERSAL_STATUS=$?
set -e

test "$TRAVERSAL_STATUS" -eq 1
[[ "$TRAVERSAL_OUTPUT" == *"contract_market_summary: fail [path_traversal_forbidden]"* ]]
[[ "$TRAVERSAL_OUTPUT" != *"Traceback (most recent call last)"* ]]

set +e
MISSING_OUTPUT="$(
  python3 adl/tools/render_v0904_contract_market_summary.py \
    --review-bundle "$(basename "$TMP_DIR")/does_not_exist.json" \
    --out "$MISSING_RENDER_REL" \
    2>&1
)"
MISSING_STATUS=$?
set -e

test "$MISSING_STATUS" -eq 1
[[ "$MISSING_OUTPUT" == *"contract_market_summary: fail [missing_input]"* ]]
[[ "$MISSING_OUTPUT" != *"Traceback (most recent call last)"* ]]
grep -F "render_failure: missing_input:" "$MISSING_RENDER"

printf '{ invalid json }\n' > "$INVALID_REVIEW"

set +e
INVALID_OUTPUT="$(
  python3 adl/tools/render_v0904_contract_market_summary.py \
    --review-bundle "$INVALID_REVIEW_REL" \
    --out "$INVALID_RENDER_REL" \
    2>&1
)"
INVALID_STATUS=$?
set -e

test "$INVALID_STATUS" -eq 1
[[ "$INVALID_OUTPUT" == *"contract_market_summary: fail [invalid_json]"* ]]
[[ "$INVALID_OUTPUT" != *"Traceback (most recent call last)"* ]]
grep -F "render_failure: invalid_json:" "$INVALID_RENDER"

echo "v0.90.4 contract-market summary smoke: pass"
