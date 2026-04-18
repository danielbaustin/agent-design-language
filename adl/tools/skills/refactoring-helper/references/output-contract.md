# Output Contract

The refactoring-helper skill produces bounded refactor plans for explicit code
surfaces. It can prepare a safe implementation handoff, but it does not by
itself claim the refactor was implemented.

Default artifact root:

```text
.adl/reviews/refactoring-helper/<run_id>/
```

## Required Artifacts

### refactor_plan.md

Required sections:

- Refactor Plan Summary
- Scope
- Current Behavior
- Refactor Intent
- Invariants
- Risk Inventory
- Refactor Slices
- Validation Plan
- Rollback Notes
- Residual Risk
- Stop Boundary

### refactor_plan.json

Required top-level fields:

- `schema`
- `run_id`
- `status`
- `scope`
- `target`
- `current_behavior`
- `refactor_intent`
- `invariants`
- `risks`
- `slices`
- `validation_plan`
- `rollback_notes`
- `residual_risk`
- `stop_boundary`

## Status Values

- `ready`: bounded slices and validation proof are available.
- `partial`: a useful plan exists, but evidence, invariants, or validation proof
  are incomplete.
- `not_run`: no bounded target was supplied.
- `blocked`: the request would require broad rewrite, silent behavior change,
  unapproved mutation, or unsafe deletion.

## Slice Shape

Each slice must include:

- stable id
- title
- intent
- behavior change flag
- target files or surfaces
- invariants
- validation commands
- rollback notes
- residual risk
- follow-up slice or explicit none

## Rules

- Require a bounded target before planning.
- Identify behavior-preserving intent unless a behavior change is explicit.
- Identify invariants before edits.
- Prefer small slices over broad rewrites.
- Treat missing tests as a risk or validation prerequisite.
- Use repo-relative, issue-relative, or packet-relative paths.
- Do not write absolute host paths into plan artifacts.
- Do not claim implementation, merge-readiness, release-readiness, or approval.
- Do not create issues, PRs, commits, fixes, test files, or code edits without
  explicit operator approval.
