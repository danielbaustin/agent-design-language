#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="${1:-$ROOT_DIR/artifacts/v090/adl_architecture_document_generation}"

artifact_label="custom-artifact-root"
case "$OUT_DIR" in
  "$ROOT_DIR"/*)
    artifact_label="${OUT_DIR#"$ROOT_DIR"/}"
    ;;
esac

rm -rf "$OUT_DIR"
mkdir -p "$OUT_DIR"

(
  cd "$ROOT_DIR"
  python3 adl/tools/validate_architecture_docs.py >/dev/null
)

cat >"$OUT_DIR/architecture_generation_manifest.json" <<EOF_JSON
{
  "schema_version": "adl.v090.architecture_document_generation_demo.v1",
  "demo_id": "D9",
  "demo_name": "ADL architecture document generation",
  "classification": "proving",
  "classification_reason": "The tracked architecture packet, diagram packet, automation plan, generation plan, and proof notes are present and pass deterministic validation.",
  "artifact_root": "$artifact_label",
  "docs": [
    "docs/architecture/ADL_ARCHITECTURE.md",
    "docs/architecture/ARCHITECTURE_REVIEW_AUTOMATION.md",
    "docs/architecture/ARCHITECTURE_DOCUMENT_GENERATION_PLAN.md",
    "docs/architecture/diagrams/DIAGRAM_PACKET.md",
    "docs/architecture/adr/CANDIDATE_ADRS.md"
  ],
  "diagram_sources": [
    "docs/architecture/diagrams/system_context.mmd",
    "docs/architecture/diagrams/runtime_lifecycle.mmd",
    "docs/architecture/diagrams/control_plane_lifecycle.mmd",
    "docs/architecture/diagrams/task_bundle_state.mmd",
    "docs/architecture/diagrams/skill_orchestration.mmd",
    "docs/architecture/diagrams/artifact_data_flow.mmd",
    "docs/architecture/diagrams/trust_boundaries.mmd"
  ],
  "skills_represented": [
    "workflow-conductor",
    "pr-run",
    "diagram-author",
    "security-threat-model",
    "repo-architecture-review",
    "architecture-fitness-function-author"
  ],
  "missing_skill_dependencies": [
    "documentation-specialist",
    "gap-analysis"
  ],
  "publication_allowed": false
}
EOF_JSON

cat >"$OUT_DIR/architecture_review_note.md" <<'EOF_MD'
# Architecture Review Note

## Scope

This proof packet reviews the tracked ADL architecture packet and its source
evidence. It does not inspect private worktrees or local session traces.

## Findings

No blocking architecture finding was introduced by this packet. The main design
risk remains process drift: if conductor, lifecycle, specialist, and closeout
skills are bypassed, repository truth can again diverge from GitHub truth.

## Evidence

- `docs/architecture/ADL_ARCHITECTURE.md`
- `docs/default_workflow.md`
- `adl/src/control_plane.rs`
- `adl/src/trace.rs`
- `adl/src/long_lived_agent.rs`

## Residual Risk

The documentation specialist and gap-analysis skills are still backlog
dependencies. Their absence is recorded in the generation plan rather than
hidden by the demo.
EOF_MD

cat >"$OUT_DIR/diagram_review_note.md" <<'EOF_MD'
# Diagram Review Note

## Result

The architecture packet contains seven Mermaid diagram sources. Each source has
an evidence comment and an assumptions comment, and the diagram packet records
purpose, evidence, assumptions, validation, and excluded claims.

## Render Status

The default proof path validates diagram source text. Rendering to SVG is an
optional local step when Mermaid CLI is installed.
EOF_MD

cat >"$OUT_DIR/threat_boundary_note.md" <<'EOF_MD'
# Threat Boundary Note

## Boundaries Reviewed

- ADL document parsing boundary
- Provider transport boundary
- Tool and delegation boundary
- Filesystem sandbox boundary
- Remote execution signature boundary
- Publication and redaction boundary

## Assumption

This is a bounded architecture threat-boundary review, not a full abuse-case
enumeration for every provider or deployment mode.
EOF_MD

cat >"$OUT_DIR/fitness_function_note.md" <<'EOF_MD'
# Fitness Function Note

## Candidate Checks

- Architecture packet file existence.
- Diagram packet coverage for every diagram source.
- Private host-path and secret-marker scan.
- Required architecture sections.
- Demo matrix command truth.
- Worktree-first issue execution in lifecycle docs.

## Human Gates

- Severity assessment.
- Diagram completeness.
- ADR promotion.
- Publication approval.
EOF_MD

(
  cd "$ROOT_DIR"
  python3 adl/tools/validate_architecture_docs.py "$ROOT_DIR" "$OUT_DIR" >/dev/null
)

echo "demo_v090_architecture_document_generation: ok"
