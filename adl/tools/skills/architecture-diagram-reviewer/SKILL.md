---
name: architecture-diagram-reviewer
description: Review architecture diagram packets for CodeBuddy-style repository reviews against source evidence, diagram plans, architecture findings, assumptions, unknowns, renderer status, labels, nodes, edges, and unsupported claims without authoring diagrams, rendering assets, or mutating repositories.
---

# Architecture Diagram Reviewer

Review finished or draft architecture diagram packets for truth, completeness,
and usability. This skill is a quality gate after `diagram-author`, not a
diagram generation skill.

Use this skill when a CodeBuddy review needs a source-grounded check of diagram
claims, diagram source, rendered status, labels, assumptions, unknowns, and
handoff corrections before a diagram is included in a report or customer-facing
packet.

## Quick Start

1. Confirm the bounded review target:
   - diagram packet
   - diagram source directory
   - rendered artifact directory
   - CodeBuddy review packet
   - specialist architecture or diagram plan artifact
2. Prefer CodeBuddy packet artifacts when available:
   - `evidence_index.json`
   - `repo_inventory.json`
   - `repo_scope.md`
   - `specialist_assignments.json`
3. Run the deterministic review helper when local access is available:
   - `scripts/review_architecture_diagrams.py <packet-root> <diagram-root> --out <artifact-root>`
4. Inspect the generated scaffold and write findings-first review output.
5. Hand corrections back to `repo-diagram-planner` or `diagram-author`. Stop
   before editing diagram source, rendering, publishing, or opening issues.

## Focus

Prioritize:

- unsupported nodes, edges, labels, or claims
- missing major components that are present in evidence and relevant to the
  stated diagram goal
- stale component names, lifecycle names, or backend labels
- hidden assumptions or unlabeled unknowns
- unrenderable diagram source or renderer status overclaims
- visually misleading structure, such as direction arrows that imply unsupported
  dependency or trust-boundary relationships
- missing accessibility basics, such as title, caption, legend, and readable
  labels
- mismatch between diagram family/backend and intended audience

Defer primary ownership of these areas:

- selecting diagram candidates: `repo-diagram-planner`
- authoring diagram source or rendering: `diagram-author`
- architecture findings: `repo-architecture-review`
- docs truth drift: `repo-review-docs`
- security trust-boundary findings: `repo-review-security`
- final report synthesis: `repo-review-synthesis` or a report writer

## Required Inputs

At minimum, gather:

- `repo_root`
- one concrete target:
  - `target.diagram_packet_path`
  - `target.diagram_source_path`
  - `target.rendered_artifact_path`
  - `target.review_packet_path`

Useful additional inputs:

- `diagram_plan_path`
- `architecture_review_artifact`
- `artifact_root`
- `audience`
- `validation_mode`
- `renderer_status`
- `expected_diagram_family`
- `forbidden_claims`

If there is no bounded diagram target, stop and report `blocked`.

## Workflow

### 1. Establish Scope

Record:

- diagram packet or source paths reviewed
- packet evidence consulted
- diagram family and backend
- rendered artifacts checked or not checked
- assumptions and known limits

Do not silently expand a single diagram review into a whole-repo diagram audit.

### 2. Compare Diagram To Evidence

Check whether:

- nodes map to source evidence or are explicitly marked as assumptions
- edges and arrows are supported by code, docs, architecture review, or diagram
  plan evidence
- labels match current repo terms and do not overclaim behavior
- omitted high-signal components are intentional or documented as out of scope
- trust boundaries, data flows, and dependency direction are not guessed
- renderer status is truthful

### 3. Review For Findings

Use this priority scale:

- `P0`: diagram can cause unsafe operational, security, or customer-facing
  misunderstanding in a high-risk context
- `P1`: diagram materially misrepresents architecture, lifecycle, trust
  boundary, dependency direction, or rendered status
- `P2`: diagram omits important evidence, uses stale labels, or hides
  assumptions likely to mislead reviewers
- `P3`: useful diagram quality issue with bounded correction value

Each finding should include:

- affected diagram file or rendered artifact
- unsupported or missing claim
- source evidence
- impact
- recommended follow-up owner

### 4. Emit Corrections

Recommend bounded corrections, but do not perform them:

- update diagram plan: `repo-diagram-planner`
- revise diagram source or render: `diagram-author`
- re-check architecture evidence: `repo-architecture-review`
- re-check trust-boundary claim: `repo-review-security`

## Output Expectations

Default output should include:

- findings first
- reviewed diagrams
- evidence coverage map
- unsupported claim checks
- missing component checks
- renderer status checks
- accessibility and readability notes
- correction handoffs
- validation performed or not run
- residual diagram risk

Use `references/output-contract.md` and the shared suite contract in
`adl/tools/skills/docs/MULTI_AGENT_REPO_REVIEW_SKILL_SUITE.md`.

## Stop Boundary

Stop after producing the diagram review artifact.

Do not:

- author or edit Mermaid, D2, PlantUML, Structurizr, SVG, PNG, or raster assets
- render diagrams
- publish diagrams
- mutate customer repositories
- create issues or PRs
- replace `diagram-author`, `repo-diagram-planner`, architecture review, security
  review, docs review, or synthesis
- claim the diagram is correct unless source evidence and renderer status were
  actually checked

## CodeBuddy Integration Notes

This skill consumes CodeBuddy packet artifacts, diagram plans, and diagram
packets. It produces a specialist diagram review artifact that can feed report
writing, synthesis, or a bounded `diagram-author` correction pass.

Deferred automation:

- Diagram source parsing by backend.
- Rendered SVG/PNG visual inspection and accessibility scoring.
- Direct comparison with `repo-diagram-planner` task ids.
- Optional renderer dry-run delegation through `diagram-author`.

