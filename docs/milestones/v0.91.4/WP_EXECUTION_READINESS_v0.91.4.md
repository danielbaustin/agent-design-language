# v0.91.4 WP Execution Readiness

## Status

Post-Sprint-1 readiness surface. WP-01 `#3346` opened the milestone and Sprint
1 is now closed; Sprint 2, Sprint 3, Sprint 4, the CodeFriend sidecar, and the
WildClawBench benchmark spike sidecar remain seeded as controlled issue/card
batches.

## Entry Bar

v0.91.4 should not begin execution until v0.91.3 has produced a reviewed first
C-SDLC transition slice or an explicit no-go/defer decision.

## Required Readiness

Before any WP starts:

- issue cards exist and validate
- every opened issue has all five cards: `SIP`, `STP`, `SPP`, `SRP`, and `SOR`
- `SIP`, `STP`, and `SPP` are issue-specific and design-time ready before
  execution starts
- `SRP` and `SOR` exist upfront but do not claim review or output completion
  before their lifecycle stages
- `workflow-conductor` routes the issue
- execution is bound to a worktree
- editor skills own card edits
- pre-PR review is scheduled before publication
- closeout responsibility is explicit
- durable workflow records are expected to be tracked, not left only in local
  `.adl` state
- new durable C-SDLC records target `docs/milestones/v0.91.4/review/evidence/csdlc/` unless a WP
  explicitly documents a narrower migration boundary
- the planned closeout tail preserves separate ordered work for proof coverage,
  quality gate, docs/adoption review, internal review, external review,
  remediation, next-milestone planning, next-milestone review, and release
  ceremony

## Opening Batch State

WP-01 uses sprint-sized issue opening batches to keep the upfront-card rule
safe:

- Batch 0: WP-01 `#3346` seeded, executed, and closed out.
- Batch 1: Sprint 1 `#3347` plus WP-02 `#3348`, WP-03 `#3349`, and WP-04
  `#3350` seeded, executed, and closed out cleanly.
- Batch 2: Sprint 2 `#3352` plus WP-05 `#3353`, WP-06 `#3354`, WP-07
  `#3355`, and WP-08 `#3356` seeded and card-ready; child execution now waits
  on Sprint 2's own dependency and closeout gates because Sprint 1 is already
  closed.
- Batch 3: Sprint 3 `#3357` plus WP-09 `#3358`, WP-10 `#3359`, WP-11
  `#3360`, and WP-12 `#3361` seeded and card-ready; child execution now waits
  on Sprint 2 sequencing and its own dependency gates rather than on Sprint 1
  opening.
- Batch 4: Sprint 4 `#3362` plus WP-13 `#3363`, WP-14 `#3364`, WP-15
  `#3365`, WP-16 `#3366`, WP-17 `#3367`, WP-18 `#3368`, WP-19 `#3369`,
  WP-20 `#3370`, and WP-21 `#3371` seeded and card-ready; child execution now
  waits on prior sprint closeout and its own dependency gates rather than on
  WP-01 opening.
- Batch 5: CodeFriend sidecar `#3372` plus CF-PRE-01 `#3373`, CF-PRE-02
  `#3374`, CF-PRE-03 `#3375`, and CF-PRE-04 `#3376` seeded and card-ready;
  sidecar child execution now waits on its own routing/dependency gates and
  remains non-core.
- Batch 6: WildClawBench benchmark spike sidecar `#3378` plus WC-PRE-01
  `#3379`, WC-PRE-02 `#3380`, WC-PRE-03 `#3381`, and WC-PRE-04 `#3382`
  seeded and card-ready; sidecar child execution now waits on its own
  routing/dependency gates and remains non-core.
- Standalone side issue: first-birthday readiness `#3377` is promoted for
  v0.92 launch preparation and should feed WP-19/WP-20 next-milestone
  planning/review; it is not a C-SDLC release-tail child and does not change
  the core sprint sequence.

Planned side issues remain list-only until the operator explicitly promotes
them.

## Completion-Specific Readiness

Every implementation WP must identify:

- which C-SDLC lifecycle state it hardens
- which validator/doctor/conductor/editor surface it changes
- which regression fixture proves old drift does not recur
- whether combined-lane validation is required
- how SRP/SOR truth flows into memory handoff
- which actors participate, which roles they hold, and what evidence supports
  their transition standing
- how durable proof is tracked in Git under `docs/milestones/v0.91.4/review/evidence/csdlc/`
- whether the WP must emit or verify a signed trace bundle
- how tracked evidence feeds ObsMem
