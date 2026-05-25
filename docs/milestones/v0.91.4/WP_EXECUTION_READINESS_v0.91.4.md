# v0.91.4 WP Execution Readiness

## Status

Opening readiness surface. WP-01 `#3346` has opened the milestone and seeded
Sprint 1 as the first controlled issue/card batch.

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

- Batch 0: WP-01 `#3346` seeded and ready.
- Batch 1: Sprint 1 `#3347` plus WP-02 `#3348`, WP-03 `#3349`, and WP-04
  `#3350` seeded and ready.
- Batch 2: Sprint 2 queued.
- Batch 3: Sprint 3 queued.
- Batch 4: Sprint 4 queued.
- Batch 5: CodeFriend sidecar queued after WP-01.

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
