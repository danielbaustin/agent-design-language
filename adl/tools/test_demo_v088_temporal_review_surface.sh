#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TMPDIR_ROOT="$(mktemp -d)"
trap 'rm -rf "$TMPDIR_ROOT"' EXIT
OUT_DIR="$TMPDIR_ROOT/artifacts"

(
  cd "$ROOT_DIR"
  bash adl/tools/demo_v088_temporal_review_surface.sh "$OUT_DIR" >/dev/null
)

for required in \
  "$OUT_DIR/demo_manifest.json" \
  "$OUT_DIR/README.md" \
  "$OUT_DIR/state/chronosense_foundation_v1.json" \
  "$OUT_DIR/state/temporal_schema_v1.json" \
  "$OUT_DIR/state/continuity_semantics_v1.json" \
  "$OUT_DIR/state/temporal_query_retrieval_v1.json" \
  "$OUT_DIR/state/commitment_deadline_semantics_v1.json" \
  "$OUT_DIR/state/temporal_causality_explanation_v1.json" \
  "$OUT_DIR/state/execution_policy_cost_model_v1.json"; do
  [[ -f "$required" ]] || {
    echo "assertion failed: missing artifact $required" >&2
    exit 1
  }
done

grep -Fq '"demo_id": "D1"' "$OUT_DIR/demo_manifest.json" || {
  echo "assertion failed: manifest missing D1 row" >&2
  exit 1
}
grep -Fq '"demo_id": "D4"' "$OUT_DIR/demo_manifest.json" || {
  echo "assertion failed: manifest missing D4 row" >&2
  exit 1
}

echo "demo_v088_temporal_review_surface: ok"
