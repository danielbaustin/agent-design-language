# Output Contract

The architecture diagram reviewer skill produces a specialist diagram review
artifact for CodeBuddy-style repository review. It does not author diagram
source or render visual assets.

Default artifact path:

```text
.adl/reviews/<timestamp>-architecture-diagram-review.md
```

Optional scaffold artifact root:

```text
.adl/reviews/codebuddy/<run_id>/architecture-diagram-review/
```

## Required Markdown Sections

### Metadata

Required fields:

- `Skill: architecture-diagram-reviewer`
- `Target`
- `Date`
- `Artifact`
- `Packet`

### Findings

Findings must come first after metadata.

Each finding must include:

- priority
- title
- diagram file or artifact
- role: `architecture-diagram`
- scenario
- unsupported, stale, missing, or unrenderable claim
- evidence
- impact
- recommended follow-up owner

If no material findings are found, state that explicitly and include residual
risk.

### Reviewed Diagrams

List bounded diagram source files, packets, or rendered artifacts reviewed.

### Evidence Coverage Map

Map diagram files to source evidence paths. Use packet-relative or repo-relative
paths.

### Unsupported Claim Checks

Record any nodes, edges, labels, trust boundaries, data flows, lifecycle steps,
or dependencies that lack source evidence.

### Missing Component Checks

Record high-signal evidence surfaces that may need representation or explicit
out-of-scope notes.

### Renderer Status Checks

Record whether source, SVG, PNG, or other rendered artifacts exist. Do not claim
rendering succeeded unless evidence exists or a render command was run.

### Accessibility And Readability Notes

Check title, caption, legend, label readability, layout clarity, and audience fit.

### Correction Handoffs

List bounded handoffs to `diagram-author`, `repo-diagram-planner`,
`repo-architecture-review`, or other specialists.

### Validation Performed

List commands and what they proved, or explain why validation was inspect-only.

### Residual Risk

Explain skipped sources, unparsed diagram syntax, missing rendered assets, or
review limits.

## JSON Scaffold Contract

`scripts/review_architecture_diagrams.py` emits
`architecture_diagram_review_scaffold.json` with:

- `schema`
- `repo_name`
- `packet_root`
- `diagram_root`
- `reviewed_diagrams`
- `evidence_coverage_map`
- `unsupported_claim_checks`
- `missing_component_checks`
- `renderer_status_checks`
- `correction_handoffs`
- `notes`

It also emits `architecture_diagram_review_scaffold.md` with the Markdown
headings needed by the specialist review artifact.

## Rules

- Use repo-relative or packet-relative paths.
- Do not write absolute host paths into artifacts.
- Do not author or edit diagram source.
- Do not render diagrams.
- Do not publish diagrams.
- Do not mutate the reviewed repository.
- Do not claim diagram correctness without source evidence.

