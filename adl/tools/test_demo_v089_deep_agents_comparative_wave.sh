#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TMPDIR_ROOT="$(mktemp -d)"
trap 'rm -rf "$TMPDIR_ROOT"' EXIT

OUT_DIR="$TMPDIR_ROOT/artifacts"
RUN_ROOT="$OUT_DIR/runtime/runs/v0-89-deep-agents-comparative-wave"

(
  cd "$ROOT_DIR"
  bash adl/tools/demo_v089_deep_agents_comparative_wave.sh "$OUT_DIR" >/dev/null
)

for required in \
  "$OUT_DIR/demo_manifest.json" \
  "$OUT_DIR/README.md" \
  "$OUT_DIR/comparative_wave/comparative_question.md" \
  "$OUT_DIR/comparative_wave/positions/chatgpt.md" \
  "$OUT_DIR/comparative_wave/positions/claude.md" \
  "$OUT_DIR/comparative_wave/positions/gemini.md" \
  "$OUT_DIR/comparative_wave/synthesis.md" \
  "$OUT_DIR/comparative_wave/differences_matrix.json" \
  "$OUT_DIR/comparative_wave/reviewer_brief.md" \
  "$RUN_ROOT/run_summary.json" \
  "$RUN_ROOT/steps.json" \
  "$RUN_ROOT/logs/trace_v1.json"; do
  [[ -f "$required" ]] || {
    echo "assertion failed: missing artifact $required" >&2
    exit 1
  }
done

python3 - "$OUT_DIR/demo_manifest.json" "$OUT_DIR/comparative_wave/differences_matrix.json" <<'PY'
import json
import sys
manifest = json.load(open(sys.argv[1], encoding="utf-8"))
matrix = json.load(open(sys.argv[2], encoding="utf-8"))
assert manifest["schema_version"] == "adl.deep_agents_comparative_wave_demo.v1"
assert manifest["demo_id"] == "v0.89.deep_agents_comparative_wave"
assert matrix["schema_version"] == "adl.deep_agents_wave_matrix.v1"
assert len(matrix["dimensions"]) >= 3
PY

grep -Fq 'Disagreement' "$OUT_DIR/comparative_wave/synthesis.md" || {
  echo "assertion failed: synthesis missing disagreement section" >&2
  exit 1
}

grep -Fq 'not a benchmark contest' "$OUT_DIR/comparative_wave/reviewer_brief.md" || {
  echo "assertion failed: reviewer brief missing bounded framing" >&2
  exit 1
}

if grep -R -E '/Users/|/private/tmp|/tmp/' "$OUT_DIR" >/dev/null 2>&1; then
  echo "assertion failed: absolute path leaked into generated artifacts" >&2
  exit 1
fi

echo "demo_v089_deep_agents_comparative_wave: ok"
