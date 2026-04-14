#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TMPDIR_ROOT="$(mktemp -d)"
trap 'rm -rf "$TMPDIR_ROOT"' EXIT

OUT_DIR="$TMPDIR_ROOT/artifacts"
RUN_ROOT="$OUT_DIR/runtime/runs/v0-89-medium-article-writing"

(
  cd "$ROOT_DIR"
  bash adl/tools/demo_v089_medium_article_writing.sh "$OUT_DIR" >/dev/null
)

for required in \
  "$OUT_DIR/demo_manifest.json" \
  "$OUT_DIR/README.md" \
  "$OUT_DIR/input_packet/v0-89-medium-article-brief.md" \
  "$OUT_DIR/article_packet/outline.md" \
  "$OUT_DIR/article_packet/title_options.md" \
  "$OUT_DIR/article_packet/draft.md" \
  "$OUT_DIR/article_packet/editorial_notes.md" \
  "$OUT_DIR/article_packet/publish_summary.json" \
  "$OUT_DIR/article_packet/reviewer_brief.md" \
  "$RUN_ROOT/run_summary.json" \
  "$RUN_ROOT/steps.json" \
  "$RUN_ROOT/logs/trace_v1.json" \
  "$OUT_DIR/run_log.txt"; do
  [[ -f "$required" ]] || {
    echo "assertion failed: missing artifact $required" >&2
    exit 1
  }
done

python3 - "$OUT_DIR/demo_manifest.json" "$OUT_DIR/article_packet/publish_summary.json" <<'PY'
import json
import sys
manifest = json.load(open(sys.argv[1], encoding="utf-8"))
summary = json.load(open(sys.argv[2], encoding="utf-8"))
assert manifest["schema_version"] == "adl.medium_article_writing_demo.v1"
assert manifest["demo_id"] == "v0.89.medium_article_writing"
assert summary["schema_version"] == "adl.medium_article_packet.v1"
assert summary["article_id"] == "v0.89.medium.writing.demo"
assert "article_packet/draft.md" in summary["artifacts"]
PY

grep -Fq '# Draft' "$OUT_DIR/article_packet/draft.md" || {
  echo "assertion failed: draft artifact missing heading" >&2
  exit 1
}

grep -Fq 'reviewer-friendly' "$OUT_DIR/article_packet/reviewer_brief.md" || {
  echo "assertion failed: reviewer brief missing expected language" >&2
  exit 1
}

if grep -R -E '/Users/|/private/tmp|/tmp/' "$OUT_DIR" >/dev/null 2>&1; then
  echo "assertion failed: absolute path leaked into generated artifacts" >&2
  exit 1
fi

echo "demo_v089_medium_article_writing: ok"
