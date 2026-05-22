#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
REVIEW_DIR="$ROOT_DIR/docs/milestones/v0.91.3/review/podcast_studio_v2"
CARD_PATH="$ROOT_DIR/demos/v0.91.3/adl_podcast_studio_v2_episode_card.html"
FEATURE_PATH="$ROOT_DIR/docs/milestones/v0.91.3/features/PODCAST_STUDIO_V2_DEMO.md"

case "${1:-}" in
  --print-review-dir)
    printf '%s\n' "$REVIEW_DIR"
    exit 0
    ;;
  --print-card-path)
    printf '%s\n' "$CARD_PATH"
    exit 0
    ;;
esac

python3 "$ROOT_DIR/adl/tools/generate_podcast_studio_v2_packet.py" \
  --review-dir "$REVIEW_DIR" \
  --card-path "$CARD_PATH" \
  --feature-path "$FEATURE_PATH"
