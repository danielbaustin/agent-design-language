# Output Contract

When ADL expects structured run output, use this shape.

```yaml
status: in_progress | done | blocked | failed
target:
  issue_number: <u32 or null>
  task_bundle_path: <path or null>
  branch: <branch or null>
  worktree_path: <path or null>
binding_state:
  doctor_execution_readiness: ready | ready_with_repairs | blocked | unknown
  preflight_status: not_checked | pass | blocked_now | overridden
  branch_state: created | reused | already_bound | blocked
  worktree_state: created | reused | already_bound | blocked
materialization_state:
  worktree_bundle_present: true | false
  worktree_stp_present: true | false
  worktree_sip_present: true | false
  worktree_sor_present: true | false
findings:
  - severity: info | warning | blocking
    area: identity | surfaces | branch | worktree | execution | validation | output_record | preflight
    message: <short finding>
actions_taken:
  - <execution action>
actions_recommended:
  - <next action>
files_touched:
  - <path>
validation_performed:
  - <command or check>
handoff_state:
  next_phase: pr_finish | pr_janitor | human_review | blocked
  ready_for_finish: true | false
  ready_for_janitor: true | false
follow_up_required:
  - <optional follow-up>
```

## Rules

- `status` must describe the actual execution outcome, not the hoped-for one.
- If execution was blocked before writing implementation changes, say so explicitly.
- If preflight was overridden, record that in `binding_state.preflight_status`.
- If worktree-local `stp.md`, `sip.md`, or `sor.md` are missing after binding, `materialization_state.worktree_bundle_present` must be `false` and the run must not be reported as fully ready for finish.
- `files_touched` should list actual edited files only.
- `validation_performed` should include only real checks that ran.
- Do not claim `done` if the issue remains only partially implemented or unvalidated.

## Default Artifact Location

When writing the run result to disk by default, use:

```text
.adl/reviews/<timestamp>-pr-run.md
```
