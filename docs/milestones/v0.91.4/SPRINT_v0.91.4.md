# v0.91.4 Sprint Plan

## Metadata

- Milestone: `v0.91.4`
- Version: `v0.91.4`
- Date: `2026-05-25`
- Owner: ADL maintainers
- Related issues: `#3210`, seeded v0.91.4 sprint umbrellas with Sprint 1 closed

## Status

Sprint 1 is closed with clean closeout truth. Sprint 2, Sprint 3, Sprint 4,
the CodeFriend sidecar, and the WildClawBench benchmark spike sidecar remain
seeded as controlled issue/card batches.

## How To Use

Use this document to track sprint umbrellas, ordered child work, and closeout
state. Sprint membership is canonical here, while live execution/closeout
truth must stay aligned with the sprint state artifacts.

## Sprint Overview

| Sprint | Title | Ordered Children | Goal |
| --- | --- | --- | --- |
| Sprint 1 | Lifecycle And Routing Hardening (`#3347`) | WP-01 `#3346`, WP-02 `#3348`, WP-03 `#3349`, WP-04 `#3350` | Make validators, doctor, conductor, and editor skills agree on C-SDLC state. |
| Sprint 2 | Transition Operation (`#3352`) | WP-05 `#3353`, WP-06 `#3354`, WP-07 `#3355`, WP-08 `#3356` | Make actor standing, shards, evidence, merge gates, and memory handoff repeatable. |
| Sprint 3 | Sprint Default And Metrics (`#3357`) | WP-09 `#3358`, WP-10 `#3359`, WP-11 `#3360`, WP-12 `#3361` | Make sprint execution default-safe and measure repeatability, validation-tail, proof-latency, and parallel-validation behavior. |
| Sprint 4 | Review, Remediation, Planning, And Release (`#3362`) | WP-13 `#3363`, WP-14 `#3364`, WP-15 `#3365`, WP-16 `#3366`, WP-17 `#3367`, WP-18 `#3368`, WP-19 `#3369`, WP-20 `#3370`, WP-21 `#3371` | Prove, gate, review, remediate, plan the next milestone, re-review the handoff, and close the completion milestone. |

## Sidecar Mini-Sprint

The CodeFriend pre-alpha repo/S3 welcome-page setup and the WildClawBench
benchmark spike run as bounded sidecar mini-sprints in v0.91.4:

| Sidecar | Title | Ordered Children | Goal |
| --- | --- | --- | --- |
| CodeFriend Pre-Alpha Setup (`#3372`) | CodeFriend pre-alpha site setup | CF-PRE-01 `#3373`, CF-PRE-02 `#3374`, CF-PRE-03 `#3375`, CF-PRE-04 `#3376` | Establish the private CodeFriend repo and a verified S3/CloudFront/HTTPS welcome page without making CodeFriend part of C-SDLC core proof. |
| WildClawBench Benchmark Spike (`#3378`) | WildClawBench benchmark spike | WC-PRE-01 `#3379`, WC-PRE-02 `#3380`, WC-PRE-03 `#3381`, WC-PRE-04 `#3382` | Run a small external benchmark spike that tests ADL substrate evidence without making WildClawBench a release gate or benchmark-win claim. |

Current sidecar state:

- CodeFriend is now complete as a bounded pre-alpha product-setup lane:
  - `#3373` created and scaffolded the private `agent-logic/codefriend.ai`
    repository
  - `#3374` refined the static coming-soon page
  - `#3375` provisioned the Terraform-managed AWS static-site substrate and
    made `https://codefriend.ai` and `https://www.codefriend.ai` live over
    HTTPS
  - `#3376` recorded publication-safety review, verification, and handoff
  - the sidecar outcome is a real live coming-soon surface, not a CodeFriend
    alpha-product claim
- WildClawBench is in final publication state as a bounded docs-and-evidence
  spike:
  - `#3379` published setup and smoke-baseline notes
  - `#3380` published adapter, validity-audit, and bounded safety-slice results
  - `#3381` published the `UTS`/`ACC` comparison framing and re-entry matrix
  - `#3382` has the final results taxonomy and handoff recommendation in draft
    PR for review
  - the sidecar recommendation is to defer broader benchmark work until a
    later post-launch evaluation lane with a real ADL benchmark subject

The sidecar may run after WP-01 has opened the v0.91.4 issue wave. It must not
interrupt the required C-SDLC closeout tail or add extra release-tail gates.

## Issue Opening Policy

The v0.91.4 issue wave is opened in sprint-sized batches so the new upfront-card
rule stays reviewable:

- Batch 0 opened WP-01 `#3346`.
- Batch 1 opened Sprint 1 `#3347` and WP-02 through WP-04 as `#3348` through
  `#3350`; Sprint 1 is now closed after `#3346`, `#3348`, `#3349`, and `#3350`
  each merged and closed out cleanly.
- Batch 2 opened Sprint 2 `#3352` and WP-05 through WP-08 as `#3353` through
  `#3356`; child execution now waits only on Sprint 2's own dependency and
  closeout gates because Sprint 1 is already closed.
- Batch 3 opened Sprint 3 `#3357` and WP-09 through WP-12 as `#3358` through
  `#3361`; child execution now waits on Sprint 2 sequencing and its own
  dependency gates rather than on Sprint 1 opening.
- Batch 4 opened Sprint 4 `#3362` and WP-13 through WP-21 as `#3363`
  through `#3371`; child execution now waits on prior sprint closeout and its
  own dependency gates rather than on WP-01 opening.
- Batch 5 opened the CodeFriend sidecar `#3372` and CF-PRE-01 through
  CF-PRE-04 as `#3373` through `#3376`; sidecar child execution now waits on
  its own routing/dependency gates and remains non-core.
- Batch 6 opened the WildClawBench benchmark spike sidecar `#3378` and
  WC-PRE-01 through WC-PRE-04 as `#3379` through `#3382`; sidecar child
  execution now waits on its own routing/dependency gates and remains
  non-core.
- Standalone first-birthday readiness side issue `#3377` is promoted for v0.92
  launch preparation and should feed WP-19/WP-20; it is not a sprint child and
  does not alter the v0.91.4 release-tail sequence.

Every opened issue receives all five cards upfront. `SIP`, `STP`, and `SPP`
must be design-time ready before execution starts; `SRP` and `SOR` remain
present but truthful to pre-review and pre-output lifecycle state.

## Sprint Goals

The sprint overview table above is the generator-facing sprint map. The goals
below explain the intended execution posture for each sprint without replacing
that canonical table.

## Sprint Goal

v0.91.4 should make the C-SDLC default lane reliable enough that future ADL
software-development work can use it without special ceremony or hidden local
state.

## Planned Scope

- Sprint 1: lifecycle, validator, conductor, doctor, and editor hardening.
- Sprint 2: actor standing, shard ownership, evidence convergence, signed
  trace, merge readiness, and memory handoff.
- Sprint 3: sprint default behavior, active issue migration, repeatability
  metrics, validation-tail/proof-latency handling, and drift fixtures.
- Sprint 4: proof coverage, quality gate, docs/adoption review, internal
  review, external review, remediation, next-milestone planning,
  next-milestone review, and release ceremony.
- Sidecar mini-sprint: bounded CodeFriend pre-alpha repo and static-site setup.

## Work Plan

Execute sprints in order unless a documented dependency gate requires a
bounded sidecar issue to run after WP-01. Do not let the sidecar replace,
delay invisibly, or expand the C-SDLC release-tail gates.

## Execution Policy

Every sprint must preserve:

- conductor routing for every lifecycle stage
- editor-only card edits
- bound worktree execution
- pre-PR review before publication
- closeout after issue closure
- combined-lane validation where integration risk exists
- proof coverage, quality gate, docs/adoption review, internal review,
  external review, remediation, next-milestone planning, next-milestone review
  pass, and release ceremony remain separate ordered tail issues
- sidecar product work remains separate from C-SDLC default-operation proof

## Cadence Expectations

- Keep issue execution bounded and reviewable.
- Prefer focused proof over broad test cycles when the touched surface is docs
  or planning only.
- Record validation-tail and proof-latency evidence instead of hiding long
  blocking checks behind the five-minute-sprint claim.

## Risks / Dependencies

- Sprint 1 must land before later sprints rely on default C-SDLC state truth.
- Signed trace and tracked workflow-state proof must land before release
  readiness is claimed.
- The CodeFriend sidecar is complete with a verified HTTPS landing surface and
  truthful handoff; later CodeFriend alpha work remains outside this mini-sprint.

## Demo / Review Plan

- Sprint 3 owns repeatability and validation-tail/proof-latency evidence.
- Sprint 4 owns demo/proof coverage, quality gate, docs/adoption review,
  internal review, external review, remediation, next-milestone planning,
  next-milestone review, and ceremony.
- CodeFriend sidecar proof is reviewed as product setup evidence only.

## Closeout Bar

The milestone is not complete merely because the process is documented.

It must show that the C-SDLC lane can run repeatedly with:

- correct card creation
- correct routing
- correct actor standing and shard ownership
- correct review recording
- correct SOR closeout
- correct sprint state
- correct memory handoff boundary
- measured coordination, validation-tail, proof-latency, and parallel-validation
  behavior

The CodeFriend sidecar is complete only when it has either a verified HTTPS
welcome page and handoff record or a truthful blocked handoff with AWS/DNS
approval blockers recorded.

## Exit Criteria

- Sprint umbrellas are opened with ordered child work and complete structured
  prompts.
- Sprint state cannot advance past child issues without closeout truth.
- v0.91.4 closes only after the full release-tail sequence completes.
