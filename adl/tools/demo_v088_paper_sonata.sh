#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="${1:-$ROOT_DIR/artifacts/v088/paper_sonata}"
FIXTURE_DIR="$ROOT_DIR/demos/fixtures/paper_sonata"
RUNTIME_ROOT="$OUT_DIR/runtime"
RUNS_ROOT="$RUNTIME_ROOT/runs"
STEP_OUT="$OUT_DIR/out"
RUN_ID="v0-88-paper-sonata-demo"
EXAMPLE="adl/examples/v0-88-paper-sonata.adl.yaml"
MANUSCRIPT_DIR="$OUT_DIR/manuscript_package"
INPUT_PACKET_DIR="$OUT_DIR/input_packet"
MANIFEST="$OUT_DIR/demo_manifest.json"
README_OUT="$OUT_DIR/README.md"

require_fixture() {
  local path="$1"
  [[ -f "$path" ]] || {
    echo "missing required Paper Sonata fixture: $path" >&2
    exit 1
  }
}

sanitize_generated_artifacts() {
  export ADL_SANITIZE_OUT_DIR="$OUT_DIR"
  export ADL_SANITIZE_OUT_REAL
  ADL_SANITIZE_OUT_REAL="$(cd "$OUT_DIR" && pwd -P)"
  export ADL_SANITIZE_ROOT_DIR="$ROOT_DIR"
  export ADL_SANITIZE_ROOT_REAL
  ADL_SANITIZE_ROOT_REAL="$(cd "$ROOT_DIR" && pwd -P)"
  find "$OUT_DIR" -type f \( -name '*.json' -o -name '*.md' -o -name '*.txt' -o -name '*.yaml' \) -print0 |
    xargs -0 perl -0pi -e '
      for my $name (qw(ADL_SANITIZE_OUT_REAL ADL_SANITIZE_OUT_DIR ADL_SANITIZE_ROOT_REAL ADL_SANITIZE_ROOT_DIR)) {
        my $value = $ENV{$name} // "";
        next if $value eq "";
        my $replacement = $name =~ /ROOT/ ? "<repo_root>" : "<output_dir>";
        s/\Q$value\E/$replacement/g;
      }
    '
}

require_fixture "$FIXTURE_DIR/idea_summary.md"
require_fixture "$FIXTURE_DIR/lab_notes.md"
require_fixture "$FIXTURE_DIR/experiment_results.json"
require_fixture "$FIXTURE_DIR/target_venue.md"
require_fixture "$FIXTURE_DIR/citations_seed.json"
require_fixture "$FIXTURE_DIR/paper_constraints.md"

rm -rf "$OUT_DIR"
mkdir -p "$INPUT_PACKET_DIR" "$MANUSCRIPT_DIR/sections"
cp "$FIXTURE_DIR/"* "$INPUT_PACKET_DIR/"

cd "$ROOT_DIR"

ADL_RUNTIME_ROOT="$RUNTIME_ROOT" \
ADL_RUNS_ROOT="$RUNS_ROOT" \
ADL_MILESTONE="v0.88" \
ADL_DEMO_NAME="paper_sonata" \
  cargo run --quiet --manifest-path adl/Cargo.toml --bin adl -- \
    "$EXAMPLE" \
    --run \
    --trace \
    --allow-unsigned \
    --out "$STEP_OUT" \
    >"$OUT_DIR/run_log.txt" 2>&1

cp "$STEP_OUT/roles/01-conductor-plan.json" "$MANUSCRIPT_DIR/plan.json"
cp "$STEP_OUT/roles/02-conductor-outline.md" "$MANUSCRIPT_DIR/outline.md"
cp "$STEP_OUT/roles/03-scholar-literature-review.md" "$MANUSCRIPT_DIR/literature_review.md"
cp "$STEP_OUT/roles/04-analyst-results-summary.md" "$MANUSCRIPT_DIR/results_summary.md"
cp "$STEP_OUT/roles/09-editor-review-notes.md" "$MANUSCRIPT_DIR/review_notes.md"
cp "$STEP_OUT/roles/05-composer-intro.md" "$MANUSCRIPT_DIR/sections/intro.md"
cp "$STEP_OUT/roles/06-composer-method.md" "$MANUSCRIPT_DIR/sections/method.md"
cp "$STEP_OUT/roles/07-composer-results.md" "$MANUSCRIPT_DIR/sections/results.md"
cp "$STEP_OUT/roles/08-composer-discussion.md" "$MANUSCRIPT_DIR/sections/discussion.md"

cat >"$MANUSCRIPT_DIR/paper_draft.md" <<'EOF'
# Paper Sonata Draft

This manuscript package is assembled from bounded role outputs produced by the
`v0.88` flagship Paper Sonata demo.
EOF

for section in intro method results discussion; do
  printf '\n\n---\n\n' >>"$MANUSCRIPT_DIR/paper_draft.md"
  cat "$MANUSCRIPT_DIR/sections/$section.md" >>"$MANUSCRIPT_DIR/paper_draft.md"
done

python3 - "$MANIFEST" "$RUN_ID" <<'PY'
import json
import sys
from pathlib import Path

manifest_path = Path(sys.argv[1])
run_id = sys.argv[2]
manifest = {
    "schema_version": "adl.paper_sonata_demo.v1",
    "demo_id": "D8",
    "title": "v0.88 Paper Sonata flagship demo",
    "command": "bash adl/tools/demo_v088_paper_sonata.sh",
    "claim": "ADL can orchestrate a bounded manuscript-assembly workflow while preserving explicit role outputs and truthful runtime artifacts.",
    "workflow_shape": {
        "packet_id": "paper_sonata.synthetic_packet.v1",
        "roles": ["conductor", "scholar", "analyst", "composer", "editor"],
        "stage_handoffs": [
            {"from": "conductor", "to": "scholar", "artifact": "manuscript_package/outline.md"},
            {"from": "conductor", "to": "analyst", "artifact": "manuscript_package/plan.json"},
            {"from": "scholar", "to": "composer", "artifact": "manuscript_package/literature_review.md"},
            {"from": "analyst", "to": "composer", "artifact": "manuscript_package/results_summary.md"},
            {"from": "composer", "to": "editor", "artifact": "manuscript_package/paper_draft.md"},
        ],
    },
    "artifacts": {
        "input_packet_dir": "input_packet",
        "role_outputs_dir": "out/roles",
        "plan": "manuscript_package/plan.json",
        "outline": "manuscript_package/outline.md",
        "literature_review": "manuscript_package/literature_review.md",
        "results_summary": "manuscript_package/results_summary.md",
        "review_notes": "manuscript_package/review_notes.md",
        "paper_draft": "manuscript_package/paper_draft.md",
        "sections": {
            "intro": "manuscript_package/sections/intro.md",
            "method": "manuscript_package/sections/method.md",
            "results": "manuscript_package/sections/results.md",
            "discussion": "manuscript_package/sections/discussion.md",
        },
        "runtime": {
            "run_summary": f"runtime/runs/{run_id}/run_summary.json",
            "steps": f"runtime/runs/{run_id}/steps.json",
            "trace": f"runtime/runs/{run_id}/logs/trace_v1.json",
        },
    },
    "reviewer_checks": [
        "inspect manuscript_package/paper_draft.md",
        "inspect out/roles for intermediate artifacts",
        "inspect runtime/runs/v0-88-paper-sonata-demo/run_summary.json",
        "inspect runtime/runs/v0-88-paper-sonata-demo/logs/trace_v1.json",
    ],
}
manifest_path.write_text(json.dumps(manifest, indent=2) + "\n", encoding="utf-8")
PY

cat >"$README_OUT" <<EOF
# v0.88 Demo - Paper Sonata

Canonical command:

\`\`\`bash
bash adl/tools/demo_v088_paper_sonata.sh
\`\`\`

Primary proof surfaces:
- \`demo_manifest.json\`
- \`manuscript_package/paper_draft.md\`
- \`out/roles/\`
- \`runtime/runs/$RUN_ID/run_summary.json\`
- \`runtime/runs/$RUN_ID/logs/trace_v1.json\`

What this proves:
- one bounded synthetic manuscript packet
- one fixed five-role workflow
- durable intermediate role outputs
- a stable manuscript package and truthful runtime evidence
EOF

python3 - "$MANIFEST" "$MANUSCRIPT_DIR/paper_draft.md" "$RUNS_ROOT/$RUN_ID/run_summary.json" "$RUNS_ROOT/$RUN_ID/steps.json" "$RUNS_ROOT/$RUN_ID/logs/trace_v1.json" <<'PY'
import json
import os
import sys

manifest_path, paper_draft, run_summary, steps, trace = sys.argv[1:6]
manifest = json.load(open(manifest_path, encoding="utf-8"))
if manifest.get("schema_version") != "adl.paper_sonata_demo.v1":
    raise SystemExit("unexpected Paper Sonata manifest schema")
for path in (paper_draft, run_summary, steps, trace):
    if not os.path.isfile(path):
        raise SystemExit(f"missing required artifact: {path}")
PY

sanitize_generated_artifacts

echo "Paper Sonata proof surface under the output directory:"
echo "  demo_manifest.json"
echo "  manuscript_package/paper_draft.md"
echo "  out/roles/"
echo "  runtime/runs/$RUN_ID/run_summary.json"
echo "  runtime/runs/$RUN_ID/logs/trace_v1.json"
