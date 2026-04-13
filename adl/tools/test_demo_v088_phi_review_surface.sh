#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TMPDIR_ROOT="$(mktemp -d)"
trap 'rm -rf "$TMPDIR_ROOT"' EXIT
OUT_DIR="$TMPDIR_ROOT/artifacts"

(
  cd "$ROOT_DIR"
  bash adl/tools/demo_v088_phi_review_surface.sh "$OUT_DIR" >/dev/null
)

[[ -f "$OUT_DIR/demo_manifest.json" ]] || {
  echo "assertion failed: missing manifest" >&2
  exit 1
}
[[ -f "$OUT_DIR/state/phi_integration_metrics_v1.json" ]] || {
  echo "assertion failed: missing phi state artifact" >&2
  exit 1
}
grep -Fq '"demo_id": "D5"' "$OUT_DIR/demo_manifest.json" || {
  echo "assertion failed: manifest missing D5 row" >&2
  exit 1
}

echo "demo_v088_phi_review_surface: ok"
