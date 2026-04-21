#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TMPDIR_ROOT="$(mktemp -d)"
trap 'rm -rf "$TMPDIR_ROOT"' EXIT

OUT_DIR="$TMPDIR_ROOT/paper_sonata_expansion"

(
  cd "$ROOT_DIR"
  bash adl/tools/demo_v0902_paper_sonata_expansion.sh "$OUT_DIR" >/dev/null
  python3 adl/tools/validate_v0902_paper_sonata_expansion.py "$OUT_DIR" >/dev/null
)

for required in \
  "$OUT_DIR/run_manifest.json" \
  "$OUT_DIR/README.md" \
  "$OUT_DIR/source_packet/source_manifest.json" \
  "$OUT_DIR/source_packet/idea_summary.md" \
  "$OUT_DIR/source_packet/lab_notes.md" \
  "$OUT_DIR/source_packet/experiment_results.json" \
  "$OUT_DIR/source_packet/target_venue.md" \
  "$OUT_DIR/source_packet/citations_seed.json" \
  "$OUT_DIR/source_packet/paper_constraints.md" \
  "$OUT_DIR/role_outputs/conductor_plan.json" \
  "$OUT_DIR/role_outputs/scholar_literature_review.md" \
  "$OUT_DIR/role_outputs/analyst_results_summary.md" \
  "$OUT_DIR/manuscript/draft.md" \
  "$OUT_DIR/review/editor_review_notes.md" \
  "$OUT_DIR/review/revision_requests.json" \
  "$OUT_DIR/review/reviewer_brief.md" \
  "$OUT_DIR/revision/revised_manuscript.md" \
  "$OUT_DIR/publication_gate/no_submission.md"; do
  [[ -f "$required" ]] || {
    echo "assertion failed: missing $required" >&2
    exit 1
  }
done

python3 - "$OUT_DIR/run_manifest.json" "$OUT_DIR/review/revision_requests.json" <<'PY'
import json
import sys
from pathlib import Path

manifest = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
requests = json.loads(Path(sys.argv[2]).read_text(encoding="utf-8"))

assert manifest["schema_version"] == "adl.v0902.paper_sonata_expansion.v1"
assert manifest["classification"] == "proving_fixture"
assert manifest["baseline_preserved"] is True
assert manifest["publication_allowed"] is False
assert manifest["submission_attempted"] is False
assert manifest["live_web_citations"] is False
assert manifest["publication_ready_claimed"] is False
assert manifest["autonomous_scientific_discovery_claimed"] is False
assert len(requests["requests"]) == 3
assert all(item["status"] == "addressed" for item in requests["requests"])
PY

grep -Fq "Generated draft text" "$OUT_DIR/manuscript/draft.md" || {
  echo "assertion failed: draft missing artifact type marker" >&2
  exit 1
}

grep -Fq "Review feedback" "$OUT_DIR/review/editor_review_notes.md" || {
  echo "assertion failed: review notes missing artifact type marker" >&2
  exit 1
}

grep -Fq "Revision output" "$OUT_DIR/revision/revised_manuscript.md" || {
  echo "assertion failed: revised manuscript missing artifact type marker" >&2
  exit 1
}

grep -Fq "Publication allowed: false" "$OUT_DIR/publication_gate/no_submission.md" || {
  echo "assertion failed: publication gate missing blocked publication marker" >&2
  exit 1
}

for leaked_text in \
  "/Users/alice/private.txt" \
  "/private/tmp/paper-sonata-leak" \
  "OPENAI_API_KEY=secret"; do
  LEAK_DIR="$TMPDIR_ROOT/leak-check"
  rm -rf "$LEAK_DIR"
  cp -R "$OUT_DIR" "$LEAK_DIR"
  printf '\nInjected leak: %s\n' "$leaked_text" >> "$LEAK_DIR/revision/revised_manuscript.md"
  if python3 "$ROOT_DIR/adl/tools/validate_v0902_paper_sonata_expansion.py" "$LEAK_DIR" >/dev/null 2>&1; then
    echo "assertion failed: validator accepted leaked text $leaked_text" >&2
    exit 1
  fi
done

echo "demo_v0902_paper_sonata_expansion: ok"
