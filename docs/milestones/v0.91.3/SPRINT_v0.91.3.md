# v0.91.3 Sprint Plan

## Status

Active sprint map. Sprint umbrella issues are open as `#3212` through `#3214`
plus closeout-tail Sprint 4 / `#3231`. Sprint 1 starts at `WP-02` / `#3200`
because `WP-01` / `#3199` is already merged and recorded closed out in Sprint
1 state.

## Sprint Overview

v0.91.3 should execute as four bounded sprints:

| Sprint | Issue | Title | Ordered Children | Goal |
| --- | --- | --- | --- | --- |
| Sprint 1 | #3212 | Transition Substrate | WP-01 / #3199, WP-02 / #3200, WP-03 / #3201, WP-04 / #3202 | Establish the schema, actor-role seed, lifecycle, and DAG needed for one transition. |
| Sprint 2 | #3213 | Evidence, Review, And Memory | WP-05 / #3203, WP-06 / #3204, WP-07 / #3205 | Make evidence, review, merge readiness, and ObsMem handoff explicit. |
| Sprint 3 | #3214 | First Proof | WP-08 / #3206, WP-09 / #3207 | Apply process-readiness lessons and run the first proof demo. |
| Sprint 4 | #3231 | Review, Remediation, Planning, And Release | WP-10 / #3226, WP-11 / #3227, WP-12 / #3228, WP-13 / #3208, WP-14 / #3229, WP-15 / #3209, WP-16 / #3210, WP-17 / #3230, WP-18 / #3211 | Run proof coverage, quality gate, docs pass, internal/external review, remediation, next-milestone planning, final next-milestone review pass, and release ceremony. |

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

## Sprint 3: First Proof

Sprint 3 runs the first bounded proof and produces the evidence needed for the
closeout tail.

Exit criteria:

- combined-lane validation and closeout-truth lessons are applied before the
  first proof
- the five-minute-sprint first proof runs successfully; a skipped proof is a
  truthful execution state but not successful milestone completion unless the
  milestone is explicitly deferred or no-go
- proof evidence is ready for Sprint 4 review and quality gates

## Sprint 4: Review, Remediation, Planning, And Release

Sprint 4 is the complete closeout tail.

Exit criteria:

- demo/proof coverage is complete or gaps are explicitly routed
- quality gate reflects the actual touched surfaces
- docs are ready for internal review
- internal and external/third-party review results are recorded
- review findings are fixed or explicitly routed
- v0.91.4 receives a concrete hardening backlog
- the final next-milestone review pass is complete
- release ceremony happens last
