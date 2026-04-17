# Output Contract

The repo architecture review skill produces a specialist architecture review
artifact for CodeBuddy-style multi-agent review.

Default artifact path:

```text
.adl/reviews/<timestamp>-repo-architecture-review.md
```

Optional scaffold artifact root:

```text
.adl/reviews/codebuddy/<run_id>/architecture-review/
```

## Required Markdown Sections

### Metadata

Required fields:

- `Skill: repo-architecture-review`
- `Target`
- `Date`
- `Artifact`
- `Packet`

### Findings

Findings must come first after metadata.

Each finding must include:

- priority
- title
- file or evidence path
- role: `architecture`
- scenario
- architecture boundary or layer
- impact
- evidence
- recommended follow-up owner

If no material findings are found, state that explicitly and include residual
risk.

### Architecture Map

Summarize:

- top-level modules or packages
- primary entrypoints
- runtime boundaries
- state or persistence boundaries
- integration boundaries
- docs or diagrams consulted

### Reviewed Surfaces

List bounded source-grounded surfaces by repo-relative path or packet-relative
artifact path.

### Candidate Diagram Tasks

List candidate handoffs for `diagram-author`, or state that none were found.

### Candidate ADRs

List candidate architecture decisions that should be captured by an ADR writer
or curator, or state that none were found.

### Candidate Fitness Functions

List executable architecture guard candidates, or state that none were found.

### Validation Performed

List commands and what they proved, or explain why validation was inspect-only.

### Residual Risk

Explain skipped surfaces, missing packet evidence, unexecuted commands, or
review limits.

## JSON Scaffold Contract

`scripts/prepare_architecture_review.py` emits
`architecture_review_scaffold.json` with:

- `schema`
- `repo_name`
- `packet_root`
- `architecture_evidence`
- `candidate_diagram_tasks`
- `candidate_adr_topics`
- `candidate_fitness_functions`
- `notes`

It also emits `architecture_review_scaffold.md` with the Markdown headings
needed by the specialist review artifact.

## Rules

- Use repo-relative or packet-relative paths.
- Do not write absolute host paths into artifacts.
- Do not author diagrams, ADRs, fitness-function code, issues, or synthesis.
- Do not mutate the reviewed repository.
- Preserve architecture findings even when code/security/tests/docs specialists
  have not reviewed the same surface.

