#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TMPDIR_ROOT="$(mktemp -d)"
trap 'rm -rf "$TMPDIR_ROOT"' EXIT
OUT_DIR="$TMPDIR_ROOT/artifacts"

(
  cd "$ROOT_DIR"
  bash adl/tools/demo_v088_review_surface.sh "$OUT_DIR" >/dev/null
)

for required in \
  "$OUT_DIR/demo_manifest.json" \
  "$OUT_DIR/README.md" \
  "$OUT_DIR/index.txt" \
  "$OUT_DIR/temporal/demo_manifest.json" \
  "$OUT_DIR/phi/state/phi_integration_metrics_v1.json" \
  "$OUT_DIR/instinct/demo_manifest.json" \
  "$OUT_DIR/paper_sonata/demo_manifest.json" \
  "$OUT_DIR/deep_agents_comparative/comparative_manifest.json"; do
  [[ -f "$required" ]] || {
    echo "assertion failed: missing artifact $required" >&2
    exit 1
  }
done

grep -Fq '"schema_version": "adl.v088.review_surface.v1"' "$OUT_DIR/demo_manifest.json" || {
  echo "assertion failed: integrated manifest schema mismatch" >&2
  exit 1
}
grep -Fq 'D8 paper_sonata/demo_manifest.json' "$OUT_DIR/index.txt" || {
  echo "assertion failed: index missing D8 row" >&2
  exit 1
}

echo "demo_v088_review_surface: ok"
