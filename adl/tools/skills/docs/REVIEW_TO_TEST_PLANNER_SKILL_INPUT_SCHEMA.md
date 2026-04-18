# Review To Test Planner Skill Input Schema

Schema id: `review_to_test_planner.v1`

This schema describes structured input accepted by the
`review-to-test-planner` skill. It is bounded so CodeBuddy can convert review
findings into test-generation briefs without writing tests, creating issues,
opening PRs, or mutating customer repositories.

## Required Shape

```yaml
skill_input_schema: review_to_test_planner.v1
mode: plan_from_review_packet | plan_from_specialist_artifacts | plan_from_synthesis | plan_from_findings_file
repo_root: /absolute/path
target:
  review_packet_path: <path or null>
  specialist_artifacts:
    code: <path or null>
    security: <path or null>
    tests: <path or null>
    docs: <path or null>
    architecture: <path or null>
    dependency: <path or null>
    synthesis: <path or null>
  synthesis_artifact: <path or null>
  findings_file: <path or null>
  artifact_root: <path or null>
  diff_base: <string or null>
  changed_paths:
    - <path>
policy:
  test_depth: focused | moderate
  validation_mode: targeted | inspect_only | none
  allow_handoff_generation: true | false
  unsafe_task_policy: mark_unsafe | skip
  write_plan_artifact: true | false
  stop_after_plan: true
```

## Required Fields

- `skill_input_schema` must equal `review_to_test_planner.v1`.
- `mode` must be one of the supported planning modes.
- `repo_root` must be absolute.
- `target` must identify one bounded review packet, specialist artifact set,
  synthesis artifact, or findings file.
- `policy.stop_after_plan` must be `true`.
- `mode: plan_from_review_packet` requires `target.review_packet_path`.
- `mode: plan_from_findings_file` requires `target.findings_file`.

## Output Contract

The skill writes or returns a planning artifact with:

- findings-to-test map
- generation-status summary
- test task briefs
- fixture and assertion map
- validation command plan
- test-generator handoffs
- deferred and unsafe tasks
- validation performed
- residual risk

The skill may also generate deterministic scaffolding with
`scripts/plan_review_tests.py`.

## Stop Boundary

The skill must not:

- write tests, fixtures, snapshots, or production code
- mutate the reviewed repository
- create issues or PRs
- invoke `test-generator` automatically
- replace review specialists, synthesis, finding-to-issue planning, or
  implementation workflow
- claim tests exist when only planning artifacts were produced
