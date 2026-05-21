# v0.91.3 Sprint Plan

## Status

Planned sprint map. Sprint issue numbers are `pending` until the issue wave is
opened.

## Sprint Overview

v0.91.3 should execute as three bounded sprints:

| Sprint | Title | Ordered Children | Goal |
| --- | --- | --- | --- |
| Sprint 1 | Transition Substrate | WP-01, WP-02, WP-03, WP-04 | Establish the schema, actor-role seed, lifecycle, and DAG needed for one transition. |
| Sprint 2 | Evidence, Review, And Memory | WP-05, WP-06, WP-07 | Make evidence, review, merge readiness, and ObsMem handoff explicit. |
| Sprint 3 | First Proof And Handoff | WP-08, WP-09, WP-10, WP-11, WP-12, WP-13 | Apply process-readiness lessons, run the proof demo, review it, remediate findings, and hand v0.91.4 the hardening plan. |

## Sprint Goals

The sprint overview table above is the generator-facing sprint map. The goals
below explain the intended execution posture for each sprint without replacing
that canonical table.

## Execution Policy

Every sprint must use the current ADL workflow discipline:

- `workflow-conductor` routes every issue and lifecycle stage
- cards are edited only with editor skills
- tracked work happens only in a bound worktree on a specific branch
- pre-PR review happens before publication
- closeout happens after issue closure

## Sprint 1: Transition Substrate

Sprint 1 builds the smallest substrate that can describe one Cognitive State
Transition.

Exit criteria:

- transition manifest shape exists
- actor and role references are present for material transition participants
- card lifecycle semantics are preserved
- transition DAG fixture exists
- shard and barrier vocabulary is explicit

## Sprint 2: Evidence, Review, And Memory

Sprint 2 makes the transition reviewable and governable.

Exit criteria:

- evidence bundle shape exists
- SRP review-result expectations are explicit
- SOR outcome truth feeds the memory handoff boundary
- merge readiness records GitHub issue/PR/CI/human review truth

## Sprint 3: First Proof And Handoff

Sprint 3 runs the first bounded proof and converts findings into v0.91.4 work.

Exit criteria:

- combined-lane validation and closeout-truth lessons are applied before the
  first proof
- the five-minute-sprint first proof runs successfully; a skipped proof is a
  truthful execution state but not successful milestone completion unless the
  milestone is explicitly deferred or no-go
- combined-lane validation is run for the touched C-SDLC surfaces
- internal review findings are fixed or routed
- v0.91.4 receives a concrete hardening backlog
