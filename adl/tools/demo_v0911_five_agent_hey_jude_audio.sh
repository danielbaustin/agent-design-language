#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="${1:-$ROOT_DIR/artifacts/v0911/five_agent_hey_jude_audio}"
SOURCE_DIR="${ADL_HEY_JUDE_AUDIO_SOURCE_DIR:-$OUT_DIR/source_episode}"
SEGMENTS_DIR="$OUT_DIR/audio/segments"
MIX_AUDIO="$OUT_DIR/episode.wav"
AUDIO_MANIFEST="$OUT_DIR/audio_manifest.json"
AUDIO_PACKET="$OUT_DIR/audio_packet.md"
PROOF_NOTE="$OUT_DIR/proof_note.md"
PLAYBACK_TRANSCRIPT="$OUT_DIR/playback_transcript.md"
CUE_TIMING_REGISTER="$OUT_DIR/cue_timing_register.json"
OPENAI_KEY_FILE="${ADL_OPENAI_KEY_FILE:-$HOME/keys/openai2.key}"
OPENAI_TTS_MODEL="${ADL_HEY_JUDE_OPENAI_TTS_MODEL:-gpt-4o-mini-tts}"
BACKING_TRACK_PATH="${ADL_HEY_JUDE_BACKING_WAV:-}"
LAYER8_VOICE="${ADL_HEY_JUDE_LAYER8_VOICE:-ash}"
CHATGPT_VOICE="${ADL_HEY_JUDE_CHATGPT_VOICE:-coral}"
CLAUDE_VOICE="${ADL_HEY_JUDE_CLAUDE_VOICE:-onyx}"
GEMINI_VOICE="${ADL_HEY_JUDE_GEMINI_VOICE:-echo}"
DEEPSEEK_VOICE="${ADL_HEY_JUDE_DEEPSEEK_VOICE:-sage}"

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

if [[ "${ADL_HEY_JUDE_AUDIO_TEST_TONES:-0}" != "1" ]]; then
  load_key OPENAI_API_KEY "$OPENAI_KEY_FILE"
fi

rm -rf "$OUT_DIR"
mkdir -p "$SEGMENTS_DIR"

if [[ ! -f "$SOURCE_DIR/transcript.md" ]]; then
  bash "$ROOT_DIR/adl/tools/demo_v0891_five_agent_hey_jude.sh" "$SOURCE_DIR"
fi

python3 "$ROOT_DIR/adl/tools/validate_five_agent_music_demo.py" "$SOURCE_DIR" >/dev/null

python3 - "$SOURCE_DIR" "$SEGMENTS_DIR" "$CUE_TIMING_REGISTER" "$PLAYBACK_TRANSCRIPT" "$AUDIO_MANIFEST" "$AUDIO_PACKET" "$PROOF_NOTE" "$MIX_AUDIO" "$BACKING_TRACK_PATH" "$LAYER8_VOICE" "$CHATGPT_VOICE" "$CLAUDE_VOICE" "$GEMINI_VOICE" "$DEEPSEEK_VOICE" <<'PY'
from __future__ import annotations

from array import array
import base64
import json
import math
import os
import struct
import subprocess
import sys
import urllib.request
import wave
from pathlib import Path

(
    source_dir,
    segments_dir,
    cue_timing_path,
    playback_transcript_path,
    manifest_path,
    packet_path,
    proof_note_path,
    mix_audio_path,
    backing_track_path,
    layer8_voice,
    chatgpt_voice,
    claude_voice,
    gemini_voice,
    deepseek_voice,
) = sys.argv[1:15]

ROOT = Path(source_dir)
SEGMENTS = Path(segments_dir)
TEST_TONES = os.environ.get("ADL_HEY_JUDE_AUDIO_TEST_TONES") == "1"
OPENAI_KEY = os.environ.get("OPENAI_API_KEY", "")
OPENAI_TTS_MODEL = os.environ.get("ADL_HEY_JUDE_OPENAI_TTS_MODEL", "gpt-4o-mini-tts")
RATE = 24000
SAMPLE_WIDTH = 2
CHANNELS = 1
TARGET_RMS = 3500.0
FINAL_CEILING = 25000
DUCK_GAIN = 0.26

INSERTS = [
    {
        "turn": 1,
        "speaker": "Layer 8",
        "role": "human bandleader",
        "section": "Opening",
        "source_file": "01-layer8-opening.md",
        "voice": layer8_voice,
        "instructions": "Speak like a calm bandleader cueing a live room with grounded confidence.",
        "start_ms": 900,
    },
    {
        "turn": 2,
        "speaker": "ChatGPT",
        "role": "conductor-builder",
        "section": "Opening",
        "source_file": "02-chatgpt-opening.md",
        "voice": chatgpt_voice,
        "instructions": "Speak warmly and clearly, like a conductor setting up the next moment.",
        "start_ms": 5200,
    },
    {
        "turn": 3,
        "speaker": "Claude",
        "role": "reflective harmonizer",
        "section": "Verse Rotation",
        "source_file": "03-claude-verse.md",
        "voice": claude_voice,
        "instructions": "Speak lower, slower, and reflectively, like a thoughtful harmonizer entering the arrangement.",
        "start_ms": 13000,
    },
    {
        "turn": 4,
        "speaker": "Gemini",
        "role": "bright performer",
        "section": "Verse Rotation",
        "source_file": "04-gemini-verse.md",
        "voice": gemini_voice,
        "instructions": "Speak brightly, briskly, and musically, like a performer lifting the energy.",
        "start_ms": 20500,
    },
    {
        "turn": 5,
        "speaker": "DeepSeek",
        "role": "pattern keeper",
        "section": "Curtain Call",
        "source_file": "10-deepseek-curtain.md",
        "voice": deepseek_voice,
        "instructions": "Speak with measured clarity, like a pattern keeper closing the room with a final cue.",
        "start_ms": 31500,
    },
]

BANNED_STRINGS = [
    "Take a sad song and make it better",
    "Remember to let her into your heart",
]


def first_snippet(path: Path, speaker: str) -> str:
    text = path.read_text(encoding="utf-8").strip()
    for banned in BANNED_STRINGS:
        if banned in text:
            raise SystemExit(f"copyright-sensitive text leaked into source snippet: {banned}")
    lines = [line.strip() for line in text.splitlines() if line.strip()]
    joined = " ".join(lines[:3]).strip()
    clipped = joined[:260].rsplit(" ", 1)[0] if len(joined) > 260 else joined
    return f"{speaker}. {clipped}"


def render_test_tone(out_path: Path, speaker: str) -> None:
    freq_map = {
        "Layer 8": 180.0,
        "ChatGPT": 220.0,
        "Claude": 260.0,
        "Gemini": 330.0,
        "DeepSeek": 300.0,
    }
    freq = freq_map.get(speaker, 240.0)
    duration = 1.9
    frame_count = int(RATE * duration)
    amplitude = 9200
    with wave.open(str(out_path), "wb") as wf:
        wf.setnchannels(CHANNELS)
        wf.setsampwidth(SAMPLE_WIDTH)
        wf.setframerate(RATE)
        frames = bytearray()
        for i in range(frame_count):
            sample = int(round(amplitude * math.sin(2.0 * math.pi * freq * (i / RATE))))
            frames.extend(struct.pack("<h", sample))
        wf.writeframes(bytes(frames))


def render_openai_wav(text: str, voice: str, instructions: str, out_path: Path) -> None:
    payload = json.dumps(
        {
            "model": OPENAI_TTS_MODEL,
            "voice": voice,
            "input": text,
            "instructions": instructions,
            "response_format": "wav",
        }
    ).encode("utf-8")
    req = urllib.request.Request(
        "https://api.openai.com/v1/audio/speech",
        data=payload,
        headers={
            "Authorization": f"Bearer {OPENAI_KEY}",
            "Content-Type": "application/json",
        },
        method="POST",
    )
    with urllib.request.urlopen(req, timeout=120) as resp:
        out_path.write_bytes(resp.read())


def stereo_to_mono(frames: bytes, channels: int) -> bytes:
    if channels == 1:
        return frames
    if channels != 2:
        raise SystemExit(f"unsupported channel count: {channels}")
    samples = array("h")
    samples.frombytes(frames)
    mono = array("h")
    for i in range(0, len(samples), 2):
        left = int(samples[i])
        right = int(samples[i + 1])
        mono.append(int(round((left + right) / 2.0)))
    return mono.tobytes()


def resample_linear(frames: bytes, src_rate: int, dst_rate: int) -> bytes:
    if src_rate == dst_rate:
        return frames
    source = array("h")
    source.frombytes(frames)
    if not source:
        return frames
    target_length = max(1, int(round(len(source) * dst_rate / src_rate)))
    target = array("h")
    if len(source) == 1:
        target.extend([source[0]] * target_length)
        return target.tobytes()
    step = src_rate / dst_rate
    for i in range(target_length):
        position = i * step
        left = int(position)
        right = min(left + 1, len(source) - 1)
        frac = position - left
        value = int(round((1.0 - frac) * int(source[left]) + frac * int(source[right])))
        target.append(max(-32768, min(32767, value)))
    return target.tobytes()


def read_wav_mono_resampled(path: Path, target_rate: int) -> bytes:
    with wave.open(str(path), "rb") as wf:
        channels = wf.getnchannels()
        sampwidth = wf.getsampwidth()
        framerate = wf.getframerate()
        frames = wf.readframes(wf.getnframes())
    if sampwidth != 2:
        raise SystemExit(f"unsupported wav sample width for backing track: {sampwidth}")
    frames = stereo_to_mono(frames, channels)
    if framerate != target_rate:
        frames = resample_linear(frames, framerate, target_rate)
    return frames


def measure(frames: bytes) -> tuple[float, int]:
    samples = array("h")
    samples.frombytes(frames)
    if not samples:
        return 0.0, 0
    rms = math.sqrt(sum(int(s) * int(s) for s in samples) / len(samples))
    peak = max(abs(int(s)) for s in samples)
    return rms, peak


def normalize(frames: bytes, target_rms: float) -> bytes:
    samples = array("h")
    samples.frombytes(frames)
    if not samples:
        return frames
    rms, peak = measure(frames)
    if rms <= 1.0:
        return frames
    scale_rms = target_rms / rms
    scale_peak = FINAL_CEILING / peak if peak > 0 else scale_rms
    scale = min(scale_rms, scale_peak, 5.0)
    for i, sample in enumerate(samples):
        value = int(round(sample * scale))
        value = max(-32768, min(32767, value))
        samples[i] = value
    return samples.tobytes()


def compress_limit(frames: bytes) -> bytes:
    samples = array("h")
    samples.frombytes(frames)
    threshold = 11000
    ratio = 2.2
    for i, sample in enumerate(samples):
        sign = 1 if sample >= 0 else -1
        magnitude = abs(int(sample))
        if magnitude > threshold:
            magnitude = threshold + int(round((magnitude - threshold) / ratio))
        value = sign * min(magnitude, FINAL_CEILING)
        samples[i] = max(-32768, min(32767, value))
    return samples.tobytes()


def mix_voice_over(backing: bytes, voice_frames: bytes, start_frame: int) -> bytes:
    backing_samples = array("h")
    backing_samples.frombytes(backing)
    voice_samples = array("h")
    voice_samples.frombytes(voice_frames)
    required = start_frame + len(voice_samples)
    if len(backing_samples) < required:
        backing_samples.extend([0] * (required - len(backing_samples)))
    for i, sample in enumerate(voice_samples):
        idx = start_frame + i
        bed = int(round(backing_samples[idx] * DUCK_GAIN))
        mixed = bed + int(sample)
        backing_samples[idx] = max(-32768, min(32767, mixed))
    return backing_samples.tobytes()


segment_entries = []
for entry in INSERTS:
    text = first_snippet(ROOT / "out" / "performance" / entry["source_file"], entry["speaker"])
    out_path = SEGMENTS / f"{entry['turn']:02d}-{entry['speaker'].lower().replace(' ', '-')}.wav"
    if TEST_TONES:
        render_test_tone(out_path, entry["speaker"])
    else:
        render_openai_wav(text, entry["voice"], entry["instructions"], out_path)
    with wave.open(str(out_path), "rb") as wf:
        frames = wf.readframes(wf.getnframes())
        params = (wf.getnchannels(), wf.getsampwidth(), wf.getframerate())
    if params != (CHANNELS, SAMPLE_WIDTH, RATE):
        if params[1] != 2:
            raise SystemExit(f"unsupported segment sample width: {params}")
        raw = frames
        raw = stereo_to_mono(raw, params[0])
        if params[2] != RATE:
            raw = resample_linear(raw, params[2], RATE)
        frames = raw
        with wave.open(str(out_path), "wb") as wf:
            wf.setnchannels(CHANNELS)
            wf.setsampwidth(SAMPLE_WIDTH)
            wf.setframerate(RATE)
            wf.writeframes(frames)
    frames = normalize(frames, TARGET_RMS)
    frames = compress_limit(frames)
    with wave.open(str(out_path), "wb") as wf:
        wf.setnchannels(CHANNELS)
        wf.setsampwidth(SAMPLE_WIDTH)
        wf.setframerate(RATE)
        wf.writeframes(frames)
    rms, peak = measure(frames)
    segment_entries.append(
        {
            **entry,
            "spoken_text": text,
            "audio_file": out_path.name,
            "segment_metrics": {"rms": round(rms, 1), "peak": peak},
        }
    )

end_frame = max(int((entry["start_ms"] / 1000.0) * RATE) + wave.open(str(SEGMENTS / entry["audio_file"]), "rb").getnframes() for entry in segment_entries)
base_length = end_frame + int(2.5 * RATE)
if backing_track_path:
    backing_path = Path(backing_track_path)
    if not backing_path.is_file():
        raise SystemExit(f"backing track not found: {backing_track_path}")
    backing = read_wav_mono_resampled(backing_path, RATE)
else:
    backing_path = None
    backing = b"\x00" * (base_length * SAMPLE_WIDTH)

mixed = backing
for entry in segment_entries:
    with wave.open(str(SEGMENTS / entry["audio_file"]), "rb") as wf:
        frames = wf.readframes(wf.getnframes())
        frame_count = wf.getnframes()
    entry["duration_ms"] = int(round((frame_count / RATE) * 1000))
    entry["end_ms"] = entry["start_ms"] + entry["duration_ms"]
    mixed = mix_voice_over(mixed, frames, int((entry["start_ms"] / 1000.0) * RATE))

mixed = compress_limit(normalize(mixed, 3200.0))
mix_rms, mix_peak = measure(mixed)
with wave.open(mix_audio_path, "wb") as wf:
    wf.setnchannels(CHANNELS)
    wf.setsampwidth(SAMPLE_WIDTH)
    wf.setframerate(RATE)
    wf.writeframes(mixed)

cue_register = {
    "schema_version": "adl.hey_jude_audio_cue_register.v1",
    "source_episode_root": str(ROOT),
    "backing_track": {
        "provided": backing_path is not None,
        "asset_name": backing_path.name if backing_path else None,
        "mode": "operator_supplied_local_wav" if backing_path else "voice_only_no_bed",
    },
    "voice_inserts": [
        {
            "turn": entry["turn"],
            "speaker": entry["speaker"],
            "role": entry["role"],
            "section": entry["section"],
            "source_text_file": f"out/performance/{entry['source_file']}",
            "audio_file": f"audio/segments/{entry['audio_file']}",
            "start_ms": entry["start_ms"],
            "end_ms": entry["end_ms"],
        }
        for entry in segment_entries
    ],
}
Path(cue_timing_path).write_text(json.dumps(cue_register, indent=2) + "\n", encoding="utf-8")

playback_lines = [
    "# Hey Jude Audio Playback Transcript",
    "",
    "> Spoken interjection track aligned to the bounded five-agent proof packet.",
    "",
]
for entry in segment_entries:
    playback_lines.extend(
        [
            f"## {entry['speaker']} · {entry['section']}",
            "",
            f"- start: `{entry['start_ms']}ms`",
            f"- end: `{entry['end_ms']}ms`",
            f"- source: `out/performance/{entry['source_file']}`",
            "",
            entry["spoken_text"],
            "",
        ]
    )
Path(playback_transcript_path).write_text("\n".join(playback_lines).rstrip() + "\n", encoding="utf-8")

manifest = {
    "schema_version": "adl.hey_jude_audio_manifest.v1",
    "demo_id": "v0.91.1.five_agent_hey_jude_audio_follow_on",
    "source_demo_id": "v0.89.1.five_agent_hey_jude_midi_demo",
    "source_episode_root": str(ROOT),
    "mixed_audio": Path(mix_audio_path).name,
    "backing_track": cue_register["backing_track"],
    "render_mode": "spoken_interjection_mix",
    "voices": {
        "Layer 8": {"provider": "openai", "voice": layer8_voice, "mode": "surrogate_spoken"},
        "ChatGPT": {"provider": "openai", "voice": chatgpt_voice, "mode": "native_spoken"},
        "Claude": {"provider": "openai", "voice": claude_voice, "mode": "surrogate_spoken"},
        "Gemini": {"provider": "openai", "voice": gemini_voice, "mode": "surrogate_spoken"},
        "DeepSeek": {"provider": "openai", "voice": deepseek_voice, "mode": "surrogate_spoken"},
    },
    "timing": cue_register["voice_inserts"],
    "mastering": {
        "segment_target_rms": TARGET_RMS,
        "final_mix_metrics": {"rms": round(mix_rms, 1), "peak": mix_peak},
        "duck_gain": DUCK_GAIN,
    },
    "proof_surfaces": {
        "source_transcript": "source_episode/transcript.md",
        "source_manifest": "source_episode/performance_manifest.json",
        "midi_event_log": "source_episode/midi_event_log.json",
        "cue_timeline": "source_episode/cue_timeline.json",
        "playback_transcript": Path(playback_transcript_path).name,
        "cue_timing_register": Path(cue_timing_path).name,
        "audio_packet": Path(packet_path).name,
        "proof_note": Path(proof_note_path).name,
    },
}
Path(manifest_path).write_text(json.dumps(manifest, indent=2) + "\n", encoding="utf-8")

packet = [
    "# Hey Jude Audio Follow-On Packet",
    "",
    "- source proof packet: `source_episode/`",
    f"- mixed audio artifact: `{Path(mix_audio_path).name}`",
    "- render mode: spoken interjection mix over optional operator-supplied backing track",
    "",
    "## What This Adds",
    "",
    "- a replayable mixed audio file",
    "- five timed spoken inserts aligned to the bounded ensemble structure",
    "- a playback transcript and cue register that preserve reviewability",
    "",
    "## What This Does Not Claim",
    "",
    "- no full synthetic singing of the song",
    "- no repo-distributed copyrighted backing audio",
    "- no general music platform or live-performance guarantee",
    "",
    "## Copyright / Provenance",
    "",
    f"- backing track active: `{'yes' if backing_path else 'no'}`",
    f"- backing asset name: `{backing_path.name if backing_path else 'not_applicable'}`",
    "- any backing track is operator-supplied local media and not a tracked repo artifact",
]
Path(packet_path).write_text("\n".join(packet).rstrip() + "\n", encoding="utf-8")

proof_note = [
    "# Hey Jude Audio Follow-On Proof Note",
    "",
    "- The original transcript/MIDI proof path remains the canonical coordination surface.",
    "- This follow-on adds a listenable spoken-performance layer without replacing the source proof packet.",
    "- Audio authorship and audio rendering remain separate from the source transcript authorship.",
]
Path(proof_note_path).write_text("\n".join(proof_note).rstrip() + "\n", encoding="utf-8")
PY

echo "Hey Jude audio proof surfaces:"
echo "  $MIX_AUDIO"
echo "  $AUDIO_MANIFEST"
echo "  $AUDIO_PACKET"
echo "  $PLAYBACK_TRANSCRIPT"
echo "  $CUE_TIMING_REGISTER"
