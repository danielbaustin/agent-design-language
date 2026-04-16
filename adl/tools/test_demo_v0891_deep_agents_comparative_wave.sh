#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TMPDIR_ROOT="$(mktemp -d)"
trap 'rm -rf "$TMPDIR_ROOT"' EXIT

OUT_DIR="$TMPDIR_ROOT/artifacts"
RUN_ROOT="$OUT_DIR/runtime/runs/v0-89-1-deep-agents-comparative-governance-wave"

(
  cd "$ROOT_DIR"
  bash adl/tools/demo_v0891_deep_agents_comparative_wave.sh "$OUT_DIR" >/dev/null
)

for required in \
  "$OUT_DIR/demo_manifest.json" \
  "$OUT_DIR/README.md" \
  "$OUT_DIR/comparative_wave/comparison_claims.md" \
  "$OUT_DIR/comparative_wave/reviewer_checklist.md" \
  "$OUT_DIR/comparative_wave/adl_surface_map.json" \
  "$OUT_DIR/comparative_wave/filesystem_surface_map.json" \
  "$OUT_DIR/comparative_wave/operator_visibility.md" \
  "$OUT_DIR/comparative_wave/governance_snapshot.json" \
  "$OUT_DIR/comparative_wave/public_positioning_summary.md" \
  "$OUT_DIR/comparative_wave/positions/chatgpt.md" \
  "$OUT_DIR/comparative_wave/positions/claude.md" \
  "$OUT_DIR/comparative_wave/positions/gemini.md" \
  "$OUT_DIR/comparative_wave/synthesis.md" \
  "$RUN_ROOT/run_summary.json" \
  "$RUN_ROOT/steps.json" \
  "$RUN_ROOT/logs/trace_v1.json"; do
  [[ -f "$required" ]] || {
    echo "assertion failed: missing artifact $required" >&2
    exit 1
  }
done

python3 - "$OUT_DIR/demo_manifest.json" "$OUT_DIR/comparative_wave/adl_surface_map.json" "$OUT_DIR/comparative_wave/filesystem_surface_map.json" "$OUT_DIR/comparative_wave/governance_snapshot.json" <<'PY'
import json
import sys

manifest = json.load(open(sys.argv[1], encoding="utf-8"))
adl_map = json.load(open(sys.argv[2], encoding="utf-8"))
filesystem_map = json.load(open(sys.argv[3], encoding="utf-8"))
governance = json.load(open(sys.argv[4], encoding="utf-8"))

assert manifest["schema_version"] == "adl.deep_agents_comparative_wave_follow_on_demo.v1"
assert manifest["demo_id"] == "v0.89.1.deep_agents_comparative_governance_wave"
assert adl_map["schema_version"] == "adl.deep_agents_surface_map.v1"
assert filesystem_map["schema_version"] == "adl.deep_agents_surface_map.v1"
assert governance["schema_version"] == "adl.deep_agents_governance_snapshot.v1"
assert "review surfaces are explicit" in governance["claims"]
PY

grep -Fq 'bounded `v0.88` comparative proof' "$OUT_DIR/comparative_wave/comparison_claims.md" || {
  echo "assertion failed: comparison claims missing predecessor acknowledgement" >&2
  exit 1
}

grep -Fq '## Governance Delta' "$OUT_DIR/comparative_wave/synthesis.md" || {
  echo "assertion failed: synthesis missing governance delta section" >&2
  exit 1
}

grep -Fq 'operator is part of a visible review contract' "$OUT_DIR/comparative_wave/operator_visibility.md" || {
  echo "assertion failed: operator visibility surface missing core claim" >&2
  exit 1
}

if grep -R -E '/Users/|/private/tmp|/tmp/' "$OUT_DIR" >/dev/null 2>&1; then
  echo "assertion failed: absolute path leaked into generated artifacts" >&2
  exit 1
fi

echo "demo_v0891_deep_agents_comparative_wave: ok"
