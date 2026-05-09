#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="${1:-$ROOT_DIR/artifacts/v0911/multiagent_podcast_pilot_audio}"
SOURCE_DIR="${ADL_PODCAST_AUDIO_SOURCE_DIR:-$OUT_DIR/source_episode}"
SEGMENTS_DIR="$OUT_DIR/audio/segments"
AUDIO_MANIFEST="$OUT_DIR/audio_manifest.json"
AUDIO_PACKET="$OUT_DIR/audio_packet.md"
EPISODE_AUDIO="$OUT_DIR/episode.wav"
OPENAI_KEY_FILE="${ADL_OPENAI_KEY_FILE:-$HOME/keys/openai2.key}"
GEMINI_KEY_FILE="${ADL_GEMINI_KEY_FILE:-$HOME/keys/gcp-ace-2023.key}"
OPENAI_TTS_MODEL="${ADL_PODCAST_OPENAI_TTS_MODEL:-gpt-4o-mini-tts}"
GEMINI_TTS_MODEL="${ADL_PODCAST_GEMINI_TTS_MODEL:-gemini-2.5-flash-preview-tts}"
CHATGPT_VOICE="${ADL_PODCAST_CHATGPT_VOICE:-coral}"
GEMINI_VOICE="${ADL_PODCAST_GEMINI_VOICE:-Kore}"
GEMINI_AUDIO_PROVIDER="${ADL_PODCAST_GEMINI_AUDIO_PROVIDER:-gemini}"
GEMINI_OPENAI_SURROGATE_VOICE="${ADL_PODCAST_GEMINI_OPENAI_SURROGATE_VOICE:-alloy}"
CLAUDE_SURROGATE_PROVIDER="${ADL_PODCAST_CLAUDE_SURROGATE_PROVIDER:-openai}"
CLAUDE_SURROGATE_VOICE="${ADL_PODCAST_CLAUDE_SURROGATE_VOICE:-alloy}"

if [[ "$GEMINI_AUDIO_PROVIDER" == "openai" ]]; then
  GEMINI_RENDER_VOICE="$GEMINI_OPENAI_SURROGATE_VOICE"
  GEMINI_SURROGATE="true"
else
  GEMINI_RENDER_VOICE="$GEMINI_VOICE"
  GEMINI_SURROGATE="false"
fi

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

render_openai_wav() {
  local text="$1"
  local voice="$2"
  local instructions="$3"
  local out_path="$4"
  local payload
  payload="$(python3 - "$text" "$voice" "$instructions" "$OPENAI_TTS_MODEL" <<'PY'
import json, sys
text, voice, instructions, model = sys.argv[1:5]
print(json.dumps({
  'model': model,
  'voice': voice,
  'input': text,
  'instructions': instructions,
  'response_format': 'wav',
}))
PY
)"
  curl -sS https://api.openai.com/v1/audio/speech \
    --connect-timeout 10 \
    --max-time 120 \
    -H "Authorization: Bearer $OPENAI_API_KEY" \
    -H "Content-Type: application/json" \
    -d "$payload" \
    --output "$out_path"
}

render_gemini_wav() {
  local text="$1"
  local voice="$2"
  local out_path="$3"
  local response_json="$out_path.response.json"
  local payload
  payload="$(python3 - "$text" "$voice" "$GEMINI_TTS_MODEL" <<'PY'
import json, sys
text, voice, model = sys.argv[1:4]
print(json.dumps({
  'contents': [{'parts': [{'text': text}]}],
  'generationConfig': {
    'responseModalities': ['AUDIO'],
    'speechConfig': {
      'voiceConfig': {
        'prebuiltVoiceConfig': {
          'voiceName': voice,
        }
      }
    }
  },
  'model': model,
}))
PY
)"
  curl -sS "https://generativelanguage.googleapis.com/v1beta/models/${GEMINI_TTS_MODEL}:generateContent" \
    --connect-timeout 10 \
    --max-time 120 \
    -H "x-goog-api-key: $GEMINI_API_KEY" \
    -H "Content-Type: application/json" \
    -d "$payload" \
    > "$response_json"
  python3 - "$response_json" "$out_path" <<'PY'
import base64
import json
import math
import sys
import sys
import wave
from pathlib import Path

response_path, out_path = sys.argv[1:3]
payload = json.loads(Path(response_path).read_text(encoding='utf-8'))
data = payload['candidates'][0]['content']['parts'][0]['inlineData']['data']
pcm = base64.b64decode(data)
with wave.open(out_path, 'wb') as wf:
    wf.setnchannels(1)
    wf.setsampwidth(2)
    wf.setframerate(24000)
    wf.writeframes(pcm)
PY
  rm -f "$response_json"
}

load_key OPENAI_API_KEY "$OPENAI_KEY_FILE"
load_key GEMINI_API_KEY "$GEMINI_KEY_FILE"

rm -rf "$OUT_DIR"
mkdir -p "$SEGMENTS_DIR"

if [[ ! -f "$SOURCE_DIR/transcript.md" ]]; then
  bash "$ROOT_DIR/adl/tools/demo_v0911_multiagent_podcast_pilot.sh" "$SOURCE_DIR"
fi

TURN_FILES=(
  "01-chatgpt-opening.md|ChatGPT|host / synthesizer|$CHATGPT_VOICE|openai|false|Speak warmly, thoughtfully, and with grounded podcast-host clarity."
  "02-gemini-challenge.md|Gemini|challenger / systems analyst|$GEMINI_RENDER_VOICE|$GEMINI_AUDIO_PROVIDER|$GEMINI_SURROGATE|Speak brightly, clearly, and incisively, like a fast systems analyst on a podcast."
  "03-claude-reframe.md|Claude|refiner / moral stylist|$CLAUDE_SURROGATE_VOICE|$CLAUDE_SURROGATE_PROVIDER|true|Speak reflectively and slightly formally, with calm measured emphasis."
  "04-chatgpt-bridge.md|ChatGPT|host / synthesizer|$CHATGPT_VOICE|openai|false|Speak warmly, thoughtfully, and with grounded podcast-host clarity."
  "05-gemini-deepening.md|Gemini|challenger / systems analyst|$GEMINI_RENDER_VOICE|$GEMINI_AUDIO_PROVIDER|$GEMINI_SURROGATE|Speak brightly, clearly, and incisively, like a fast systems analyst on a podcast."
  "06-claude-closure.md|Claude|refiner / moral stylist|$CLAUDE_SURROGATE_VOICE|$CLAUDE_SURROGATE_PROVIDER|true|Speak reflectively and slightly formally, with calm measured emphasis."
)

SEGMENT_MANIFEST_TMP="$OUT_DIR/segment_manifest.jsonl"
: > "$SEGMENT_MANIFEST_TMP"
SPEAKERS_INTRODUCED="|"

for spec in "${TURN_FILES[@]}"; do
  IFS='|' read -r filename speaker role voice provider surrogate instructions <<< "$spec"
  original_text="$(cat "$SOURCE_DIR/out/podcast/$filename")"
  if [[ "$SPEAKERS_INTRODUCED" != *"|$speaker|"* ]]; then
    text="I'm ${speaker}. ${original_text}"
    SPEAKERS_INTRODUCED="${SPEAKERS_INTRODUCED}${speaker}|"
  else
    text="$original_text"
  fi
  out_wav="$SEGMENTS_DIR/${filename%.md}.wav"
  if [[ "$provider" == "openai" ]]; then
    render_openai_wav "$text" "$voice" "$instructions" "$out_wav"
  elif [[ "$provider" == "gemini" ]]; then
    render_gemini_wav "$text" "$voice" "$out_wav"
  else
    echo "unsupported provider for audio segment: $provider" >&2
    exit 1
  fi
  python3 - "$SEGMENT_MANIFEST_TMP" "$speaker" "$role" "$filename" "$(basename "$out_wav")" "$provider" "$voice" "$surrogate" <<'PY'
import json, sys
path, speaker, role, source_file, audio_file, provider, voice, surrogate = sys.argv[1:9]
with open(path, 'a', encoding='utf-8') as f:
    f.write(json.dumps({
        'speaker': speaker,
        'role': role,
        'source_text_file': f'out/podcast/{source_file}',
        'audio_file': audio_file,
        'provider': provider,
        'voice': voice,
        'surrogate_render': surrogate == 'true',
    }) + '\n')
PY
done

python3 - "$SEGMENT_MANIFEST_TMP" "$AUDIO_MANIFEST" "$AUDIO_PACKET" "$EPISODE_AUDIO" "$SEGMENTS_DIR" "$SOURCE_DIR" "$CHATGPT_VOICE" "$GEMINI_AUDIO_PROVIDER" "$GEMINI_VOICE" "$GEMINI_OPENAI_SURROGATE_VOICE" "$CLAUDE_SURROGATE_PROVIDER" "$CLAUDE_SURROGATE_VOICE" <<'PY'
from array import array
import json
import math
import wave
import sys
from pathlib import Path

segment_manifest_tmp, manifest_path, packet_path, episode_audio_path, segments_dir, source_dir, chatgpt_voice, gemini_provider, gemini_voice, gemini_openai_voice, claude_provider, claude_voice = sys.argv[1:13]
segments = [json.loads(line) for line in Path(segment_manifest_tmp).read_text(encoding='utf-8').splitlines() if line.strip()]
GAP_MS = 350
params = None
combined = []
target_rms = 5000.0
peak_headroom = 30000.0

def normalize_pcm_16le(frames: bytes) -> bytes:
    samples = array('h')
    samples.frombytes(frames)
    if sys.byteorder != 'little':
        samples.byteswap()
    if not samples:
        return frames
    rms = math.sqrt(sum(int(s) * int(s) for s in samples) / len(samples))
    peak = max(abs(int(s)) for s in samples)
    if rms <= 1.0:
        return frames
    scale_rms = target_rms / rms
    scale_peak = peak_headroom / peak if peak > 0 else scale_rms
    scale = min(scale_rms, scale_peak)
    if scale > 6.0:
        scale = 6.0
    if scale < 0.35:
        scale = 0.35
    for i, sample in enumerate(samples):
        value = int(round(sample * scale))
        if value > 32767:
            value = 32767
        elif value < -32768:
            value = -32768
        samples[i] = value
    if sys.byteorder != 'little':
        samples.byteswap()
    return samples.tobytes()

for entry in segments:
    segment_path = Path(segments_dir) / entry['audio_file']
    with wave.open(str(segment_path), 'rb') as wf:
        current_params = (wf.getnchannels(), wf.getsampwidth(), wf.getframerate())
        frames = wf.readframes(wf.getnframes())
    if params is None:
        params = current_params
    elif current_params != params:
        raise SystemExit(f"mismatched wav params: expected {params}, got {current_params} for {entry['audio_file']}")
    if current_params[1] == 2:
        frames = normalize_pcm_16le(frames)
        with wave.open(str(segment_path), 'wb') as out_segment:
            out_segment.setnchannels(current_params[0])
            out_segment.setsampwidth(current_params[1])
            out_segment.setframerate(current_params[2])
            out_segment.writeframes(frames)
    combined.append(frames)
channels, width, rate = params
silence = b'\x00' * int(rate * width * channels * (GAP_MS / 1000.0))
with wave.open(episode_audio_path, 'wb') as out:
    out.setnchannels(channels)
    out.setsampwidth(width)
    out.setframerate(rate)
    for idx, frames in enumerate(combined):
        out.writeframes(frames)
        if idx != len(combined) - 1:
            out.writeframes(silence)
manifest = {
    'schema_version': 'adl.demo.multiagent_podcast_audio_manifest.v1',
    'source_episode_root': str(Path(source_dir)),
    'episode_audio': Path(episode_audio_path).name,
    'segments': segments,
}
Path(manifest_path).write_text(json.dumps(manifest, indent=2) + '\n', encoding='utf-8')
packet = [
    '# Multi-Agent Podcast Audio Packet',
    '',
    '- source packet: `source_episode/`',
    '- final episode audio: `episode.wav`',
    '- segment audio root: `audio/segments/`',
    '',
    '## Voice Routing',
    '',
    f'- `ChatGPT`: native OpenAI TTS via `{chatgpt_voice}`',
    f'- `Gemini`: {"native Gemini TTS" if gemini_provider == "gemini" else "surrogate OpenAI TTS"} via `{gemini_voice if gemini_provider == "gemini" else gemini_openai_voice}`',
    f'- `Claude`: surrogate `{claude_provider}` TTS via `{claude_voice}`',
    '',
    '## Audio Presentation',
    '',
    '- each speaker says their own name at the start of each segment',
    '- segment loudness is normalized toward a shared target so the episode is easier to follow',
    '',
    '## Proof Boundary',
    '',
    '- transcript authorship and audio-renderer identity remain separate',
    '- Claude remains the transcript author for Claude turns even though the audio is rendered by a surrogate TTS lane',
    '- audio does not prove long-term identity continuity',
]
Path(packet_path).write_text('\n'.join(packet).rstrip() + '\n', encoding='utf-8')
PY
rm -f "$SEGMENT_MANIFEST_TMP"

echo "Podcast audio proof surfaces:"
echo "  $EPISODE_AUDIO"
echo "  $AUDIO_MANIFEST"
echo "  $AUDIO_PACKET"
