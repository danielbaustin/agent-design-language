#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TMPDIR_ROOT="$(mktemp -d)"
trap 'rm -rf "$TMPDIR_ROOT"' EXIT

OUT_DIR="$TMPDIR_ROOT/artifacts"
RUN_ROOT="$OUT_DIR/runtime/runs/v0-89-gemini-provider-harmony-and-economics"

(
  cd "$ROOT_DIR"
  bash adl/tools/demo_v089_gemini_provider_harmony_and_economics.sh "$OUT_DIR" >/dev/null
)

for required in \
  "$OUT_DIR/demo_manifest.json" \
  "$OUT_DIR/README.md" \
  "$OUT_DIR/packet/review_packet.md" \
  "$OUT_DIR/packet/packet_manifest.json" \
  "$OUT_DIR/provider_selection/candidate_providers.json" \
  "$OUT_DIR/provider_selection/provider_selection_manifest.json" \
  "$OUT_DIR/provider_selection/capability_and_cost_reasoning.md" \
  "$OUT_DIR/review_artifacts/validated_review.json" \
  "$OUT_DIR/review_artifacts/gemini_artifact.md" \
  "$OUT_DIR/provider_setup/provider.adl.yaml" \
  "$RUN_ROOT/run_summary.json" \
  "$RUN_ROOT/steps.json" \
  "$RUN_ROOT/logs/trace_v1.json"; do
  [[ -f "$required" ]] || {
    echo "assertion failed: missing artifact $required" >&2
    exit 1
  }
done

python3 - "$OUT_DIR/demo_manifest.json" "$OUT_DIR/provider_selection/provider_selection_manifest.json" "$OUT_DIR/review_artifacts/validated_review.json" <<'PY'
import json
import sys
manifest = json.load(open(sys.argv[1], encoding="utf-8"))
selection = json.load(open(sys.argv[2], encoding="utf-8"))
review = json.load(open(sys.argv[3], encoding="utf-8"))
assert manifest["schema_version"] == "adl.gemini_provider_harmony_demo.v1"
assert manifest["selected_provider_family"] == "gemini"
assert selection["selected_provider_family"] == "gemini"
assert review["provider_family"] == "gemini"
assert review["execution_mode"] == "bounded_http_profile"
PY

grep -Fq 'economical' "$OUT_DIR/provider_selection/candidate_providers.json" || {
  echo "assertion failed: candidate provider surface missing cost-class language" >&2
  exit 1
}

grep -Fq 'welcomed bounded participant' "$OUT_DIR/README.md" || {
  echo "assertion failed: README missing expected framing" >&2
  exit 1
}

if grep -R -E '/Users/|/private/tmp|/tmp/' "$OUT_DIR" >/dev/null 2>&1; then
  echo "assertion failed: absolute path leaked into generated artifacts" >&2
  exit 1
fi

echo "demo_v089_gemini_provider_harmony_and_economics: ok"
