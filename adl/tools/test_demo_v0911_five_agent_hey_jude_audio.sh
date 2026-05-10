#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TMPDIR_ROOT="$(mktemp -d)"
trap 'rm -rf "$TMPDIR_ROOT"' EXIT

BACKING_WAV="$TMPDIR_ROOT/backing.wav"
python3 - "$BACKING_WAV" <<'PY'
import math
import struct
import sys
import wave

out_path = sys.argv[1]
rate = 24000
duration = 40.0
freq = 196.0
amplitude = 3600
frame_count = int(rate * duration)
with wave.open(out_path, 'wb') as wf:
    wf.setnchannels(1)
    wf.setsampwidth(2)
    wf.setframerate(rate)
    frames = bytearray()
    for i in range(frame_count):
        wobble = math.sin(2.0 * math.pi * (i / rate) * 0.4) * 0.15
        sample = int(round(amplitude * (1.0 + wobble) * math.sin(2.0 * math.pi * freq * (i / rate))))
        frames.extend(struct.pack('<h', sample))
    wf.writeframes(bytes(frames))
PY

OUT_DIR="$TMPDIR_ROOT/artifacts"
(
  cd "$ROOT_DIR"
  ADL_HEY_JUDE_AUDIO_TEST_TONES=1 \
  ADL_HEY_JUDE_BACKING_WAV="$BACKING_WAV" \
  bash adl/tools/demo_v0911_five_agent_hey_jude_audio.sh "$OUT_DIR" >/dev/null
)

python3 "$ROOT_DIR/adl/tools/validate_five_agent_hey_jude_audio_demo.py" "$OUT_DIR"

for required in \
  "$OUT_DIR/episode.wav" \
  "$OUT_DIR/audio_manifest.json" \
  "$OUT_DIR/audio_packet.md" \
  "$OUT_DIR/proof_note.md" \
  "$OUT_DIR/playback_transcript.md" \
  "$OUT_DIR/cue_timing_register.json" \
  "$OUT_DIR/source_episode/performance_manifest.json" \
  "$OUT_DIR/source_episode/transcript.md"; do
  [[ -f "$required" ]] || {
    echo "assertion failed: missing artifact $required" >&2
    exit 1
  }
done

echo "demo_v0911_five_agent_hey_jude_audio: ok"
