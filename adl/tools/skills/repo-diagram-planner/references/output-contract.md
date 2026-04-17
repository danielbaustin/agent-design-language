# Output Contract

The repo diagram planner skill produces a bounded diagram planning artifact for
CodeBuddy-style multi-agent review. It does not author diagram source or render
visual assets.

Default artifact path:

```text
.adl/reviews/<timestamp>-repo-diagram-plan.md
```

Optional scaffold artifact root:

```text
.adl/reviews/codebuddy/<run_id>/diagram-plan/
```

## Required Markdown Sections

### Metadata

Required fields:

- `Skill: repo-diagram-planner`
- `Target`
- `Date`
- `Artifact`
- `Packet`

### Diagram Tasks

Each task must include:

- task id
- diagram family
- suggested backend
- audience
- communication goal
- source evidence paths
- assumptions
- unknowns
- claims not allowed
- renderer expectation
- handoff target: `diagram-author`

### Source Evidence Map

List packet-relative or repo-relative evidence paths used to justify each task.

### Family / Backend Rationale

Explain why each diagram family and backend was chosen. Prefer Mermaid for
GitHub-friendly review surfaces unless the evidence calls for Structurizr, D2,
PlantUML, or markdown-only planning.

### Assumptions And Unknowns

Separate source-backed structure from assumptions and unknowns.

### Blocked Or Skipped Candidates

Record diagram ideas that were skipped because evidence was insufficient,
duplicated, or outside scope.

### Validation Performed

List commands and what they proved, or explain why validation was inspect-only.

### Diagram Author Handoff

Give the exact bounded brief that should be handed to `diagram-author` for each
selected task.

### Residual Risk

Explain skipped sources, missing packet evidence, unexecuted rendering checks, or
review limits.

## JSON Plan Contract

`scripts/plan_repo_diagrams.py` emits `repo_diagram_plan.json` with:

- `schema`
- `repo_name`
- `packet_root`
- `diagram_tasks`
- `source_evidence_map`
- `blocked_or_skipped_candidates`
- `notes`

It also emits `repo_diagram_plan.md` with the Markdown headings needed by the
planning artifact.

## Rules

- Use repo-relative or packet-relative paths.
- Do not write absolute host paths into artifacts.
- Do not author diagram source or rendered diagram assets.
- Do not publish diagrams.
- Do not mutate the reviewed repository.
- Do not invent architecture, runtime behavior, dependencies, trust boundaries,
  data flows, or actor responsibilities that are not backed by evidence.
- Preserve diagram planning tasks even when synthesis has not yet run.

