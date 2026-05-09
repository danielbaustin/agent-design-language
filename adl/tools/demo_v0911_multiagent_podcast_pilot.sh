#!/usr/bin/env bash
set -euo pipefail

provider_demo_wait_for_port() {
  local port_file="$1"
  local attempts="${2:-100}"
  local sleep_secs="${3:-0.1}"
  local port=""
  local i
  for ((i = 0; i < attempts; i++)); do
    if [[ -s "$port_file" ]]; then
      port="$(<"$port_file")"
      if [[ "$port" =~ ^[0-9]+$ ]]; then
        printf '%s\n' "$port"
        return 0
      fi
    fi
    sleep "$sleep_secs"
  done
  echo "timed out waiting for demo server port in $port_file" >&2
  return 1
}

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="${1:-$ROOT_DIR/artifacts/v0911/multiagent_podcast_pilot}"
RUNTIME_ROOT="$OUT_DIR/runtime"
RUNS_ROOT="$RUNTIME_ROOT/runs"
STEP_OUT="$OUT_DIR/out"
RUN_ID="v0-91-1-multi-agent-podcast-pilot"
PORT="${ADL_PODCAST_PORT:-0}"
PORT_FILE="$OUT_DIR/provider_server.port"
SERVER_LOG="$OUT_DIR/provider_adapter.log"
INVOCATIONS="$OUT_DIR/provider_invocations.json"
TRANSCRIPT="$OUT_DIR/transcript.md"
PROOF_NOTE="$OUT_DIR/proof_note.md"
OBSERVATORY_PROJECTION="$OUT_DIR/observatory_projection.json"
MANIFEST="$OUT_DIR/demo_manifest.json"
EPISODE_CONTRACT="$OUT_DIR/episode_contract.json"
SERIES_MANIFEST="$OUT_DIR/series_manifest.json"
EPISODE_PACKET="$OUT_DIR/episode_packet.md"
BEST_LINES="$OUT_DIR/best_lines.md"
GENERATED_EXAMPLE="$OUT_DIR/v0-91-1-multi-agent-podcast-pilot.runtime.adl.yaml"
OPENAI_KEY_FILE="${ADL_OPENAI_KEY_FILE:-$HOME/keys/openai2.key}"
GEMINI_KEY_FILE="${ADL_GEMINI_KEY_FILE:-$HOME/keys/gcp-ace-2023.key}"
ANTHROPIC_KEY_FILE="${ADL_ANTHROPIC_KEY_FILE:-$HOME/keys/ADL_demo_ref_04.txt}"
OPENAI_MODEL="${ADL_LIVE_OPENAI_MODEL:-gpt-5.5}"
GEMINI_MODEL="${ADL_LIVE_GEMINI_MODEL:-gemini-3.1-pro-preview}"
ANTHROPIC_MODEL="${ADL_LIVE_ANTHROPIC_MODEL:-claude-opus-4-1-20250805}"
LIVE_PROVIDER_TIMEOUT_SECS="${ADL_LIVE_PROVIDER_TIMEOUT_SECS:-240}"
PODCAST_TOPIC="${ADL_PODCAST_TOPIC:-Should AI systems have consistent personalities across conversations?}"
SERIES_NAME="${ADL_PODCAST_SERIES_NAME:-ADL Multi-Agent Podcast}"
EPISODE_TITLE="${ADL_PODCAST_EPISODE_TITLE:-Episode 1: Should AI systems have consistent personalities across conversations?}"

load_key() {
  local env_name="$1"
  local key_file="$2"
  if [[ -n "${!env_name:-}" ]]; then
    return 0
  fi
  if [[ ! -s "$key_file" ]]; then
    echo "missing required key file for $env_name: $key_file" >&2
    return 1
  fi
  local key_value
  key_value="$(python3 - "$env_name" "$key_file" <<'PY'
import sys
env_name, path = sys.argv[1:3]
raw = open(path, encoding='utf-8').read().strip()
value = raw
for line in raw.splitlines():
    stripped = line.strip()
    if not stripped or stripped.startswith('#'):
        continue
    if stripped.startswith(env_name + '='):
        value = stripped.split('=', 1)[1].strip().strip("'\"")
        break
    value = stripped.strip("'\"")
    break
print(value, end='')
PY
)"
  if [[ -z "$key_value" ]]; then
    echo "empty required key file for $env_name: $key_file" >&2
    return 1
  fi
  export "$env_name=$key_value"
}

load_key OPENAI_API_KEY "$OPENAI_KEY_FILE"
load_key GEMINI_API_KEY "$GEMINI_KEY_FILE"
load_key ANTHROPIC_API_KEY "$ANTHROPIC_KEY_FILE"

rm -rf "$OUT_DIR"
mkdir -p "$STEP_OUT"
cp "$ROOT_DIR/adl/examples/v0-91-1-multi-agent-podcast-pilot.adl.yaml" "$GENERATED_EXAMPLE"

python3 "$ROOT_DIR/adl/tools/real_chatgpt_gemini_claude_provider_adapter.py" \
  --port "$PORT" \
  --port-file "$PORT_FILE" \
  --metadata "$INVOCATIONS" \
  --openai-model "$OPENAI_MODEL" \
  --gemini-model "$GEMINI_MODEL" \
  --anthropic-model "$ANTHROPIC_MODEL" \
  --timeout "$LIVE_PROVIDER_TIMEOUT_SECS" \
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

python3 - "$GENERATED_EXAMPLE" "$PORT" "$PODCAST_TOPIC" <<'PY'
import re
import sys
from pathlib import Path

path, port, topic = sys.argv[1:4]
text = Path(path).read_text(encoding='utf-8')
text = text.replace('__PODCAST_TOPIC__', topic)
text = re.sub(r'http://127\.0\.0\.1:8796/openai', f'http://127.0.0.1:{port}/openai', text)
text = re.sub(r'http://127\.0\.0\.1:8796/gemini', f'http://127.0.0.1:{port}/gemini', text)
text = re.sub(r'http://127\.0\.0\.1:8796/anthropic', f'http://127.0.0.1:{port}/anthropic', text)
Path(path).write_text(text, encoding='utf-8')
PY

python3 - "$PORT" <<'PY'
import json
import sys
import time
import urllib.request

port = int(sys.argv[1])
url = f'http://127.0.0.1:{port}/health'
deadline = time.time() + 10.0
last_error = None
while time.time() < deadline:
    try:
        with urllib.request.urlopen(url, timeout=1.0) as resp:
            payload = json.load(resp)
        if payload.get('ok') is True:
            raise SystemExit(0)
    except Exception as exc:
        last_error = exc
        time.sleep(0.1)
raise SystemExit(f'provider adapter failed health check: {last_error}')
PY

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

python3 - "$TRANSCRIPT" "$RUNS_ROOT/$RUN_ID/logs/trace_v1.json" "$STEP_OUT" "$OPENAI_MODEL" "$GEMINI_MODEL" "$ANTHROPIC_MODEL" "$PODCAST_TOPIC" "$SERIES_NAME" "$EPISODE_TITLE" "$EPISODE_CONTRACT" "$SERIES_MANIFEST" "$EPISODE_PACKET" "$BEST_LINES" "$OUT_DIR" <<'PY'
import json
import sys
from pathlib import Path

(
    transcript_path,
    trace_path,
    step_out,
    openai_model,
    gemini_model,
    anthropic_model,
    topic,
    series_name,
    episode_title,
    contract_path,
    manifest_path,
    packet_path,
    best_lines_path,
    out_dir,
) = sys.argv[1:15]

trace = json.loads(Path(trace_path).read_text(encoding='utf-8'))
events = trace.get('events', [])
step_end_timestamps = [event.get('timestamp') for event in events if event.get('event_type') == 'STEP_END']
turn_files = [
    ('01-chatgpt-opening.md', 'ChatGPT', 'Host opening'),
    ('02-gemini-challenge.md', 'Gemini', 'Challenge'),
    ('03-claude-reframe.md', 'Claude', 'Reframe'),
    ('04-chatgpt-bridge.md', 'ChatGPT', 'Bridge and synthesis'),
    ('05-gemini-deepening.md', 'Gemini', 'Deepening'),
    ('06-claude-closure.md', 'Claude', 'Closure'),
]
role_map = {
    'ChatGPT': 'host / synthesizer',
    'Gemini': 'challenger / systems analyst',
    'Claude': 'refiner / moral stylist',
}
voice_map = {
    'ChatGPT': 'warm, thoughtful, feminine, grounded',
    'Gemini': 'androgynous, bright, incisive',
    'Claude': 'masculine, reflective, slightly formal',
}
turns = []
for index, (filename, speaker, label) in enumerate(turn_files, start=1):
    body = (Path(step_out) / 'podcast' / filename).read_text(encoding='utf-8').strip()
    timestamp = step_end_timestamps[index - 1] if index - 1 < len(step_end_timestamps) else 'unknown'
    turns.append({
        'turn': index,
        'speaker': speaker,
        'label': label,
        'timestamp': timestamp,
        'body': body,
    })

transcript_lines = [
    f'# {series_name}',
    '',
    f'## {episode_title}',
    '',
    '> A bounded six-turn three-provider transcript-first episode run through the live ADL runtime.',
    '',
    '## Original Question',
    '',
    f'**{topic}**',
    '',
    '## Episode Conditions',
    '',
    '- Stop after six explicit turns total.',
    '- All three participants must appear in the same saved exchange.',
    '- Stable participant roles remain explicit and attributable.',
    '- Transcript-first proof remains primary; this pilot does not depend on audio.',
    '',
    '## Participants',
    '',
    f"- `ChatGPT`: {role_map['ChatGPT']}",
    f"- `Gemini`: {role_map['Gemini']}",
    f"- `Claude`: {role_map['Claude']}",
    '',
    '## Providers',
    '',
    f'- `ChatGPT`: `{openai_model}`',
    f'- `Gemini`: `{gemini_model}`',
    f'- `Claude`: `{anthropic_model}`',
    '',
    '## Transcript',
    '',
]
for turn in turns:
    transcript_lines.extend([
        f"### Turn {turn['turn']} · {turn['speaker']}",
        '',
        f"- Role: {role_map[turn['speaker']]}",
        f"- Label: {turn['label']}",
        f"- Timestamp: `{turn['timestamp']}`",
        '',
        turn['body'],
        '',
    ])
Path(transcript_path).write_text('\n'.join(transcript_lines).rstrip() + '\n', encoding='utf-8')

contract = {
    'schema_version': 'adl.demo.multiagent_podcast_episode_contract.v1',
    'series_name': series_name,
    'episode_title': episode_title,
    'topic': topic,
    'format': 'transcript_first_roundtable',
    'stop_rule': {'type': 'fixed_turn_count', 'turns': 6},
    'participants': {
        speaker.lower(): {
            'role': role_map[speaker],
            'voice_casting': voice_map[speaker],
        }
        for speaker in role_map
    },
    'proof_expectations': {
        'required_artifacts': [
            'transcript.md',
            'proof_note.md',
            'provider_invocations.json',
            'episode_contract.json',
        ],
        'transcript_first': True,
        'audio_required': False,
        'identity_continuity_claim_allowed': False,
    },
    'non_goals': [
        'broad_media_platform',
        'native_audio_for_all_providers',
        'long_term_identity_proof',
        'autonomous_federation_claims',
    ],
}
series_manifest = {
    'schema_version': 'adl.demo.multiagent_podcast_series_manifest.v1',
    'series_name': series_name,
    'pilot_episode_title': episode_title,
    'episode_count': 1,
    'default_roles': {
        speaker.lower(): role_map[speaker] for speaker in role_map
    },
}
Path(contract_path).write_text(json.dumps(contract, indent=2) + '\n', encoding='utf-8')
Path(manifest_path).write_text(json.dumps(series_manifest, indent=2) + '\n', encoding='utf-8')

best_lines = ['# Best Lines', '']
for turn in turns:
    first_line = next((line.strip() for line in turn['body'].splitlines() if line.strip()), '')
    best_lines.append(f"- `{turn['speaker']}`: {first_line}")
Path(best_lines_path).write_text('\n'.join(best_lines).rstrip() + '\n', encoding='utf-8')

packet = [
    f'# {episode_title} Packet',
    '',
    f'- Series: `{series_name}`',
    f'- Topic: `{topic}`',
    '- Format: `transcript_first_roundtable`',
    '- Stop rule: `6 turns total`',
    '',
    '## Included Artifacts',
    '',
    '- `transcript.md`',
    '- `proof_note.md`',
    '- `provider_invocations.json`',
    '- `episode_contract.json`',
    '- `series_manifest.json`',
    '- `best_lines.md`',
    '',
    '## Stable Roles',
    '',
    f"- `ChatGPT`: {role_map['ChatGPT']}",
    f"- `Gemini`: {role_map['Gemini']}",
    f"- `Claude`: {role_map['Claude']}",
    '',
    '## Voice Casting Recommendation',
    '',
    f"- `ChatGPT`: {voice_map['ChatGPT']}",
    f"- `Gemini`: {voice_map['Gemini']}",
    f"- `Claude`: {voice_map['Claude']}",
    '',
    '## Proof Boundary',
    '',
    '- This pilot proves one reusable transcript-first episode shape.',
    '- It does not prove persistent identity continuity, native audio availability, or a broad media product.',
]
Path(packet_path).write_text('\n'.join(packet).rstrip() + '\n', encoding='utf-8')

readme = [
    f'# {episode_title}',
    '',
    'Canonical command:',
    '',
    '```bash',
    'bash adl/tools/demo_v0911_multiagent_podcast_pilot.sh',
    '```',
    '',
    'Primary proof surfaces:',
    '- `transcript.md`',
    '- `proof_note.md`',
    '- `episode_contract.json`',
    '- `best_lines.md`',
    '',
    'Secondary proof surfaces:',
    '- `provider_invocations.json`',
    '- `observatory_projection.json`',
    '- `series_manifest.json`',
    '- `episode_packet.md`',
    '',
    'Success signal:',
    '- one six-turn episode packet exists with explicit roles, a saved transcript, and an honest proof note',
]
Path(out_dir, 'README.md').write_text('\n'.join(readme).rstrip() + '\n', encoding='utf-8')
PY

python3 - "$OBSERVATORY_PROJECTION" "$INVOCATIONS" "$RUNS_ROOT/$RUN_ID/logs/trace_v1.json" <<'PY'
import json
import sys
from pathlib import Path

projection_path, invocations_path, trace_path = sys.argv[1:4]
invocations = json.loads(Path(invocations_path).read_text(encoding='utf-8'))
trace = json.loads(Path(trace_path).read_text(encoding='utf-8'))
events = trace.get('events', [])
payload = {
    'schema': 'adl.demo.observatory_projection.v1',
    'demo_id': 'v0.91.1.multiagent_podcast_pilot',
    'view_kind': 'bounded_agent_runtime_projection',
    'providers': invocations.get('providers', []),
    'turns': [
        {'turn': 1, 'speaker': 'ChatGPT', 'artifact_ref': 'out/podcast/01-chatgpt-opening.md'},
        {'turn': 2, 'speaker': 'Gemini', 'artifact_ref': 'out/podcast/02-gemini-challenge.md'},
        {'turn': 3, 'speaker': 'Claude', 'artifact_ref': 'out/podcast/03-claude-reframe.md'},
        {'turn': 4, 'speaker': 'ChatGPT', 'artifact_ref': 'out/podcast/04-chatgpt-bridge.md'},
        {'turn': 5, 'speaker': 'Gemini', 'artifact_ref': 'out/podcast/05-gemini-deepening.md'},
        {'turn': 6, 'speaker': 'Claude', 'artifact_ref': 'out/podcast/06-claude-closure.md'},
    ],
    'trace_event_count': len(events),
}
Path(projection_path).write_text(json.dumps(payload, indent=2) + '\n', encoding='utf-8')
PY

python3 - "$MANIFEST" "$TRANSCRIPT" "$PROOF_NOTE" "$OBSERVATORY_PROJECTION" "$EPISODE_CONTRACT" "$BEST_LINES" <<'PY'
import json
import sys
from pathlib import Path

manifest_path, transcript_path, proof_note_path, projection_path, contract_path, best_lines_path = sys.argv[1:7]
payload = {
    'schema_version': 'adl.demo.manifest.v1',
    'demo_id': 'v0.91.1.multiagent_podcast_pilot',
    'primary_artifacts': [
        Path(transcript_path).name,
        Path(proof_note_path).name,
        Path(contract_path).name,
        Path(best_lines_path).name,
    ],
    'secondary_artifacts': [
        Path(projection_path).name,
        'provider_invocations.json',
        'series_manifest.json',
        'episode_packet.md',
    ],
}
Path(manifest_path).write_text(json.dumps(payload, indent=2) + '\n', encoding='utf-8')
PY

cat > "$PROOF_NOTE" <<EOF2
# Multi-Agent Podcast Pilot Proof Note

## What This Pilot Proves

- one reusable transcript-first podcast episode shape exists
- three named participants can keep stable explicit roles in one saved episode
- the packet preserves transcript, proof note, role contract, and reviewable outputs

## What This Pilot Does Not Prove

- long-term identity continuity across episodes
- native audio support from every provider
- a broad always-on media platform
- autonomous multi-agent federation or society

## Boundaries

- topic: \
$PODCAST_TOPIC\

- stop rule: six explicit turns total
- audio: optional and not required for the pilot
- proof priority: transcript and role attribution over entertainment value
EOF2

echo "Podcast pilot proof surfaces:"
echo "  $TRANSCRIPT"
echo "  $PROOF_NOTE"
echo "  $EPISODE_CONTRACT"
echo "  $BEST_LINES"
echo "  $SERIES_MANIFEST"
