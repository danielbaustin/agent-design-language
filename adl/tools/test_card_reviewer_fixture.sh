#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE_DIR="$ROOT_DIR/docs/tooling/examples/reviewer-regression/issue-661"
EXPECTED="$FIXTURE_DIR/expected_review_output_661.yaml"

[[ -f "$FIXTURE_DIR/input_661.md" ]] || { echo "missing fixture input card"; exit 1; }
[[ -f "$FIXTURE_DIR/output_661.md" ]] || { echo "missing fixture output card"; exit 1; }
[[ -f "$EXPECTED" ]] || { echo "missing expected reviewer output"; exit 1; }

keys=(
  review_format_version:
  review_metadata:
  review_target:
  decision:
  summary:
  domain_results:
  findings:
  acceptance_criteria:
  determinism_checks:
  security_privacy_checks:
  artifact_checks:
  validation_checks:
  follow_ups:
)

prev=0
for key in "${keys[@]}"; do
  line="$(awk -v k="$key" '$0 ~ ("^" k "($|[[:space:]])") { print NR; exit }' "$EXPECTED")"
  [[ -n "$line" ]] || { echo "missing top-level key: ${key%:}"; exit 1; }
  if [[ "$line" -le "$prev" ]]; then
    echo "top-level key out of order: ${key%:}"
    exit 1
  fi
  prev="$line"
done

for state in contradicted not_evidenced not_applicable; do
  rg -n "evidence_state: $state" "$EXPECTED" >/dev/null || {
    echo "missing evidence_state: $state"
    exit 1
  }
done

rg -n "/Users/|/home/" "$EXPECTED" >/dev/null && {
  echo "absolute host path found in reviewer fixture output"
  exit 1
}

echo "ok"
