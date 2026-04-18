# Architecture Diagram Reviewer Skill Input Schema

Schema id: `architecture_diagram_reviewer.v1`

This schema describes structured input accepted by the
`architecture-diagram-reviewer` skill. It is bounded so CodeBuddy can check
diagram truth and quality without authoring diagrams, rendering assets,
publishing, or mutating customer repositories.

## Required Shape

```yaml
skill_input_schema: architecture_diagram_reviewer.v1
mode: review_diagram_packet | review_diagram_source | review_rendered_artifacts | review_diagram_against_packet
repo_root: /absolute/path
target:
  diagram_packet_path: <path or null>
  diagram_source_path: <path or null>
  rendered_artifact_path: <path or null>
  review_packet_path: <path or null>
  diagram_plan_path: <path or null>
  architecture_review_artifact: <path or null>
  artifact_root: <path or null>
policy:
  audience: reviewers | maintainers | operators | users | mixed
  validation_mode: targeted | inspect_only | none
  renderer_status_required: true | false
  write_review_artifact: true | false
  stop_after_review: true
```

## Required Fields

- `skill_input_schema` must equal `architecture_diagram_reviewer.v1`.
- `mode` must be one of the supported review modes.
- `repo_root` must be absolute.
- `target` must identify one bounded diagram packet, diagram source, rendered
  artifact, or review packet target.
- `policy.stop_after_review` must be `true`.
- `mode: review_diagram_against_packet` requires `target.review_packet_path`.

## Output Contract

The skill writes or returns a findings-first diagram review artifact with:

- findings
- reviewed diagrams
- evidence coverage map
- unsupported claim checks
- missing component checks
- renderer status checks
- accessibility and readability notes
- correction handoffs
- validation performed
- residual risk

The skill may also generate deterministic scaffolding with
`scripts/review_architecture_diagrams.py`.

## Stop Boundary

The skill must not:

- author or edit Mermaid, D2, PlantUML, Structurizr, SVG, PNG, or raster assets
- render diagrams
- publish diagrams
- mutate the reviewed repository
- create issues or PRs
- replace `diagram-author`, `repo-diagram-planner`, or specialist review skills
- claim diagram correctness without source evidence and renderer truth

