#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TMPDIR_ROOT="$(mktemp -d)"
trap 'rm -rf "$TMPDIR_ROOT"' EXIT
OUT_DIR="$TMPDIR_ROOT/artifacts"

(
  cd "$ROOT_DIR"
  bash adl/tools/demo_v089_proof_entrypoints.sh "$OUT_DIR" >/dev/null
)

for required in \
  "$OUT_DIR/demo_manifest.json" \
  "$OUT_DIR/README.md" \
  "$OUT_DIR/index.txt"; do
  [[ -f "$required" ]] || {
    echo "assertion failed: missing artifact $required" >&2
    exit 1
  }
done

grep -Fq '"schema_version": "adl.v089.proof_entrypoints.v1"' "$OUT_DIR/demo_manifest.json" || {
  echo "assertion failed: manifest schema mismatch" >&2
  exit 1
}
grep -Fq '"demo_id": "D1"' "$OUT_DIR/demo_manifest.json" || {
  echo "assertion failed: manifest missing D1 row" >&2
  exit 1
}
grep -Fq '"demo_id": "D7"' "$OUT_DIR/demo_manifest.json" || {
  echo "assertion failed: manifest missing D7 row" >&2
  exit 1
}
grep -Fq 'D7 docs/milestones/v0.89/features/SECURITY_AND_THREAT_MODELING.md' "$OUT_DIR/index.txt" || {
  echo "assertion failed: index missing D7 row" >&2
  exit 1
}
grep -Fq 'D1, D2, and D3 intentionally share one bounded runtime-control proof run' "$OUT_DIR/README.md" || {
  echo "assertion failed: README missing shared-proof guidance" >&2
  exit 1
}

echo "demo_v089_proof_entrypoints: ok"
