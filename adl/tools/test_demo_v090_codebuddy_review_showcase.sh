#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TMPDIR_ROOT="$(mktemp -d)"
trap 'rm -rf "$TMPDIR_ROOT"' EXIT
OUT_DIR="$TMPDIR_ROOT/codebuddy_review_showcase"

(
  cd "$ROOT_DIR"
  bash adl/tools/demo_v090_codebuddy_review_showcase.sh "$OUT_DIR" >/dev/null
  python3 adl/tools/validate_codebuddy_review_showcase_demo.py "$OUT_DIR" >/dev/null
)

for required in \
  "$OUT_DIR/run_manifest.json" \
  "$OUT_DIR/repo_scope.md" \
  "$OUT_DIR/repo_inventory.json" \
  "$OUT_DIR/specialist_reviews/code.md" \
  "$OUT_DIR/specialist_reviews/security.md" \
  "$OUT_DIR/specialist_reviews/tests.md" \
  "$OUT_DIR/specialist_reviews/docs.md" \
  "$OUT_DIR/specialist_reviews/architecture.md" \
  "$OUT_DIR/specialist_reviews/dependencies.md" \
  "$OUT_DIR/diagrams/system_map.mmd" \
  "$OUT_DIR/diagrams/diagram_manifest.md" \
  "$OUT_DIR/diagrams/diagram_review.md" \
  "$OUT_DIR/redaction_report.md" \
  "$OUT_DIR/test_recommendations/test_gap_report.md" \
  "$OUT_DIR/issue_planning/issue_candidates.md" \
  "$OUT_DIR/adr_candidates/adr_candidates.md" \
  "$OUT_DIR/fitness_functions/fitness_function_plan.md" \
  "$OUT_DIR/final_report.md" \
  "$OUT_DIR/quality_evaluation.md" \
  "$OUT_DIR/demo_operator_result.json"; do
  [[ -f "$required" ]] || {
    echo "assertion failed: missing artifact $required" >&2
    exit 1
  }
done

python3 - "$OUT_DIR/run_manifest.json" "$OUT_DIR/demo_operator_result.json" <<'PY'
import json
import sys
from pathlib import Path

manifest = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
result = json.loads(Path(sys.argv[2]).read_text(encoding="utf-8"))
assert manifest["schema_version"] == "codebuddy.review_showcase.v1"
assert manifest["classification"] == "non_proving"
assert result["classification"] == "non_proving"
lanes = {lane["skill"]: lane["status"] for lane in manifest["skill_lanes"]}
assert lanes["repo-packet-builder"] == "represented"
assert lanes["product-report-writer"] == "represented"
assert lanes["review-quality-evaluator"] == "staged_pending_2070"
assert manifest["publication_allowed"] is False
PY

grep -Fq "Finding CB-SEC-001: [P1]" "$OUT_DIR/final_report.md" || {
  echo "assertion failed: final report missing top P1 finding" >&2
  exit 1
}

grep -Fq "Staged pending #2070" "$OUT_DIR/quality_evaluation.md" || {
  echo "assertion failed: staged quality-evaluator truth missing" >&2
  exit 1
}

for leaked_text in \
  "/Users/alice/private.txt" \
  "/private/tmp/codebuddy-leak" \
  "OPENAI_API_KEY=secret"; do
  LEAK_DIR="$TMPDIR_ROOT/leak-check"
  rm -rf "$LEAK_DIR"
  cp -R "$OUT_DIR" "$LEAK_DIR"
  printf '\nInjected leak: %s\n' "$leaked_text" >> "$LEAK_DIR/final_report.md"
  if python3 "$ROOT_DIR/adl/tools/validate_codebuddy_review_showcase_demo.py" "$LEAK_DIR" >/dev/null 2>&1; then
    echo "assertion failed: validator accepted leaked text $leaked_text" >&2
    exit 1
  fi
done

echo "demo_v090_codebuddy_review_showcase: ok"
