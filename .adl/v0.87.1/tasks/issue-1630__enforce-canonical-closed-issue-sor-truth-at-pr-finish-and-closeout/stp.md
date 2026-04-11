---
issue_card_schema: adl.issue.v1
wp: "unassigned"
slug: "enforce-canonical-closed-issue-sor-truth-at-pr-finish-and-closeout"
title: "[v0.87.1][tools] Enforce canonical closed-issue SOR truth at pr finish and closeout"
labels:
  - "track:roadmap"
  - "type:task"
  - "area:tools"
  - "version:v0.87.1"
issue_number: 1630
status: "draft"
action: "edit"
depends_on: []
milestone_sprint: "Pending sprint assignment"
required_outcome_type:
  - "code"
repo_inputs: []
canonical_files: []
demo_required: false
demo_names: []
issue_graph_notes:
  - "Hardens the compressed issue-record model after #1555 exposed stale closed-issue SOR drift."
pr_start:
  enabled: false
  slug: "enforce-canonical-closed-issue-sor-truth-at-pr-finish-and-closeout"
---

## Summary

Add a hard guard so closed issues cannot publish or close out while their canonical root `sor.md` still reports stale lifecycle truth.

## Goal

Finish the missing enforcement step in the milestone-compression program by making stale closed-issue output records impossible to ignore.

## Required Outcome

- fail `pr finish` and/or closeout when a closed issue's canonical `sor.md` is stale
- validate the root canonical `.adl/<scope>/tasks/.../sor.md`, not only the active worktree copy
- surface exact mismatch details so the failure is actionable
- add regression coverage for failure and success paths

## Deliverables

- control-plane enforcement in the finish or closeout path
- actionable drift diagnostics for stale closed-issue SOR state
- regression coverage proving stale closed records are rejected and truthful closed records pass

## Acceptance Criteria

- `pr finish` and/or closeout fails when a closed issue's canonical `sor.md` still reports stale lifecycle state
- the check validates the root canonical `.adl/<scope>/tasks/.../sor.md`, not just the active worktree copy
- the failure message identifies the exact issue bundle path and the mismatched fields that require normalization
- tooling distinguishes open-issue execution state from closed-issue closeout truth and only enforces the gate for issues that should be merged or closed
- duplicate or superseded issue-bundle residue is surfaced as actionable drift instead of being silently ignored
- a passing path exists where truthful merged/closed SOR state allows `pr finish` / closeout to continue without manual cleanup
- regression coverage includes at least one stale-SOR failure case and one normalized-success case

## Repo Inputs

- `https://github.com/danielbaustin/agent-design-language/issues/1630`
- `.adl/docs/TBD/MILESTONE_COMPRESSION_PLAN.md`
- `.adl/v0.87.1/tasks/issue-1555__v0-87-1-records-normalize-remaining-closed-issue-task-bundles-to-truthful-merged-closeout-state/`
- `adl/tools/skills/pr-finish/SKILL.md`
- `adl/tools/skills/pr-closeout/SKILL.md`

## Dependencies

- `#1555` as the motivating cleanup wave
- the compressed issue-record model introduced by the v0.87/v0.87.1 tooling work

## Demo Expectations

- No standalone demo required. Proof is deterministic failing and passing tooling behavior plus regression coverage.

## Non-goals

- broad milestone closeout redesign unrelated to stale closed-issue records
- manual one-off cleanup without a durable guardrail
- changing the canonical issue-record model again instead of enforcing the current one

## Issue-Graph Notes

- This issue turns the compressed-model closeout rules into an enforced guardrail.
- It should reduce the chance of future `#1555`-style hygiene waves.

## Notes

- This is the enforcement counterpart to the earlier model simplification work.

## Tooling Notes

- Keep GitHub issue metadata, local source prompt, and task cards aligned.
- Prefer bounded failure messages that tell the operator exactly what to normalize.
