#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="${1:-$ROOT_DIR/artifacts/v088/deep_agents_comparative_proof}"
FILESYSTEM_OUT="$OUT_DIR/filesystem_style_packet"
ADL_OUT="$OUT_DIR/adl_comparative_surface"
PROOF_MD="$OUT_DIR/comparative_proof.md"
MANIFEST="$OUT_DIR/comparative_manifest.json"

rm -rf "$OUT_DIR"
mkdir -p "$FILESYSTEM_OUT" "$ADL_OUT"

cd "$ROOT_DIR"

cat >"$FILESYSTEM_OUT/01-planner-brief.md" <<'EOF'
# Planner Brief

- goal: turn a synthetic research packet into a bounded manuscript package
- stages: scholar -> analyst -> composer -> reviewer
- persistence model: intermediate files in one visible packet
EOF

cat >"$FILESYSTEM_OUT/02-scholar-notes.md" <<'EOF'
# Scholar Notes

- summarize prior work from the bounded packet
- preserve only claims already present in the packet
- emit references to the planner brief rather than restating it in full
EOF

cat >"$FILESYSTEM_OUT/03-analyst-summary.md" <<'EOF'
# Analyst Summary

- supported claim: the packet contains enough evidence for a narrow manuscript draft
- unsupported claim: broad real-world generalization
- dependency: planner brief + scholar notes
EOF

cat >"$FILESYSTEM_OUT/04-composer-draft.md" <<'EOF'
# Composer Draft

This draft assembles the packet into a coherent bounded manuscript skeleton.
It references the scholar notes and analyst summary rather than re-embedding all state.
EOF

cat >"$FILESYSTEM_OUT/05-review-notes.md" <<'EOF'
# Review Notes

- preserve intermediate files
- reject unsupported claims
- require provenance for every stage output
- keep the workflow bounded and inspectable
EOF

cat >"$ADL_OUT/provenance_manifest.json" <<'EOF'
{
  "schema_version": "adl.provenance_manifest.v1",
  "artifacts": [
    {
      "artifact_id": "planner_brief",
      "path": "filesystem_style_packet/01-planner-brief.md",
      "stage": "planner",
      "derived_from": []
    },
    {
      "artifact_id": "scholar_notes",
      "path": "filesystem_style_packet/02-scholar-notes.md",
      "stage": "scholar",
      "derived_from": ["planner_brief"]
    },
    {
      "artifact_id": "analyst_summary",
      "path": "filesystem_style_packet/03-analyst-summary.md",
      "stage": "analyst",
      "derived_from": ["planner_brief", "scholar_notes"]
    },
    {
      "artifact_id": "composer_draft",
      "path": "filesystem_style_packet/04-composer-draft.md",
      "stage": "composer",
      "derived_from": ["scholar_notes", "analyst_summary"]
    },
    {
      "artifact_id": "review_notes",
      "path": "filesystem_style_packet/05-review-notes.md",
      "stage": "reviewer",
      "derived_from": ["composer_draft"]
    }
  ]
}
EOF

cat >"$ADL_OUT/reference_map.json" <<'EOF'
{
  "schema_version": "adl.reference_map.v1",
  "externalized_state_principle": "No large state should be embedded if it can be referenced.",
  "references": [
    {
      "consumer": "scholar_notes",
      "references": ["planner_brief"]
    },
    {
      "consumer": "analyst_summary",
      "references": ["planner_brief", "scholar_notes"]
    },
    {
      "consumer": "composer_draft",
      "references": ["scholar_notes", "analyst_summary"]
    },
    {
      "consumer": "review_notes",
      "references": ["composer_draft"]
    }
  ]
}
EOF

cat >"$ADL_OUT/reviewer_checklist.md" <<'EOF'
# Reviewer Checklist

- Can I inspect the role outputs directly?
- Can I see which artifacts depend on which earlier artifacts?
- Can I tell whether later stages referenced earlier state instead of re-embedding it opaquely?
- Can I audit the bounded claim without relying on hidden prompt history?
EOF

cat >"$PROOF_MD" <<EOF
# v0.88 Deep Agents Comparative Proof

## Command

\`\`\`bash
bash adl/tools/demo_v088_deep_agents_comparative_proof.sh
\`\`\`

## Reviewer Claim

This proof packet shows the bounded v0.88 comparative claim:
ADL is stronger than a visible-files-only deep-agent workflow because it adds explicit
provenance, reference-based state handling, and a reviewer-oriented audit surface.

## Filesystem-Style Deep-Agent Surface

- role-separated outputs in a visible packet
- persistence outside the prompt
- inspectable intermediate files

## ADL Comparative Surface

- the same visible packet
- explicit provenance manifest
- explicit reference map for externalized state
- reviewer checklist explaining what can be audited
- a cleaner distinction between raw artifacts and the review surface over them

## Primary Artifacts

- filesystem-style packet: \`filesystem_style_packet/\`
- provenance manifest: \`adl_comparative_surface/provenance_manifest.json\`
- reference map: \`adl_comparative_surface/reference_map.json\`
- reviewer checklist: \`adl_comparative_surface/reviewer_checklist.md\`

## Relationship To v0.88

This is a supporting WP-13 comparative proof surface.
It strengthens the reviewer story around Paper Sonata without replacing it as the flagship demo.
Paper Sonata remains the real flagship runtime workflow. This packet is the explanation surface
for why ADL's artifact-and-reference model matters.
EOF

cat >"$MANIFEST" <<EOF
{
  "schema_version": "adl.deep_agents_comparative_proof.v1",
  "demo_id": "D9",
  "title": "v0.88 deep-agents comparative proof",
  "command": "bash adl/tools/demo_v088_deep_agents_comparative_proof.sh",
  "claim": "ADL turns a visible file-based deep-agent packet into a reviewer-auditable comparative proof surface.",
  "comparative_dimensions": [
    "artifact_visibility",
    "externalized_state",
    "artifact_provenance",
    "review_surface_clarity"
  ],
  "artifacts": {
    "comparative_proof": "comparative_proof.md",
    "filesystem_packet": "filesystem_style_packet/",
    "provenance_manifest": "adl_comparative_surface/provenance_manifest.json",
    "reference_map": "adl_comparative_surface/reference_map.json",
    "reviewer_checklist": "adl_comparative_surface/reviewer_checklist.md"
  }
}
EOF

python3 - "$MANIFEST" "$PROOF_MD" "$ADL_OUT/provenance_manifest.json" "$ADL_OUT/reference_map.json" "$ADL_OUT/reviewer_checklist.md" "$FILESYSTEM_OUT/01-planner-brief.md" "$FILESYSTEM_OUT/05-review-notes.md" <<'PY'
import json
import os
import sys

manifest_path, proof_md, provenance, reference_map, checklist, planner_brief, review_notes = sys.argv[1:8]
manifest = json.load(open(manifest_path, encoding="utf-8"))
if manifest.get("schema_version") != "adl.deep_agents_comparative_proof.v1":
    raise SystemExit("unexpected comparative proof manifest schema")
for path in (proof_md, provenance, reference_map, checklist, planner_brief, review_notes):
    if not os.path.isfile(path):
        raise SystemExit(f"missing required artifact: {path}")
PY

echo "Deep-agents comparative proof surface under the output directory:"
echo "  comparative_proof.md"
echo "  comparative_manifest.json"
echo "  filesystem_style_packet/"
echo "  adl_comparative_surface/provenance_manifest.json"
echo "  adl_comparative_surface/reference_map.json"
echo "  adl_comparative_surface/reviewer_checklist.md"
