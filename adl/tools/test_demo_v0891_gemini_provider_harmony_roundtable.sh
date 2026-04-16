#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TMPDIR_ROOT="$(mktemp -d)"
trap 'rm -rf "$TMPDIR_ROOT"' EXIT

OUT_DIR="$TMPDIR_ROOT/artifacts"
RUN_ROOT="$OUT_DIR/runtime/runs/v0-89-1-gemini-provider-harmony-roundtable"

(
  cd "$ROOT_DIR"
  bash adl/tools/demo_v0891_gemini_provider_harmony_roundtable.sh "$OUT_DIR" >/dev/null
)

for required in \
  "$OUT_DIR/demo_manifest.json" \
  "$OUT_DIR/README.md" \
  "$OUT_DIR/packet/topic.md" \
  "$OUT_DIR/packet/packet_manifest.json" \
  "$OUT_DIR/provider_selection/candidate_providers.json" \
  "$OUT_DIR/provider_selection/provider_selection_manifest.json" \
  "$OUT_DIR/provider_selection/provider_fit_scorecard.json" \
  "$OUT_DIR/provider_selection/capability_and_cost_reasoning.md" \
  "$OUT_DIR/roundtable/01-gemini-opening.md" \
  "$OUT_DIR/roundtable/02-chatgpt-response.md" \
  "$OUT_DIR/roundtable/03-claude-response.md" \
  "$OUT_DIR/roundtable/synthesis.md" \
  "$OUT_DIR/roundtable/provider_participation_summary.json" \
  "$OUT_DIR/provider_setup/provider.adl.yaml" \
  "$RUN_ROOT/run_summary.json" \
  "$RUN_ROOT/steps.json" \
  "$RUN_ROOT/logs/trace_v1.json"; do
  [[ -f "$required" ]] || {
    echo "assertion failed: missing artifact $required" >&2
    exit 1
  }
done

python3 - "$OUT_DIR/demo_manifest.json" "$OUT_DIR/provider_selection/provider_selection_manifest.json" "$OUT_DIR/provider_selection/provider_fit_scorecard.json" <<'PY'
import json
import sys
manifest = json.load(open(sys.argv[1], encoding="utf-8"))
selection = json.load(open(sys.argv[2], encoding="utf-8"))
scorecard = json.load(open(sys.argv[3], encoding="utf-8"))
assert manifest["schema_version"] == "adl.gemini_provider_harmony_follow_on_demo.v1"
assert manifest["selected_provider_family"] == "gemini"
assert selection["selected_provider_family"] == "gemini"
selected_rows = [row for row in scorecard["rows"] if row["selected"]]
assert len(selected_rows) == 1 and selected_rows[0]["provider_family"] == "gemini"
PY

grep -Fq 'economical' "$OUT_DIR/provider_selection/candidate_providers.json" || {
  echo "assertion failed: candidate providers missing cost-class rationale" >&2
  exit 1
}

grep -Fq '## Acknowledgement' "$OUT_DIR/roundtable/synthesis.md" || {
  echo "assertion failed: synthesis missing acknowledgement section" >&2
  exit 1
}

grep -Fq 'first-class' "$OUT_DIR/README.md" || {
  echo "assertion failed: README missing expected participant framing" >&2
  exit 1
}

if grep -R -E '/Users/|/private/tmp|/tmp/' "$OUT_DIR" >/dev/null 2>&1; then
  echo "assertion failed: absolute path leaked into generated artifacts" >&2
  exit 1
fi

echo "demo_v0891_gemini_provider_harmony_roundtable: ok"
