#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TMPDIR_ROOT="$(mktemp -d)"
trap 'rm -rf "$TMPDIR_ROOT"' EXIT

OUT_DIR="$TMPDIR_ROOT/artifacts"
RUN_ROOT="$OUT_DIR/runtime/runs/v0-89-gemini-in-the-loop"

(
  cd "$ROOT_DIR"
  bash adl/tools/demo_v089_gemini_in_the_loop.sh "$OUT_DIR" >/dev/null
)

for required in \
  "$OUT_DIR/demo_manifest.json" \
  "$OUT_DIR/README.md" \
  "$OUT_DIR/packet/review_packet.md" \
  "$OUT_DIR/provider_setup/provider.adl.yaml" \
  "$OUT_DIR/provider_setup/.env.example" \
  "$OUT_DIR/review_artifacts/validated_review.json" \
  "$OUT_DIR/review_artifacts/findings.md" \
  "$OUT_DIR/out/review/01-gemini-review.json" \
  "$RUN_ROOT/run_summary.json" \
  "$RUN_ROOT/steps.json" \
  "$RUN_ROOT/logs/trace_v1.json" \
  "$OUT_DIR/run_log.txt"; do
  [[ -f "$required" ]] || {
    echo "assertion failed: missing artifact $required" >&2
    exit 1
  }
done

python3 - "$OUT_DIR/demo_manifest.json" "$OUT_DIR/review_artifacts/validated_review.json" <<'PY'
import json
import sys
manifest = json.load(open(sys.argv[1], encoding="utf-8"))
review = json.load(open(sys.argv[2], encoding="utf-8"))
assert manifest["schema_version"] == "adl.gemini_in_the_loop_demo.v1"
assert manifest["provider_family"] == "gemini"
assert manifest["provider_profile"] == "http:gemini-2.0-flash"
assert review["packet_id"] == "gemini_review_packet.v1"
assert review["provider_family"] == "gemini"
assert len(review["findings"]) >= 1
PY

grep -Fq 'profile: "http:gemini-2.0-flash"' "$OUT_DIR/provider_setup/provider.adl.yaml" || {
  echo "assertion failed: provider setup missing gemini profile" >&2
  exit 1
}

grep -Fq '# Gemini Findings' "$OUT_DIR/review_artifacts/findings.md" || {
  echo "assertion failed: findings artifact missing heading" >&2
  exit 1
}

if grep -R -E '/Users/|/private/tmp|/tmp/' "$OUT_DIR" >/dev/null 2>&1; then
  echo "assertion failed: absolute path leaked into generated artifacts" >&2
  exit 1
fi

echo "demo_v089_gemini_in_the_loop: ok"
