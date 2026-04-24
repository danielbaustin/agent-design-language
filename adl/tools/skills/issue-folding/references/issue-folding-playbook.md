# Issue Folding Playbook

Use `issue-folding` when an issue appears to have become a no-op or should close
without a fresh implementation PR.

## Evidence Order

Prefer these surfaces:

1. source issue prompt
2. `stp.md`
3. `sip.md`
4. `sor.md`
5. explicit linked issue or PR references

Do not infer `duplicate`, `superseded`, or `absorbed` without a linked issue or
PR reference unless the packet clearly marks the issue as obsolete or already
satisfied.

## Recommended Mappings

- `duplicate`:
  - use when the packet says the same work is already tracked elsewhere
  - preserve linked issue references
- `superseded`:
  - use when the issue was replaced by a newer, clearer issue or PR
- `absorbed`:
  - use when the work is explicitly folded into another issue or PR
- `already_satisfied`:
  - use when the intended state is already true on main and no new PR is needed
- `obsolete`:
  - use when the premise no longer matters because policy, milestone, or design changed

## Blockers

Classify as `blocked` when:

- more than one non-actionable class appears with conflicting meaning
- linked references are missing for duplicate, superseded, or absorbed claims
- the issue packet contains both “still do this” and “already satisfied” style markers

## Handoff

- `actionable` -> normal workflow execution
- non-actionable class with sufficient evidence -> `pr-closeout`
- `blocked` -> operator review
