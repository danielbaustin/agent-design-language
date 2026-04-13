#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TMPDIR_ROOT="$(mktemp -d)"
trap 'rm -rf "$TMPDIR_ROOT"' EXIT
OUT_DIR="$TMPDIR_ROOT/artifacts"

(
  cd "$ROOT_DIR"
  bash adl/tools/demo_v088_instinct_review_surface.sh "$OUT_DIR" >/dev/null
)

for required in \
  "$OUT_DIR/demo_manifest.json" \
  "$OUT_DIR/state/instinct_model_v1.json" \
  "$OUT_DIR/state/instinct_runtime_surface_v1.json"; do
  [[ -f "$required" ]] || {
    echo "assertion failed: missing artifact $required" >&2
    exit 1
  }
done

grep -Fq '"demo_id": "D6"' "$OUT_DIR/demo_manifest.json" || {
  echo "assertion failed: manifest missing D6 row" >&2
  exit 1
}
grep -Fq '"demo_id": "D7"' "$OUT_DIR/demo_manifest.json" || {
  echo "assertion failed: manifest missing D7 row" >&2
  exit 1
}

echo "demo_v088_instinct_review_surface: ok"
