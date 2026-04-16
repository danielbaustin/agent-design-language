#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TMPDIR_ROOT="$(mktemp -d)"
trap 'rm -rf "$TMPDIR_ROOT"' EXIT

OUT_DIR="$TMPDIR_ROOT/artifacts"
RUN_ROOT="$OUT_DIR/runtime/runs/v0-89-1-five-agent-hey-jude-midi-demo"

(
  cd "$ROOT_DIR"
  bash adl/tools/demo_v0891_five_agent_hey_jude.sh "$OUT_DIR" >/dev/null
)

python3 "$ROOT_DIR/adl/tools/validate_five_agent_music_demo.py" "$OUT_DIR"

for required in \
  "$OUT_DIR/performance_manifest.json" \
  "$OUT_DIR/cast.json" \
  "$OUT_DIR/section_plan.json" \
  "$OUT_DIR/cue_timeline.json" \
  "$OUT_DIR/transcript.md" \
  "$OUT_DIR/performance_summary.md" \
  "$OUT_DIR/midi_binding.json" \
  "$OUT_DIR/midi_event_log.json" \
  "$OUT_DIR/midi_event_summary.json" \
  "$OUT_DIR/provider_participation_summary.json" \
  "$RUN_ROOT/run_summary.json" \
  "$RUN_ROOT/steps.json" \
  "$RUN_ROOT/logs/trace_v1.json"; do
  [[ -f "$required" ]] || {
    echo "assertion failed: missing artifact $required" >&2
    exit 1
  }
done

python3 - "$OUT_DIR/performance_manifest.json" "$OUT_DIR/midi_binding.json" "$OUT_DIR/midi_event_summary.json" <<'PY'
import json
import sys

manifest = json.load(open(sys.argv[1], encoding="utf-8"))
binding = json.load(open(sys.argv[2], encoding="utf-8"))
summary = json.load(open(sys.argv[3], encoding="utf-8"))

assert manifest["schema_version"] == "adl.five_agent_music_demo.v1"
assert manifest["demo_id"] == "v0.89.1.five_agent_hey_jude_midi_demo"
assert binding["profile_id"] == "mvave_chocolate"
assert summary["event_count"] >= 4
assert summary["actions"][-1] == "curtain_call"
PY

grep -Fq 'Layer 8' "$OUT_DIR/transcript.md" || {
  echo "assertion failed: transcript missing Layer 8" >&2
  exit 1
}

if grep -Fq 'Take a sad song and make it better' "$OUT_DIR/transcript.md"; then
  echo "assertion failed: copyrighted lyric line leaked into transcript" >&2
  exit 1
fi

echo "demo_v0891_five_agent_hey_jude: ok"
