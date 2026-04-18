# Architecture Fitness Function Author Skill Input Schema

Schema id: `architecture_fitness_function_author.v1`

This schema describes structured input accepted by the
`architecture-fitness-function-author` skill. It is bounded so CodeBuddy can
author reviewable fitness-function specs without editing tests, CI, docs,
policies, issues, PRs, or customer repository files.

## Required Shape

```yaml
skill_input_schema: architecture_fitness_function_author.v1
mode: author_from_review_packet | author_from_architecture_review | author_from_findings_file | author_from_path
repo_root: /absolute/path
target:
  review_packet_path: <path or null>
  architecture_review_artifact: <path or null>
  findings_file: <path or null>
  target_path: <path or null>
  artifact_root: <path or null>
policy:
  allowed_check_types:
    - dependency_rule
    - forbidden_import
    - contract_test
    - docs_check
    - ci_gate
    - repo_policy_check
  validation_mode: targeted | inspect_only | none
  implementation_allowed: false
  ci_gate_allowed: true | false
  write_plan_artifact: true | false
  stop_after_plan: true
```

## Required Fields

- `skill_input_schema` must equal
  `architecture_fitness_function_author.v1`.
- `mode` must be one of the supported authoring modes.
- `repo_root` must be absolute.
- `target` must identify one bounded review packet, architecture review
  artifact, findings file, or target path.
- `policy.stop_after_plan` must be `true`.
- `mode: author_from_review_packet` requires `target.review_packet_path`.
- `mode: author_from_findings_file` requires `target.findings_file`.

## Output Contract

The skill writes or returns a fitness-function plan with:

- fitness-function catalog
- machine-checkable invariants
- human-judgment candidates
- deferred automation boundaries
- validation command plan
- expected failure modes
- implementation handoffs
- validation performed
- residual risk

The skill may also generate deterministic scaffolding with
`scripts/author_architecture_fitness_functions.py`.

## Stop Boundary

The skill must not:

- edit tests, CI, docs, policy files, production code, issues, or PRs
- mutate customer repositories
- run network or paid services
- claim a check is installed when only a plan exists
- replace architecture review, ADR writing, test generation, or implementation
  workflow
