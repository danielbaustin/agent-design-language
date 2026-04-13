#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TMPDIR_ROOT="$(mktemp -d)"
trap 'rm -rf "$TMPDIR_ROOT"' EXIT

OUT_DIR="$TMPDIR_ROOT/artifacts"
RUN_ROOT="$OUT_DIR/runtime/runs/v0-88-paper-sonata-demo"

(
  cd "$ROOT_DIR"
  bash adl/tools/demo_v088_paper_sonata.sh "$OUT_DIR" >/dev/null
)

for required in \
  "$OUT_DIR/demo_manifest.json" \
  "$OUT_DIR/README.md" \
  "$OUT_DIR/manuscript_package/plan.json" \
  "$OUT_DIR/manuscript_package/outline.md" \
  "$OUT_DIR/manuscript_package/literature_review.md" \
  "$OUT_DIR/manuscript_package/results_summary.md" \
  "$OUT_DIR/manuscript_package/review_notes.md" \
  "$OUT_DIR/manuscript_package/paper_draft.md" \
  "$OUT_DIR/manuscript_package/sections/intro.md" \
  "$OUT_DIR/manuscript_package/sections/method.md" \
  "$OUT_DIR/manuscript_package/sections/results.md" \
  "$OUT_DIR/manuscript_package/sections/discussion.md" \
  "$OUT_DIR/out/roles/01-conductor-plan.json" \
  "$OUT_DIR/out/roles/09-editor-review-notes.md" \
  "$RUN_ROOT/run_summary.json" \
  "$RUN_ROOT/steps.json" \
  "$RUN_ROOT/logs/trace_v1.json" \
  "$OUT_DIR/run_log.txt"; do
  [[ -f "$required" ]] || {
    echo "assertion failed: missing artifact $required" >&2
    exit 1
  }
done

python3 - "$OUT_DIR/demo_manifest.json" "$OUT_DIR/manuscript_package/paper_draft.md" <<'PY'
import json
import sys

manifest = json.load(open(sys.argv[1], encoding="utf-8"))
paper_draft = open(sys.argv[2], encoding="utf-8").read()
if manifest.get("schema_version") != "adl.paper_sonata_demo.v1":
    raise SystemExit("unexpected Paper Sonata manifest schema")
if manifest.get("demo_id") != "D8":
    raise SystemExit("unexpected demo id")
roles = manifest.get("workflow_shape", {}).get("roles", [])
if roles != ["conductor", "scholar", "analyst", "composer", "editor"]:
    raise SystemExit(f"unexpected roles: {roles}")
if "# Introduction" not in paper_draft or "# Discussion" not in paper_draft:
    raise SystemExit("paper draft missing expected sections")
PY

grep -Fq '"run_id": "v0-88-paper-sonata-demo"' "$RUN_ROOT/run_summary.json" || {
  echo "assertion failed: run_summary missing run id" >&2
  exit 1
}
grep -Fq '"paper_sonata.editor.review"' "$RUN_ROOT/steps.json" || {
  echo "assertion failed: steps artifact missing editor review step" >&2
  exit 1
}
grep -Fq 'bounded manuscript-assembly workflow' "$OUT_DIR/demo_manifest.json" || {
  echo "assertion failed: manifest missing flagship claim" >&2
  exit 1
}

if grep -R -E '/Users/|/private/tmp|/tmp/' "$OUT_DIR" >/dev/null 2>&1; then
  echo "assertion failed: absolute path leaked into generated artifacts" >&2
  exit 1
fi

echo "demo_v088_paper_sonata: ok"
