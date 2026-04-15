#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TMPDIR_ROOT="$(mktemp -d)"
trap 'rm -rf "$TMPDIR_ROOT"' EXIT

OUT_DIR="$TMPDIR_ROOT/review_surface"

(
  cd "$ROOT_DIR"
  bash adl/tools/demo_v089_review_surface.sh "$OUT_DIR" >/dev/null
)

for required in \
  "$OUT_DIR/demo_manifest.json" \
  "$OUT_DIR/README.md" \
  "$OUT_DIR/index.txt" \
  "$OUT_DIR/claim_matrix.md" \
  "$OUT_DIR/proof_entrypoints/demo_manifest.json" \
  "$OUT_DIR/proof_entrypoints/README.md" \
  "$OUT_DIR/proof_entrypoints/index.txt"; do
  [[ -f "$required" ]] || {
    echo "assertion failed: missing artifact $required" >&2
    exit 1
  }
done

grep -Fq '"schema_version": "adl.v089.review_surface.v1"' "$OUT_DIR/demo_manifest.json" || {
  echo "assertion failed: review manifest schema mismatch" >&2
  exit 1
}
grep -Fq '"package_id": "core_runtime_control"' "$OUT_DIR/demo_manifest.json" || {
  echo "assertion failed: manifest missing core runtime-control package" >&2
  exit 1
}
grep -Fq '"package_id": "experiment_memory_security"' "$OUT_DIR/demo_manifest.json" || {
  echo "assertion failed: manifest missing experiment/memory/security package" >&2
  exit 1
}
grep -Fq '| D7 | security, posture, and trust-under-adversary main-band contract |' "$OUT_DIR/claim_matrix.md" || {
  echo "assertion failed: claim matrix missing D7 row" >&2
  exit 1
}
grep -Fq 'Review the integrated manifest first to understand the two bounded package families.' "$OUT_DIR/README.md" || {
  echo "assertion failed: README missing reviewer walkthrough guidance" >&2
  exit 1
}

echo "demo_v089_review_surface: ok"
