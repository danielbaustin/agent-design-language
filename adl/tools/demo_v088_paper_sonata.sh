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
PACKET_MANIFEST="$INPUT_PACKET_DIR/packet_manifest.json"

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

python3 - "$PACKET_MANIFEST" <<'PY'
import json
import sys
from pathlib import Path

manifest = {
    "schema_version": "paper_sonata.packet_manifest.v2",
    "packet_id": "paper_sonata.synthetic_packet.v1",
    "inputs": [
        "idea_summary.md",
        "lab_notes.md",
        "experiment_results.json",
        "target_venue.md",
        "citations_seed.json",
        "paper_constraints.md",
    ],
    "notes": [
        "synthetic bounded packet only",
        "no live-web citation claims",
        "no publication-readiness guarantee",
    ],
}
Path(sys.argv[1]).write_text(json.dumps(manifest, indent=2) + "\n", encoding="utf-8")
PY

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

cat >"$MANUSCRIPT_DIR/abstract.md" <<'EOF'
# Abstract

Paper Sonata demonstrates a bounded manuscript-assembly workflow in which five
explicit roles transform one synthetic research packet into a reviewer-legible
paper package. The claim is narrow and inspectable: explicit anchors and
retained intermediate artifacts make manuscript assembly easier to review and
replay without overclaiming autonomous scientific discovery.
EOF

cat >"$MANUSCRIPT_DIR/reviewer_brief.md" <<'EOF'
# Reviewer Brief

Review this package in the following order:

1. `input_packet/packet_manifest.json`
2. `manuscript_package/abstract.md`
3. `manuscript_package/paper_draft.md`
4. `manuscript_package/claim_matrix.md`
5. `manuscript_package/revision_requests.json`
6. `out/roles/`
7. `runtime/runs/v0-88-paper-sonata-demo/run_summary.json`

This demo proves bounded manuscript assembly with truthful runtime evidence. It
does not claim publication readiness or autonomous scientific discovery.
EOF

python3 - "$FIXTURE_DIR/experiment_results.json" "$MANUSCRIPT_DIR/claim_matrix.md" "$MANUSCRIPT_DIR/figures_spec.json" "$MANUSCRIPT_DIR/revision_requests.json" <<'PY'
import json
import sys
from pathlib import Path

results_path, claim_matrix_path, figures_path, revisions_path = [Path(arg) for arg in sys.argv[1:]]
results = json.loads(results_path.read_text(encoding="utf-8"))
metrics = results["metrics"]
supported = results["supported_claim"]
unsupported = results["unsupported_claim"]

claim_matrix = f"""# Claim Matrix

## Supported Claim
- Claim: {supported}
- Evidence:
  - dropped handoff fields improved from {metrics['dropped_handoff_fields_baseline']} to {metrics['dropped_handoff_fields_anchored']}
  - reviewer repair time improved from {metrics['reviewer_minutes_baseline']} to {metrics['reviewer_minutes_anchored']} minutes
  - replay consistency score remained {metrics['replay_consistency_score']:.2f}

## Unsupported Claim
- Claim: {unsupported}
- Reason:
  - the packet is synthetic and bounded
  - there is no evidence for general scientific autonomy
"""
claim_matrix_path.write_text(claim_matrix, encoding="utf-8")

figures = {
    "schema_version": "paper_sonata.figures_spec.v1",
    "figures": [
        {
            "figure_id": "fig-01",
            "title": "Anchored handoff completeness",
            "chart": "bar",
            "x": ["baseline", "anchored"],
            "y": [
                metrics["dropped_handoff_fields_baseline"],
                metrics["dropped_handoff_fields_anchored"],
            ],
            "y_label": "Dropped handoff fields",
        },
        {
            "figure_id": "fig-02",
            "title": "Reviewer repair time",
            "chart": "bar",
            "x": ["baseline", "anchored"],
            "y": [
                metrics["reviewer_minutes_baseline"],
                metrics["reviewer_minutes_anchored"],
            ],
            "y_label": "Minutes",
        },
    ],
}
figures_path.write_text(json.dumps(figures, indent=2) + "\n", encoding="utf-8")

revisions = {
    "schema_version": "paper_sonata.revision_requests.v1",
    "requests": [
        {
            "id": "rev-01",
            "priority": "high",
            "request": "Keep the framing on bounded manuscript assembly, not discovery.",
            "source": "editor.review_notes",
        },
        {
            "id": "rev-02",
            "priority": "medium",
            "request": "Keep unsupported-claim caveats visible in reviewer-facing artifacts.",
            "source": "analyst.results_summary",
        },
        {
            "id": "rev-03",
            "priority": "medium",
            "request": "Show the packet manifest and trace bundle beside the draft package.",
            "source": "editor.review_notes",
        },
    ],
}
revisions_path.write_text(json.dumps(revisions, indent=2) + "\n", encoding="utf-8")
PY

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
    "schema_version": "adl.paper_sonata_demo.v2",
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
        "packet_manifest": "input_packet/packet_manifest.json",
        "role_outputs_dir": "out/roles",
        "abstract": "manuscript_package/abstract.md",
        "plan": "manuscript_package/plan.json",
        "outline": "manuscript_package/outline.md",
        "literature_review": "manuscript_package/literature_review.md",
        "results_summary": "manuscript_package/results_summary.md",
        "claim_matrix": "manuscript_package/claim_matrix.md",
        "figures_spec": "manuscript_package/figures_spec.json",
        "revision_requests": "manuscript_package/revision_requests.json",
        "reviewer_brief": "manuscript_package/reviewer_brief.md",
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
- \`input_packet/packet_manifest.json\`
- \`manuscript_package/reviewer_brief.md\`
- \`manuscript_package/claim_matrix.md\`
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
if manifest.get("schema_version") != "adl.paper_sonata_demo.v2":
    raise SystemExit("unexpected Paper Sonata manifest schema")
for path in (paper_draft, run_summary, steps, trace):
    if not os.path.isfile(path):
        raise SystemExit(f"missing required artifact: {path}")
PY

sanitize_generated_artifacts

echo "Paper Sonata proof surface under the output directory:"
echo "  demo_manifest.json"
echo "  input_packet/packet_manifest.json"
echo "  manuscript_package/reviewer_brief.md"
echo "  manuscript_package/claim_matrix.md"
echo "  manuscript_package/paper_draft.md"
echo "  out/roles/"
echo "  runtime/runs/$RUN_ID/run_summary.json"
echo "  runtime/runs/$RUN_ID/logs/trace_v1.json"
