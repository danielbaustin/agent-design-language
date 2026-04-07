# Output Contract

When ADL expects structured `pr-ready` output, use this shape.

```yaml
status: ready | ready_with_repairs | blocked
execution_readiness: ready | ready_with_repairs | blocked
preflight_status: not_checked | pass | blocked_now
target:
  issue_number: <u32 or null>
  task_bundle_path: <path or null>
  branch: <branch or null>
  worktree_path: <path or null>
findings:
  - severity: info | warning | blocking
    area: identity | surfaces | branch | worktree | preflight | traceability | placeholder_drift
    message: <short finding>
actions_taken:
  - <bounded repair or inspection action>
actions_recommended:
  - <next action>
files_touched:
  - <path>
validation_performed:
  - ready_check
  - preflight_check
  - direct_file_inspection
  - branch_inspection
  - worktree_inspection
handoff_state:
  next_phase: issue_bootstrap | qualitative_card_review | pr_run | human_review
  ready_for_bootstrap: true | false
  ready_for_card_review: true | false
  ready_for_run: true | false
follow_up_required:
  - <optional follow-up>
```

## Rules

- If `status: ready`, there must be no remaining blocking findings.
- If `status: ready_with_repairs`, `actions_taken` must record the repairs and no blocking findings may remain after them.
- If `status: blocked`, `actions_recommended` should identify the correct handoff or missing prerequisite.
- `status` should mirror `execution_readiness`, not temporary wave scheduling state.
- `preflight_status: blocked_now` may coexist with `status: ready` when the issue is structurally ready but cannot start immediately under the current wave/open-PR policy.
- Do not claim readiness for `pr_run` if execution-critical surfaces still contain blocking placeholder drift or branch/worktree mismatch.
- List `files_touched` only for actual bounded repair writes.

## Default Artifact Location

When writing the `pr-ready` result to disk by default, use:

```text
.adl/reviews/<timestamp>-pr-ready.md
```
