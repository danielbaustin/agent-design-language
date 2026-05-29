#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TMPDIR_ROOT="$(mktemp -d)"
trap 'rm -rf "$TMPDIR_ROOT"' EXIT
OUT_DIR="$TMPDIR_ROOT/artifacts"

(
  cd "$ROOT_DIR"
  bash adl/tools/demo_v0914_multi_agent_repo_review_serious_proof.sh "$OUT_DIR" >/dev/null
)

for required in \
  "$OUT_DIR/run_manifest.json" \
  "$OUT_DIR/review_packet/repo_scope.md" \
  "$OUT_DIR/review_packet/heuristic_contract.json" \
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
    echo "assertion failed: missing artifact $required" >&2
    exit 1
  }
done

grep -Fq '"schema_version": "adl.v0914.multi_agent_repo_review_serious_proof.v1"' "$OUT_DIR/run_manifest.json" || {
  echo "assertion failed: run manifest schema mismatch" >&2
  exit 1
}

grep -Fq '"heuristics_visible": true' "$OUT_DIR/run_manifest.json" || {
  echo "assertion failed: heuristics visibility missing" >&2
  exit 1
}

grep -Fq 'No material findings.' "$OUT_DIR/specialist_reviews/security.md" || {
  echo "assertion failed: security no-finding statement missing" >&2
  exit 1
}

grep -Fq '[P2]' "$OUT_DIR/specialist_reviews/code.md" || {
  echo "assertion failed: code finding severity marker missing" >&2
  exit 1
}

grep -Fq -- '- Evidence:' "$OUT_DIR/specialist_reviews/code.md" || {
  echo "assertion failed: code finding evidence marker missing" >&2
  exit 1
}

grep -Fq -- '- Recommended Action:' "$OUT_DIR/specialist_reviews/code.md" || {
  echo "assertion failed: code finding recommended action marker missing" >&2
  exit 1
}

grep -Fq -- '- Residual Risk:' "$OUT_DIR/synthesis/final_findings_first_review.md" || {
  echo "assertion failed: synthesis residual risk marker missing" >&2
  exit 1
}

grep -Fq '## Role-Specific Caveats' "$OUT_DIR/specialist_reviews/code.md" || {
  echo "assertion failed: specialist caveats section missing" >&2
  exit 1
}

CUSTOM_DIR="$TMPDIR_ROOT/existing-custom-output"
mkdir -p "$CUSTOM_DIR"
printf 'keep\n' > "$CUSTOM_DIR/keep.txt"
if bash "$ROOT_DIR/adl/tools/demo_v0914_multi_agent_repo_review_serious_proof.sh" "$CUSTOM_DIR" >/dev/null 2>&1; then
  echo "assertion failed: demo accepted unsafe existing custom output directory" >&2
  exit 1
fi
[[ -f "$CUSTOM_DIR/keep.txt" ]] || {
  echo "assertion failed: unsafe custom output directory contents were removed" >&2
  exit 1
}

for leaked_path in \
  "/Users/alice/private.txt" \
  "/home/bob/private.txt" \
  "/private/tmp/demo-leak.txt"; do
  LEAK_DIR="$TMPDIR_ROOT/leak-check"
  rm -rf "$LEAK_DIR"
  cp -R "$OUT_DIR" "$LEAK_DIR"
  printf '\nInjected leak: %s\n' "$leaked_path" >> "$LEAK_DIR/specialist_reviews/code.md"
  if python3 "$ROOT_DIR/adl/tools/validate_v0914_multi_agent_repo_review_serious_proof.py" "$LEAK_DIR" >/dev/null 2>&1; then
    echo "assertion failed: validator accepted leaked path $leaked_path" >&2
    exit 1
  fi
done

echo "demo_v0914_multi_agent_repo_review_serious_proof: ok"
