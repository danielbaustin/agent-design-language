# Output Contract

When ADL expects structured janitor output, use this shape.

```yaml
status: healthy | action_required | blocked
target:
  pr_number: <u32 or null>
  pr_url: <url or null>
  branch: <branch or null>
  issue_number: <u32 or null>
checks_summary:
  passing: <int>
  failing: <int>
  pending: <int>
  details:
    - name: <check name>
      state: pass | fail | pending | unknown
conflict_status:
  mergeable: true | false | unknown
  details: <short explanation>
repair_outcome:
  mode: inspect_only | bounded_blocker_fixes
  result: no_repair_attempted | repair_applied | repair_attempt_failed | repair_not_safe
  details: <short explanation>
actions_taken:
  - <inspection, rerun, or bounded fix>
actions_recommended:
  - <next action>
review_required:
  human_review_required: true | false
  reason: <short explanation>
handoff_state:
  next_phase: pr_janitor | pr_finish | pr_closeout | human_review | blocked
  ready_for_finish: true | false
  shepherding_active: true | false
  settlement_state: waiting_for_review | waiting_for_checks | green_and_mergeable | merged_needs_closeout | blocked
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
follow_up_required:
  - <optional follow-up>
```

## Rules

- If `status: healthy`, there must be no known failing checks or blocking conflicts.
- If `status: action_required`, the blocker must be concrete and the recommended or applied response must stay in-bounds.
- If `status: blocked`, explain whether the blocker is ambiguity, reviewer judgment, unsafe remediation scope, or unresolved conflict/check state.
- Do not mark `human_review_required: false` when substantive review feedback still needs judgment.
- `repair_outcome` must say explicitly whether the run only inspected, applied a bounded repair, or declined repair because it was unsafe or unsuccessful.
- `handoff_state.next_phase` must say whether the PR should remain in janitor monitoring, move to `pr-finish`, hand off to `pr_closeout`, escalate to human review, or stay blocked.
- `handoff_state.shepherding_active` must stay `true` for healthy waiting states until merge or explicit closure settles.
- `handoff_state.settlement_state` must distinguish ordinary waiting from `merged_needs_closeout`.
- `lifecycle_shepherd.state` should normally be `janitor_active` while bounded blocker remediation is in progress.

## Default Artifact Location

When writing the janitor result to disk by default, use:

```text
.adl/reviews/<timestamp>-pr-janitor.md
```

See `docs/tooling/ISSUE_LIFECYCLE_SHEPHERD_CONTRACT.md` for the canonical
shared state meanings.
