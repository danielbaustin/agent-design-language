#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TMPDIR_ROOT="$(mktemp -d)"
trap 'rm -rf "$TMPDIR_ROOT"' EXIT

OUT_DIR="$TMPDIR_ROOT/multi_agent_repo_review_proof"

(
  cd "$ROOT_DIR"
  bash adl/tools/demo_v0902_multi_agent_repo_review_proof.sh "$OUT_DIR" >/dev/null
  python3 adl/tools/validate_v0902_multi_agent_repo_review_proof.py "$OUT_DIR" >/dev/null
)

for required in \
  "$OUT_DIR/run_manifest.json" \
  "$OUT_DIR/review_packet/repo_scope.md" \
  "$OUT_DIR/review_packet/evidence_index.json" \
  "$OUT_DIR/review_packet/specialist_assignments.json" \
  "$OUT_DIR/specialist_reviews/code.md" \
  "$OUT_DIR/specialist_reviews/security.md" \
  "$OUT_DIR/specialist_reviews/tests.md" \
  "$OUT_DIR/specialist_reviews/docs.md" \
  "$OUT_DIR/synthesis/final_findings_first_review.md" \
  "$OUT_DIR/synthesis/coverage_matrix.json" \
  "$OUT_DIR/quality_gate/review_quality_evaluation.md" \
  "$OUT_DIR/quality_gate/redaction_and_publication_gate.md" \
  "$OUT_DIR/README.md"; do
  [[ -f "$required" ]] || {
    echo "assertion failed: missing $required" >&2
    exit 1
  }
done

python3 - "$OUT_DIR/run_manifest.json" "$OUT_DIR/synthesis/coverage_matrix.json" <<'PY'
import json
import sys
from pathlib import Path

manifest = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
coverage = json.loads(Path(sys.argv[2]).read_text(encoding="utf-8"))

assert manifest["schema_version"] == "adl.v0902.multi_agent_repo_review_proof.v1"
assert manifest["classification"] == "proving_fixture"
assert manifest["publication_allowed"] is False
assert manifest["merge_approval_claimed"] is False
assert manifest["live_provider_execution"] is False
assert coverage["required_roles_present"] is True
assert coverage["roles"]["security"]["findings"] == 0
assert coverage["roles"]["security"]["non_findings"] is True
PY

grep -Fq "Finding MR-CODE-001: [P2]" "$OUT_DIR/synthesis/final_findings_first_review.md" || {
  echo "assertion failed: synthesis missing code finding" >&2
  exit 1
}

grep -Fq "Security found no material issue" "$OUT_DIR/synthesis/final_findings_first_review.md" || {
  echo "assertion failed: synthesis missing explicit security non-finding" >&2
  exit 1
}

for leaked_text in \
  "/Users/alice/private.txt" \
  "/private/tmp/review-leak" \
  "OPENAI_API_KEY=secret"; do
  LEAK_DIR="$TMPDIR_ROOT/leak-check"
  rm -rf "$LEAK_DIR"
  cp -R "$OUT_DIR" "$LEAK_DIR"
  printf '\nInjected leak: %s\n' "$leaked_text" >> "$LEAK_DIR/synthesis/final_findings_first_review.md"
  if python3 "$ROOT_DIR/adl/tools/validate_v0902_multi_agent_repo_review_proof.py" "$LEAK_DIR" >/dev/null 2>&1; then
    echo "assertion failed: validator accepted leaked text $leaked_text" >&2
    exit 1
  fi
done

echo "demo_v0902_multi_agent_repo_review_proof: ok"
