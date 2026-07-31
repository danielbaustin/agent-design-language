#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="${1:-$ROOT_DIR/demos/podcast}"
WORK_DIR="${ADL_PODCAST_LAUNCH_WORK_DIR:-$ROOT_DIR/out/podcast-launch-5711}"
EPISODES="$ROOT_DIR/docs/milestones/v0.91.8/review/podcast_launch_5711/episodes.json"
SOURCE_DIR="$WORK_DIR/source_episode"
AUDIO_RENDER_DIR="$WORK_DIR/audio-render"
AUDIO_FILE="meet-the-ai-coworkers.wav"

work_base="$(basename "$WORK_DIR")"
case "$WORK_DIR" in
  ""|"/"|"$HOME"|"$ROOT_DIR"|"$ROOT_DIR/"*|"/Volumes/FastWork"|"/Volumes/FastWork/"*)
    case "$WORK_DIR" in
      "$ROOT_DIR"/out/podcast-launch-*|"$ROOT_DIR"/out/podcast-launch-*/*|*/adl-podcast-launch-*|*/adl-podcast-launch-*/*)
        ;;
      *)
        echo "unsafe ADL_PODCAST_LAUNCH_WORK_DIR: $WORK_DIR" >&2
        echo "use a dedicated path named podcast-launch-* or adl-podcast-launch-*" >&2
        exit 2
        ;;
    esac
    ;;
  *)
    case "$work_base" in
      podcast-launch-*|adl-podcast-launch-*) ;;
      *)
        echo "unsafe ADL_PODCAST_LAUNCH_WORK_DIR: $WORK_DIR" >&2
        echo "use a dedicated path named podcast-launch-* or adl-podcast-launch-*" >&2
        exit 2
        ;;
    esac
    ;;
esac

rm -rf "$WORK_DIR"
mkdir -p "$WORK_DIR" "$OUT_DIR/audio"

python3 "$ROOT_DIR/adl/tools/generate_podcast_launch_packet.py" \
  --episodes "$EPISODES" \
  --out "$OUT_DIR" \
  --audio-source "$SOURCE_DIR" \
  --audio-file "$AUDIO_FILE" \
  --audio-bytes 0

ADL_PODCAST_AUDIO_SOURCE_DIR="$SOURCE_DIR" \
ADL_PODCAST_AUDIO_TEST_TONES="${ADL_PODCAST_AUDIO_TEST_TONES:-1}" \
ADL_PODCAST_GEMINI_AUDIO_PROVIDER="${ADL_PODCAST_GEMINI_AUDIO_PROVIDER:-openai}" \
ADL_PODCAST_CLAUDE_SURROGATE_PROVIDER="${ADL_PODCAST_CLAUDE_SURROGATE_PROVIDER:-openai}" \
  bash "$ROOT_DIR/adl/tools/demo_v0911_multiagent_podcast_audio.sh" "$AUDIO_RENDER_DIR"

cp "$AUDIO_RENDER_DIR/episode.wav" "$OUT_DIR/audio/$AUDIO_FILE"
audio_bytes="$(wc -c < "$OUT_DIR/audio/$AUDIO_FILE" | tr -d ' ')"

python3 "$ROOT_DIR/adl/tools/generate_podcast_launch_packet.py" \
  --episodes "$EPISODES" \
  --out "$OUT_DIR" \
  --audio-source "$SOURCE_DIR" \
  --audio-file "$AUDIO_FILE" \
  --audio-bytes "$audio_bytes"

python3 "$ROOT_DIR/adl/tools/validate_podcast_launch_packet.py" "$OUT_DIR" "$EPISODES"

printf 'Podcast launch surfaces:\n'
printf '  %s\n' "$OUT_DIR/index.html"
printf '  %s\n' "$OUT_DIR/episodes/meet-the-ai-coworkers/index.html"
printf '  %s\n' "$OUT_DIR/feed.xml"
printf '  %s\n' "$OUT_DIR/audio/$AUDIO_FILE"
