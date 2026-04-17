# Repo Architecture Review Skill Input Schema

Schema id: `repo_architecture_review.v1`

Use this schema for structured invocations of the
`repo-architecture-review` skill.

## Required Top-Level Fields

- `skill_input_schema`: must equal `repo_architecture_review.v1`
- `mode`: one of the supported review modes
- `repo_root`: absolute repository root
- `target`: concrete review target
- `policy`: explicit review policy

## Supported Modes

- `review_repository`
- `review_path`
- `review_branch`
- `review_diff`
- `review_packet`

## Target Fields

At least one target field should be concrete:

- `target_path`
- `branch`
- `diff_base`
- `review_packet_path`
- `changed_paths`
- `artifact_root`

`review_packet` mode requires `review_packet_path`.

## Required Policy Fields

- `review_depth`: `quick`, `standard`, or `deep`
- `validation_mode`: `targeted`, `inspect_only`, or `none`
- `write_review_artifact`: boolean
- `stop_after_review`: must be true
- `architecture_focus`: optional list or string naming focus areas

## Example

```yaml
skill_input_schema: repo_architecture_review.v1
mode: review_packet
repo_root: /absolute/path/to/repo
target:
  target_path: null
  branch: null
  diff_base: null
  review_packet_path: .adl/reviews/codebuddy/20260417-120000-repo-packet
  changed_paths: []
  artifact_root: .adl/reviews/codebuddy/20260417-120000-repo-packet/architecture-review
policy:
  review_depth: deep
  validation_mode: inspect_only
  write_review_artifact: true
  stop_after_review: true
  architecture_focus:
    - boundaries
    - layering
    - runtime_state
    - drift
```

## Output

The skill writes or returns a findings-first architecture review artifact with:

- findings
- architecture map
- reviewed surfaces
- candidate diagram tasks
- candidate ADRs
- candidate fitness functions
- validation performed
- residual risk

The deterministic scaffold helper may write:

- `architecture_review_scaffold.md`
- `architecture_review_scaffold.json`

See `repo-architecture-review/references/output-contract.md`.

