# Refactoring Helper Skill Input Schema

Schema id: `refactoring_helper.v1`

## Purpose

Turn a bounded code surface into a behavior-preserving refactor plan with
invariants, validation commands, rollback notes, residual risks, and small
execution slices.

## Required Top-Level Fields

- `skill_input_schema`: must be `refactoring_helper.v1`.
- `mode`: one of `plan_refactor`, `slice_refactor`,
  `review_refactor_plan`, or `prepare_refactor_handoff`.
- `target`: bounded files, modules, diff paths, plan path, or handoff target.
- `current_behavior`: source-grounded description of behavior to preserve.
- `refactor_intent`: structural improvement intent and any explicit behavior
  changes.
- `policy`: refactor safety and stop-boundary policy.

## Optional Fields

- `artifact_root`: refactor plan destination.
- `known_risks`
- `invariants`
- `validation_commands`
- `rollback_constraints`

## Policy Fields

- `behavior_change_allowed`
- `max_slice_count`
- `require_tests_before_edits`
- `require_rollback_notes`
- `stop_before_broad_rewrite`
- `write_refactor_artifact`

## Example

```yaml
skill_input_schema: refactoring_helper.v1
mode: plan_refactor
target:
  paths:
    - adl/src/cli/pr_cmd.rs
current_behavior:
  preserve:
    - issue lifecycle commands keep root main clean
    - started issue work happens in issue worktrees
refactor_intent:
  summary: split branch/worktree guard helpers from publication code without behavior changes
  behavior_change_allowed: false
policy:
  behavior_change_allowed: false
  max_slice_count: 4
  require_tests_before_edits: true
  require_rollback_notes: true
  stop_before_broad_rewrite: true
  write_refactor_artifact: true
```

## Output Contract

Default artifact root:

```text
.adl/reviews/refactoring-helper/<run_id>/
```

Required artifacts:

- `refactor_plan.md`
- `refactor_plan.json`

Statuses:

- `ready`: bounded slices and validation proof are available.
- `partial`: evidence, invariants, or validation proof are incomplete.
- `not_run`: bounded target missing.
- `blocked`: request violates broad-rewrite, mutation, or behavior-change
  boundary.

## Stop Boundary

The skill must not perform large unbounded rewrites, make silent behavior
changes, delete code without proof, create issues or PRs, claim implementation
completion, or mutate repositories without explicit operator approval.
