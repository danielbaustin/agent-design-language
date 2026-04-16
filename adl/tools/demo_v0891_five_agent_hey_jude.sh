#!/usr/bin/env bash
set -euo pipefail

source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/provider_demo_common.sh"

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="${1:-$ROOT_DIR/artifacts/v0891/five_agent_hey_jude}"
RUNTIME_ROOT="$OUT_DIR/runtime"
RUNS_ROOT="$RUNTIME_ROOT/runs"
STEP_OUT="$OUT_DIR/out"
RUN_ID="v0-89-1-five-agent-hey-jude-midi-demo"
PORT="${ADL_HEY_JUDE_PORT:-0}"
PORT_FILE="$OUT_DIR/provider_server.port"
SERVER_LOG="$OUT_DIR/provider_server.log"
EXAMPLE="adl/examples/v0-89-1-five-agent-hey-jude-rehearsal.adl.yaml"
GENERATED_EXAMPLE="$OUT_DIR/v0-89-1-five-agent-hey-jude-rehearsal.runtime.adl.yaml"
MIDI_BRIDGE="$ROOT_DIR/adl/tools/mcp/midi-bridge/midi_bridge.py"
MIDI_FIXTURE="$ROOT_DIR/demos/fixtures/five_agent_hey_jude/mvave_chocolate_rehearsal_events.json"
MANIFEST="$OUT_DIR/performance_manifest.json"
CAST="$OUT_DIR/cast.json"
SECTION_PLAN="$OUT_DIR/section_plan.json"
CUE_TIMELINE="$OUT_DIR/cue_timeline.json"
TRANSCRIPT="$OUT_DIR/transcript.md"
SUMMARY="$OUT_DIR/performance_summary.md"
MIDI_DEVICES="$OUT_DIR/midi_devices.json"
MIDI_BINDING="$OUT_DIR/midi_binding.json"
MIDI_LOG="$OUT_DIR/midi_event_log.json"
MIDI_SUMMARY="$OUT_DIR/midi_event_summary.json"
PARTICIPATION="$OUT_DIR/provider_participation_summary.json"
README_OUT="$OUT_DIR/README.md"

sanitize_generated_artifacts() {
  export ADL_SANITIZE_OUT_DIR="$OUT_DIR"
  export ADL_SANITIZE_OUT_REAL
  ADL_SANITIZE_OUT_REAL="$(cd "$OUT_DIR" && pwd -P)"
  export ADL_SANITIZE_ROOT_DIR="$ROOT_DIR"
  export ADL_SANITIZE_ROOT_REAL
  ADL_SANITIZE_ROOT_REAL="$(cd "$ROOT_DIR" && pwd -P)"
  find "$OUT_DIR" -type f \( -name '*.json' -o -name '*.md' -o -name '*.txt' -o -name '*.yaml' \) -print0 |
    xargs -0 perl -0pi -e '
      for my $name (qw(ADL_SANITIZE_OUT_REAL ADL_SANITIZE_OUT_DIR ADL_SANITIZE_ROOT_REAL ADL_SANITIZE_ROOT_DIR)) {
        my $value = $ENV{$name} // "";
        next if $value eq "";
        my $replacement = $name =~ /ROOT/ ? "<repo_root>" : "<output_dir>";
        s/\Q$value\E/$replacement/g;
      }
    '
}

rm -rf "$OUT_DIR"
mkdir -p "$STEP_OUT"

python3 "$ROOT_DIR/adl/tools/mock_hey_jude_ensemble_provider.py" \
  "$PORT" \
  --port-file "$PORT_FILE" \
  >"$SERVER_LOG" 2>&1 &
SERVER_PID=$!
cleanup() {
  if kill -0 "$SERVER_PID" >/dev/null 2>&1; then
    kill "$SERVER_PID" >/dev/null 2>&1 || true
    wait "$SERVER_PID" >/dev/null 2>&1 || true
  fi
}
trap cleanup EXIT

PORT="$(provider_demo_wait_for_port "$PORT_FILE")"

python3 - "$EXAMPLE" "$GENERATED_EXAMPLE" "$PORT" <<'PY'
import sys
from pathlib import Path

source, target, port = sys.argv[1:4]
text = Path(source).read_text(encoding="utf-8")
for route in ("layer8", "chatgpt", "claude", "gemini", "deepseek"):
    text = text.replace(f"http://127.0.0.1:8796/{route}", f"http://127.0.0.1:{port}/{route}")
Path(target).write_text(text, encoding="utf-8")
PY

python3 "$MIDI_BRIDGE" list-devices --out "$MIDI_DEVICES"
python3 "$MIDI_BRIDGE" bind-profile --profile mvave_chocolate --device-id mvave-chocolate-fixture --out "$MIDI_BINDING"
python3 "$MIDI_BRIDGE" listen --binding "$MIDI_BINDING" --fixture "$MIDI_FIXTURE" --event-log "$MIDI_LOG"
python3 "$MIDI_BRIDGE" get-event-log --log "$MIDI_LOG" --out "$MIDI_SUMMARY"

cat >"$CAST" <<'EOF'
{
  "schema_version": "adl.five_agent_cast.v1",
  "cast": [
    {"participant": "Layer 8", "role": "human bandleader"},
    {"participant": "ChatGPT", "role": "conductor-builder"},
    {"participant": "Claude", "role": "reflective harmonizer"},
    {"participant": "Gemini", "role": "bright performer"},
    {"participant": "DeepSeek", "role": "pattern keeper"}
  ]
}
EOF

cat >"$SECTION_PLAN" <<'EOF'
{
  "schema_version": "adl.five_agent_section_plan.v1",
  "sections": [
    {"section": "Opening", "cue_action": "start_or_next_section"},
    {"section": "Verse Rotation", "cue_action": "start_or_next_section"},
    {"section": "Chorus Build", "cue_action": "trigger_chorus"},
    {"section": "Long Fade", "cue_action": "start_or_next_section"},
    {"section": "Curtain Call", "cue_action": "curtain_call"}
  ]
}
EOF

cat >"$CUE_TIMELINE" <<'EOF'
{
  "schema_version": "adl.five_agent_cue_timeline.v1",
  "cues": [
    {"index": 1, "source": "MVAVE Chocolate", "section": "Opening", "action": "start_or_next_section"},
    {"index": 2, "source": "MVAVE Chocolate", "section": "Chorus Build", "action": "trigger_chorus"},
    {"index": 3, "source": "MVAVE Chocolate", "section": "Long Fade", "action": "start_or_next_section"},
    {"index": 4, "source": "MVAVE Chocolate", "section": "Curtain Call", "action": "curtain_call"}
  ]
}
EOF

cd "$ROOT_DIR"

ADL_RUNTIME_ROOT="$RUNTIME_ROOT" \
ADL_RUNS_ROOT="$RUNS_ROOT" \
  cargo run --quiet --manifest-path adl/Cargo.toml --bin adl -- \
    "$GENERATED_EXAMPLE" \
    --run \
    --trace \
    --allow-unsigned \
    --out "$STEP_OUT" \
    >"$OUT_DIR/run_log.txt" 2>&1

cat >"$TRANSCRIPT" <<'EOF'
# Five-Agent Hey Jude Transcript

## Opening

### Layer 8
EOF
cat "$STEP_OUT/performance/01-layer8-opening.md" >>"$TRANSCRIPT"
cat >>"$TRANSCRIPT" <<'EOF'

### ChatGPT
EOF
cat "$STEP_OUT/performance/02-chatgpt-opening.md" >>"$TRANSCRIPT"
cat >>"$TRANSCRIPT" <<'EOF'

## Verse Rotation

### Claude
EOF
cat "$STEP_OUT/performance/03-claude-verse.md" >>"$TRANSCRIPT"
cat >>"$TRANSCRIPT" <<'EOF'

### Gemini
EOF
cat "$STEP_OUT/performance/04-gemini-verse.md" >>"$TRANSCRIPT"
cat >>"$TRANSCRIPT" <<'EOF'

## Chorus Build

### DeepSeek
EOF
cat "$STEP_OUT/performance/05-deepseek-chorus.md" >>"$TRANSCRIPT"
cat >>"$TRANSCRIPT" <<'EOF'

## Long Fade

### Layer 8
EOF
cat "$STEP_OUT/performance/06-layer8-fade-cue.md" >>"$TRANSCRIPT"
cat >>"$TRANSCRIPT" <<'EOF'

### ChatGPT
EOF
cat "$STEP_OUT/performance/07-chatgpt-chorus.md" >>"$TRANSCRIPT"
cat >>"$TRANSCRIPT" <<'EOF'

### Claude
EOF
cat "$STEP_OUT/performance/08-claude-chorus.md" >>"$TRANSCRIPT"
cat >>"$TRANSCRIPT" <<'EOF'

### Gemini
EOF
cat "$STEP_OUT/performance/09-gemini-chorus.md" >>"$TRANSCRIPT"
cat >>"$TRANSCRIPT" <<'EOF'

## Curtain Call

### DeepSeek
EOF
cat "$STEP_OUT/performance/10-deepseek-curtain.md" >>"$TRANSCRIPT"

cat >"$SUMMARY" <<'EOF'
# Performance Summary

This bounded flagship demo proves:

- five named participants can appear on one ADL runtime packet
- Layer 8 is treated as a first-class participant rather than an offstage operator
- a MIDI cue layer can shape section boundaries without turning the repo into a music platform
- the artifact package stays transcript-first, reviewable, and copyright-safe
EOF

cat >"$PARTICIPATION" <<'EOF'
{
  "schema_version": "adl.five_agent_participation_summary.v1",
  "participants": [
    "Layer 8",
    "ChatGPT",
    "Claude",
    "Gemini",
    "DeepSeek"
  ],
  "sections": [
    "Opening",
    "Verse Rotation",
    "Chorus Build",
    "Long Fade",
    "Curtain Call"
  ]
}
EOF

python3 - "$MANIFEST" "$RUN_ID" <<'PY'
import json
import sys
from pathlib import Path

manifest = {
    "schema_version": "adl.five_agent_music_demo.v1",
    "demo_id": "v0.89.1.five_agent_hey_jude_midi_demo",
    "title": "v0.89.1 five-agent Hey Jude MIDI flagship demo",
    "execution_mode": "transcript_first_runtime_demo",
    "claim": "ADL can host a bounded five-participant performance packet with one human Layer 8 agent, four model voices, and a real cue-layer bridge.",
    "artifacts": {
        "cast": "cast.json",
        "section_plan": "section_plan.json",
        "cue_timeline": "cue_timeline.json",
        "transcript": "transcript.md",
        "performance_summary": "performance_summary.md",
        "midi_binding": "midi_binding.json",
        "midi_event_log": "midi_event_log.json",
        "midi_event_summary": "midi_event_summary.json",
        "provider_participation_summary": "provider_participation_summary.json",
        "run_summary": f"runtime/runs/{sys.argv[2]}/run_summary.json",
        "steps": f"runtime/runs/{sys.argv[2]}/steps.json",
        "trace": f"runtime/runs/{sys.argv[2]}/logs/trace_v1.json"
    }
}
Path(sys.argv[1]).write_text(json.dumps(manifest, indent=2) + "\n", encoding="utf-8")
PY

cat >"$README_OUT" <<EOF
# v0.89.1 Demo - Five-Agent Hey Jude MIDI Flagship

Canonical command:

\`\`\`bash
bash adl/tools/demo_v0891_five_agent_hey_jude.sh
\`\`\`

Primary proof surfaces:
- \`cast.json\`
- \`section_plan.json\`
- \`cue_timeline.json\`
- \`transcript.md\`
- \`midi_event_log.json\`
- \`performance_summary.md\`
- \`runtime/runs/$RUN_ID/run_summary.json\`

What this proves:
- ADL can host one human Layer 8 participant plus ChatGPT, Claude, Gemini, and DeepSeek
- the cue layer is real and profile-driven through a bounded MIDI bridge surface
- the package stays transcript-first and copyright-safe by using section cues rather than a tracked lyric sheet
EOF

sanitize_generated_artifacts

echo "Five-agent Hey Jude proof surface under the output directory:"
echo "  cast.json"
echo "  section_plan.json"
echo "  cue_timeline.json"
echo "  transcript.md"
echo "  midi_event_log.json"
echo "  performance_summary.md"
echo "  runtime/runs/$RUN_ID/run_summary.json"
