# Output Contract

When ADL expects structured `issue-watcher` output, use this shape.

```yaml
status: ready | pending | blocked | action_required | merged | closed
target:
  issue_number: <u32 or null>
  pr_number: <u32 or null>
  pr_url: <url or null>
  branch: <branch or null>
  dependency_issue_number: <u32 or null>
observed_state:
  issue_state: open | closed | unknown | not_applicable
  pr_state: draft | open | merged | closed | unknown | not_applicable
  checks_state: pass | fail | pending | mixed | unknown | not_applicable
  merge_state: clean | dirty | blocked | unknown | not_applicable
dependency_state:
  satisfied: true | false | unknown
  blockers:
    - <short blocker description>
handoff:
  next_skill: pr-init | pr-ready | pr-run | pr-janitor | pr-closeout | none
  next_operator_action: <short action or null>
  ready_for_next_work: true | false | unknown
lifecycle_shepherd:
  active: true | false
  state: pre_run | execution_bound | publication_ready | pr_waiting | janitor_active | merged_needs_closeout | closed_no_pr | settled | blocked
  owner_skill: workflow-conductor | pr-ready | pr-run | pr-finish | issue-watcher | pr-janitor | pr-closeout | human_review | none
  next_skill: pr-init | pr-ready | pr-run | pr-finish | issue-watcher | pr-janitor | pr-closeout | stp-editor | sip-editor | spp-editor | srp-editor | sor-editor | human_review | none
  closeout_required: true | false
  authority_boundary:
    merge_authority_human_only: true | false
    issue_close_authority_human_only: true | false
    review_authority_human_only: true | false
actions_taken:
  - <inspection action>
actions_recommended:
  - <next action>
notes:
  - <optional bounded note>
```

## Rules

- If `status: ready`, `handoff.ready_for_next_work` must be `true`.
- If `status: pending`, the target must have no known hard blocker; it is still waiting on checks, review, draft state, merge, or closure.
- If `status: action_required`, `handoff.next_skill` or `handoff.next_operator_action` must name the required next step.
- If PR checks, conflicts, or requested changes are the blocker, `handoff.next_skill` must be `pr-janitor`.
- If cards or workflow readiness are the blocker, route to `pr-init` or `pr-ready` rather than repairing inside this skill.
- Do not claim `ready_for_next_work: true` when the watched prerequisite is still a draft PR, has failing checks, is ambiguous, or requires human approval.
- Healthy PR waiting states should normally record `lifecycle_shepherd.state: pr_waiting` with `active: true`.

## Default Artifact Location

When writing the watcher result to disk by default, use:

```text
.adl/reviews/<timestamp>-issue-watcher.md
```

See `docs/tooling/ISSUE_LIFECYCLE_SHEPHERD_CONTRACT.md` for the canonical
shared state meanings.
