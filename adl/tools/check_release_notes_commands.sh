#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR/adl"

release_notes="../docs/milestones/v0.89/RELEASE_NOTES_v0.89.md"
demo_matrix_ref='docs/milestones/v0.89/DEMO_MATRIX_v0.89.md'
tag_ref='Tag: `v0.89`'
quality_gate_ref='docs/milestones/v0.89/QUALITY_GATE_v0.89.md'
draft_status_ref='this document is pre-release and should not be treated as a shipped-claims document yet'
quality_gate_note_ref='demo/review package and quality-gate outputs should be cited before release'
v02_release_notes="../docs/milestones/v0.2/RELEASE_NOTES_v0.2.md"
v02_run_dir_ref='Run from the `adl/` directory.'
v02_plan_cmd='cargo run -- examples/v0-2-coordinator-agents-sdk.adl.yaml --print-plan'
v02_examples_ref='adl/examples/README.md'
v02_walkthrough_ref='adl/examples/v0-2-coordinator-agents-sdk.md'

if ! grep -Fq "$demo_matrix_ref" "$release_notes"; then
  echo "missing demo-matrix reference in $release_notes: $demo_matrix_ref" >&2
  exit 1
fi

if ! grep -Fq "$tag_ref" "$release_notes"; then
  echo "missing tag reference in $release_notes: $tag_ref" >&2
  exit 1
fi

if ! grep -Fq "$draft_status_ref" "$release_notes"; then
  echo "missing pre-release draft status note in $release_notes: $draft_status_ref" >&2
  exit 1
fi

if ! grep -Fq "$quality_gate_note_ref" "$release_notes"; then
  echo "missing quality-gate note in $release_notes: $quality_gate_note_ref" >&2
  exit 1
fi

if [ ! -f "../docs/milestones/v0.89/QUALITY_GATE_v0.89.md" ]; then
  echo "missing canonical quality gate doc: $quality_gate_ref" >&2
  exit 1
fi

if ! grep -Fq "$v02_run_dir_ref" "$v02_release_notes"; then
  echo "missing adl runtime-directory note in $v02_release_notes: $v02_run_dir_ref" >&2
  exit 1
fi

if ! grep -Fq "$v02_plan_cmd" "$v02_release_notes"; then
  echo "missing current adl plan command in $v02_release_notes: $v02_plan_cmd" >&2
  exit 1
fi

if grep -Fq 'cargo run --bin swarm --' "$v02_release_notes"; then
  echo "obsolete swarm binary command still present in $v02_release_notes" >&2
  exit 1
fi

if ! grep -Fq "$v02_examples_ref" "$v02_release_notes"; then
  echo "missing current examples index reference in $v02_release_notes: $v02_examples_ref" >&2
  exit 1
fi

if ! grep -Fq "$v02_walkthrough_ref" "$v02_release_notes"; then
  echo "missing current walkthrough reference in $v02_release_notes: $v02_walkthrough_ref" >&2
  exit 1
fi

echo "release-notes command check: ok"
