# Output Contract

When ADL expects structured `pr-init` output, use this shape.

```yaml
status: complete | partial | blocked | failed
mode: create_and_bootstrap | bootstrap_existing_issue
issue:
  number: <issue number or null>
  url: <issue url or null>
  title: <title>
  slug: <slug>
  version: <version or scope>
paths:
  source_prompt: <path or null>
  bundle_dir: <path or null>
  stp: <path or null>
  sip: <path or null>
  sor: <path or null>
validation:
  issue_created_or_resolved: true | false
  source_prompt_present: true | false
  bundle_dir_present: true | false
  stp_present: true | false
  sip_present: true | false
  sor_present: true | false
  compatibility_links_present: true | false | not_applicable
  branch_created: true | false
  worktree_created: true | false
next_step:
  recommended_phase: qualitative_card_review
  recommended_command: <command, skill, or manual phase>
  handoff_reason: <short explanation>
handoff:
  ready_for_card_review: true | false
  ready_for_execution: true | false
  notes:
    - <optional handoff note>
notes:
  - <optional note>
```

## Rules

- If `status: complete`, all required bootstrap surfaces must be present and both `branch_created` and `worktree_created` must be `false`.
- If `status: complete`, `ready_for_card_review` should normally be `true` and `ready_for_execution` should normally be `false`.
- If the issue exists but one or more bundle surfaces are missing, use `partial` or `blocked`, not `complete`.
- If bootstrap output is obviously contradictory or not ready for the next step, use `blocked`.
- Report exact paths when they are known.
- Do not claim readiness for issue-mode `pr run` or execution unless a later qualitative review step has actually been completed.

## Default Artifact Location

When writing the `pr-init` result to disk by default, use:

```text
.adl/reviews/<timestamp>-pr-init.md
```
