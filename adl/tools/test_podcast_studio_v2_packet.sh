#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
REVIEW_DIR="$ROOT_DIR/docs/milestones/v0.91.3/review/podcast_studio_v2"
CARD_PATH="$ROOT_DIR/demos/v0.91.3/adl_podcast_studio_v2_episode_card.html"
FEATURE_PATH="$ROOT_DIR/docs/milestones/v0.91.3/features/PODCAST_STUDIO_V2_DEMO.md"
TMP_BASE="${TMPDIR:-/tmp}"
TMP_BASE="${TMP_BASE%/}"
TMP_OUT="$(mktemp -d "${TMP_BASE}/adl-podcast-studio-v2.XXXXXX")"
trap 'rm -rf "$TMP_OUT"' EXIT

bash "$ROOT_DIR/adl/tools/demo_v0913_podcast_studio_v2.sh" >"$TMP_OUT/default-run.out"
python3 "$ROOT_DIR/adl/tools/validate_podcast_studio_v2_packet.py" "$REVIEW_DIR" "$CARD_PATH" "$FEATURE_PATH"

GENERATED_PATHS=()
while IFS= read -r path; do
  GENERATED_PATHS+=("$path")
done < <(
  find "$REVIEW_DIR" -maxdepth 1 -type f | sort
  printf '%s\n' "$CARD_PATH"
  printf '%s\n' "$FEATURE_PATH"
)

BEFORE_HASHES_FILE="$TMP_OUT/before.hashes"
AFTER_HASHES_FILE="$TMP_OUT/after.hashes"
>"$BEFORE_HASHES_FILE"
>"$AFTER_HASHES_FILE"
for path in "${GENERATED_PATHS[@]}"; do
  printf '%s  %s\n' "$(shasum -a 256 "$path" | awk '{print $1}')" "$path" >>"$BEFORE_HASHES_FILE"
done

bash "$ROOT_DIR/adl/tools/demo_v0913_podcast_studio_v2.sh" >"$TMP_OUT/default-rerun.out"

for path in "${GENERATED_PATHS[@]}"; do
  printf '%s  %s\n' "$(shasum -a 256 "$path" | awk '{print $1}')" "$path" >>"$AFTER_HASHES_FILE"
done

cmp -s "$BEFORE_HASHES_FILE" "$AFTER_HASHES_FILE"

CUSTOM_REVIEW_DIR="$TMP_OUT/review/podcast_studio_v2"
CUSTOM_CARD_PATH="$TMP_OUT/adl_podcast_studio_v2_episode_card.html"
CUSTOM_FEATURE_PATH="$TMP_OUT/PODCAST_STUDIO_V2_DEMO.md"
python3 "$ROOT_DIR/adl/tools/generate_podcast_studio_v2_packet.py" \
  --review-dir "$CUSTOM_REVIEW_DIR" \
  --card-path "$CUSTOM_CARD_PATH" \
  --feature-path "$CUSTOM_FEATURE_PATH" \
  >"$TMP_OUT/custom-run.out"
python3 "$ROOT_DIR/adl/tools/validate_podcast_studio_v2_packet.py" "$CUSTOM_REVIEW_DIR" "$CUSTOM_CARD_PATH" "$CUSTOM_FEATURE_PATH"

grep -F "$CUSTOM_CARD_PATH" "$CUSTOM_REVIEW_DIR/README.md" >/dev/null
grep -F "$CUSTOM_CARD_PATH" "$CUSTOM_REVIEW_DIR/PODCAST_STUDIO_V2_PACKET_v0.91.3.md" >/dev/null
grep -F "$CUSTOM_FEATURE_PATH" "$CUSTOM_REVIEW_DIR/PODCAST_STUDIO_V2_PACKET_v0.91.3.md" >/dev/null
grep -F "$CUSTOM_REVIEW_DIR/" "$CUSTOM_FEATURE_PATH" >/dev/null

[[ "$(bash "$ROOT_DIR/adl/tools/demo_v0913_podcast_studio_v2.sh" --print-review-dir)" == "$REVIEW_DIR" ]]
[[ "$(bash "$ROOT_DIR/adl/tools/demo_v0913_podcast_studio_v2.sh" --print-card-path)" == "$CARD_PATH" ]]

echo "PASS: podcast studio v2 demo packet is deterministic and helper output is truthful"
