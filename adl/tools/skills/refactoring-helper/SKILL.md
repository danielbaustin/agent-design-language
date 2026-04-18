---
name: refactoring-helper
description: Plan and bound source-grounded refactoring work by turning a concrete code surface into small behavior-preserving slices with invariants, risk inventory, validation commands, rollback notes, and residual risks, while stopping before broad rewrites or silent behavior changes.
---

# Refactoring Helper

Use this skill when a messy code surface needs a safer refactoring path before
implementation. The skill creates a behavior-preserving refactor plan, bounded
refactor plans, and execution slices; it does not turn vague cleanup instincts
into broad rewrites.

## Quick Start

1. Confirm the bounded code surface:
   - exact files, modules, diff, or issue scope
   - current behavior to preserve
   - explicit behavior changes, if any
2. Identify invariants before edits:
   - public API behavior
   - persistence or artifact format
   - ordering or determinism guarantees
   - security/privacy boundaries
   - compatibility contracts
3. Run the deterministic helper when local filesystem access is available:
   - `scripts/plan_refactor.py <refactor-root> --out <artifact-root>`
4. Review the refactor plan, risk inventory, validation plan, slices, and
   rollback notes.
5. Stop unless the operator explicitly asks to implement one bounded slice.

## Required Inputs

At minimum, gather:

- `mode`
- `target`
- `current_behavior`
- `refactor_intent`
- `policy`

Supported modes:

- `plan_refactor`
- `slice_refactor`
- `review_refactor_plan`
- `prepare_refactor_handoff`

Useful policy fields:

- `behavior_change_allowed`
- `max_slice_count`
- `require_tests_before_edits`
- `require_rollback_notes`
- `stop_before_broad_rewrite`
- `write_refactor_artifact`

If no bounded target is supplied, stop and report `not_run`. If behavior change
is requested, name it explicitly and keep it separate from behavior-preserving
refactor work.

## Refactor Slice Rules

Each slice should include:

- the smallest coherent code surface
- behavior-preservation intent or explicit behavior-change note
- invariants that must remain true
- likely changed files
- validation commands and what they prove
- rollback notes
- residual risk and follow-on slices

Prefer sequencing that reduces blast radius:

1. characterization or contract tests
2. extraction or naming cleanup
3. dependency boundary tightening
4. internal data-shape cleanup
5. deletion of dead paths only after proof exists

## Output

Write Markdown and JSON artifacts when an output root is available.

Default artifact root:

```text
.adl/reviews/refactoring-helper/<run_id>/
```

Required artifacts:

- `refactor_plan.md`
- `refactor_plan.json`

Use the detailed contract in `references/output-contract.md`.

## Stop Boundary

This skill must not:

- perform large unbounded rewrites
- make silent behavior changes
- skip invariant identification before proposing edits
- replace code review, test generation, CI, or human approval
- claim implementation completion when it only produced a plan
- create issues, PRs, commits, or release notes without explicit operator
  approval
- delete code without proof that the path is unused or replaced

Handoff candidates:

- `test-generator` when characterization or regression tests are needed.
- `repo-code-review` when the code surface needs defect review before planning.
- `gap-analysis` when refactor output must be compared to an expected contract.
- `finding-to-issue-planner` when human-approved follow-up issues are needed.

## Blocked States

Return `not_run` when the bounded target is missing.

Return `blocked` when the request requires broad rewrite, silent behavior
change, repo mutation, or issue/PR creation without explicit approval.

Return `partial` when the plan can be produced but the evidence is incomplete or
important invariants are unknown.
