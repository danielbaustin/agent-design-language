#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TMPDIR_ROOT="$(mktemp -d)"
trap 'rm -rf "$TMPDIR_ROOT"' EXIT
OUT_DIR="$TMPDIR_ROOT/artifacts"

(
  cd "$ROOT_DIR"
  bash adl/tools/demo_v089_multi_agent_repo_code_review.sh "$OUT_DIR" >/dev/null
)

for required in \
  "$OUT_DIR/demo_manifest.json" \
  "$OUT_DIR/README.md" \
  "$OUT_DIR/review_packet/review_packet_manifest.json" \
  "$OUT_DIR/review_packet/inventory_summary.json" \
  "$OUT_DIR/review_packet/selected_paths.txt" \
  "$OUT_DIR/reviewers/code_review.md" \
  "$OUT_DIR/reviewers/security_review.md" \
  "$OUT_DIR/reviewers/test_review.md" \
  "$OUT_DIR/reviewers/docs_review.md" \
  "$OUT_DIR/reviewers/cross_review_notes.md" \
  "$OUT_DIR/synthesis/final_synthesis_review.md"; do
  [[ -f "$required" ]] || {
    echo "assertion failed: missing artifact $required" >&2
    exit 1
  }
done

grep -Fq '"schema_version": "adl.v089.multi_agent_repo_review_demo.v1"' "$OUT_DIR/demo_manifest.json" || {
  echo "assertion failed: demo manifest schema mismatch" >&2
  exit 1
}

grep -Fq 'Blocking Findings: none.' "$OUT_DIR/synthesis/final_synthesis_review.md" || {
  echo "assertion failed: synthesis missing blocking findings classification" >&2
  exit 1
}

grep -Fq '[P3] Workflow conductor routing remains concentrated in one large dispatch surface' "$OUT_DIR/reviewers/code_review.md" || {
  echo "assertion failed: code reviewer finding missing" >&2
  exit 1
}

for leaked_path in \
  "/Users/alice/private.txt" \
  "/home/bob/private.txt" \
  "/private/tmp/demo-leak.txt"; do
  LEAK_DIR="$TMPDIR_ROOT/leak-check"
  rm -rf "$LEAK_DIR"
  cp -R "$OUT_DIR" "$LEAK_DIR"
  printf '\nInjected leak: %s\n' "$leaked_path" >> "$LEAK_DIR/reviewers/code_review.md"
  if python3 "$ROOT_DIR/adl/tools/validate_multi_agent_repo_review_demo.py" "$LEAK_DIR" >/dev/null 2>&1; then
    echo "assertion failed: validator accepted leaked path $leaked_path" >&2
    exit 1
  fi
done

echo "demo_v089_multi_agent_repo_code_review: ok"
