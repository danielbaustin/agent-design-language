# Repo Packet Builder Skill Input Schema

Schema id: `repo_packet_builder.v1`

Use this schema when invoking the `repo-packet-builder` skill with structured
input.

## Required Top-Level Fields

- `skill_input_schema`: must equal `repo_packet_builder.v1`.
- `mode`: one supported mode.
- `repo_root`: absolute path to the repository root.
- `target`: object describing the scope.
- `policy`: object describing packet-building policy.

## Modes

- `build_repository_packet`
- `build_path_packet`
- `build_branch_packet`
- `build_diff_packet`
- `refresh_packet`

## Target Fields

- `target_path`: required for `build_path_packet`.
- `branch`: required for `build_branch_packet`.
- `diff_base`: required for `build_diff_packet`.
- `changed_paths`: optional path list.
- `existing_packet_path`: required for `refresh_packet`.
- `artifact_root`: optional output root.

## Policy Fields

- `scope_policy`: required; for example `whole_repo`, `path_only`,
  `diff_plus_context`, or `refresh_existing`.
- `privacy_mode`: required; for example `local_only`, `customer_private`, or
  `public_safe_shape`.
- `include_generated_code`: required boolean when structured input is used.
- `include_vendor_code`: required boolean when structured input is used.
- `context_budget`: required; may be a qualitative label or numeric budget.
- `specialist_lanes`: required list when structured input is used.
- `stop_before_review`: must be `true`.

## Example

```yaml
skill_input_schema: repo_packet_builder.v1
mode: build_repository_packet
repo_root: /abs/path/to/repo
target:
  artifact_root: .adl/reviews/codebuddy/example-run
policy:
  scope_policy: whole_repo
  privacy_mode: local_only
  include_generated_code: false
  include_vendor_code: false
  context_budget: bounded
  specialist_lanes:
    - code
    - security
    - tests
    - docs
    - architecture
    - dependencies
  stop_before_review: true
```

## Stop Conditions

Stop and report `blocked` when:

- `repo_root` is missing.
- the requested path/diff/branch target is missing.
- policy is not explicit.
- `stop_before_review` is false.
- the artifact root cannot be written.

