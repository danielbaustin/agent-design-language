# Demo Operator Skill Input Schema

Schema id: `demo_operator.v1`

## Purpose

Provide one structured invocation shape for the bounded `demo-operator` skill.

The skill should run one named demo or documented demo command, then classify the outcome as:

- `proving`
- `non_proving`
- `skipped`
- `failed`

## Supported Modes

- `operate_named_demo`
- `operate_demo_command`
- `operate_demo_doc`

## Top-Level Shape

```yaml
skill_input_schema: demo_operator.v1
mode: operate_named_demo | operate_demo_command | operate_demo_doc
repo_root: /absolute/path
target:
  demo_name: <string or null>
  demo_command: <string or null>
  demo_doc_path: <path or null>
  artifact_root: <path or null>
  expected_artifacts:
    - <path>
  provider_requirements:
    - <string>
  operator_gate_reason: <string or null>
policy:
  classification_mode: strict | standard
  allow_live_provider: true | false
  validation_mode: dry_run | bounded_live | none
  stop_after_operation: true
```

## Mode Requirements

### `operate_named_demo`

Requires:

- `target.demo_name`

Use when:

- the repo already has a known named demo surface and the operator wants consistent classification

### `operate_demo_command`

Requires:

- `target.demo_command`

Use when:

- the demo entrypoint is best expressed as one concrete command

### `operate_demo_doc`

Requires:

- `target.demo_doc_path`

Use when:

- the operator wants the skill to begin from the canonical demo doc and its documented command

## Policy Fields

- `classification_mode`
  - required
  - use `strict` when proof claims should be conservative
- `allow_live_provider`
  - required
  - whether live-provider execution is allowed
- `validation_mode`
  - required
  - one of `dry_run`, `bounded_live`, or `none`
- `stop_after_operation`
  - must be `true`

## Example Invocation

```yaml
Use $demo-operator at /Users/daniel/git/agent-design-language/adl/tools/skills/demo-operator/SKILL.md with this validated input:

skill_input_schema: demo_operator.v1
mode: operate_named_demo
repo_root: /Users/daniel/git/agent-design-language
target:
  demo_name: gemma4_issue_clerk
  demo_command: bash adl/tools/demo_v089_gemma4_issue_clerk.sh --dry-run
  demo_doc_path: demos/v0.89/gemma4_issue_clerk_demo.md
  artifact_root: null
  expected_artifacts:
    - artifacts/v089/gemma4_issue_clerk/demo_manifest.json
  provider_requirements: []
  operator_gate_reason: null
policy:
  classification_mode: strict
  allow_live_provider: false
  validation_mode: dry_run
  stop_after_operation: true
```

## Notes

- prefer dry-run or fixture-backed demo paths first
- do not classify a demo as `proving` unless the intended artifact surface exists
- do not widen into demo implementation or release-evidence work
