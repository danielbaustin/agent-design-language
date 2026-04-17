# Repo Diagram Planner Skill Input Schema

Schema id: `repo_diagram_planner.v1`

This schema describes the structured input accepted by the
`repo-diagram-planner` skill. It is intentionally bounded so CodeBuddy can plan
diagram work without authoring diagrams, rendering assets, publishing, or
mutating customer repositories.

## Required Shape

```yaml
skill_input_schema: repo_diagram_planner.v1
mode: plan_from_review_packet | plan_from_specialist_artifacts | plan_from_path | plan_from_issue | refresh_diagram_plan
repo_root: /absolute/path
target:
  review_packet_path: <path or null>
  specialist_artifacts:
    architecture: <path or null>
    security: <path or null>
    dependency: <path or null>
    docs: <path or null>
  target_path: <path or null>
  issue_number: <number or null>
  doc_path: <path or null>
  artifact_root: <path or null>
policy:
  audience: reviewers | maintainers | operators | users | mixed
  diagram_goals:
    - orientation
    - architecture_boundaries
    - workflow
    - state
    - data_flow
    - dependencies
    - responsibility_map
  max_tasks: 1
  preferred_backends:
    - mermaid
    - structurizr
    - d2
    - plantuml
  write_plan_artifact: true | false
  stop_after_plan: true
```

## Required Fields

- `skill_input_schema` must equal `repo_diagram_planner.v1`.
- `mode` must be one of the supported planning modes.
- `repo_root` must be absolute.
- `target` must identify one bounded packet, artifact set, path, issue, or doc
  target.
- `policy.stop_after_plan` must be `true`.
- `mode: plan_from_review_packet` requires `target.review_packet_path`.

## Output Contract

The skill writes or returns a source-grounded diagram planning artifact with:

- diagram tasks
- source evidence map
- family and backend rationale
- assumptions and unknowns
- blocked or skipped candidates
- validation performed
- diagram-author handoff instructions
- residual risk

The skill may also generate deterministic scaffolding with
`scripts/plan_repo_diagrams.py`.

## Stop Boundary

The skill must not:

- author Mermaid, D2, PlantUML, Structurizr, SVG, PNG, or raster artifacts
- render diagrams
- publish diagrams
- mutate the reviewed repository
- create issues or PRs
- replace `diagram-author` or specialist review skills
- invent architecture, runtime behavior, dependency direction, trust boundaries,
  data flows, or actor responsibilities

