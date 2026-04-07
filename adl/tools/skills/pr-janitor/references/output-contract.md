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
  next_phase: pr_janitor | pr_finish | human_review | blocked
  ready_for_finish: true | false
follow_up_required:
  - <optional follow-up>
```

## Rules

- If `status: healthy`, there must be no known failing checks or blocking conflicts.
- If `status: action_required`, the blocker must be concrete and the recommended or applied response must stay in-bounds.
- If `status: blocked`, explain whether the blocker is ambiguity, reviewer judgment, unsafe remediation scope, or unresolved conflict/check state.
- Do not mark `human_review_required: false` when substantive review feedback still needs judgment.
- `repair_outcome` must say explicitly whether the run only inspected, applied a bounded repair, or declined repair because it was unsafe or unsuccessful.
- `handoff_state.next_phase` must say whether the PR should remain in janitor monitoring, move to `pr-finish`, escalate to human review, or stay blocked.

## Default Artifact Location

When writing the janitor result to disk by default, use:

```text
.adl/reviews/<timestamp>-pr-janitor.md
```
