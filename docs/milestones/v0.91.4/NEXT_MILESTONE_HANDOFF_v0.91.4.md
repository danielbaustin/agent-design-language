# v0.91.4 Next Milestone Handoff

## Status

Planned handoff scaffold for `WP-19`.

This document is present before v0.91.4 starts so the release tail has a named,
tracked home for next-milestone planning. It must be refreshed during `WP-19`
after the v0.91.4 implementation, quality gate, reviews, and remediation are
complete.

Do not treat this scaffold as the final next-milestone decision.

## Purpose

v0.91.4 completes the C-SDLC rollout. The next milestone should be planned from
the evidence produced by that completion work, not from chat memory or local
drafts.

By `WP-19`, this handoff should answer:

- what milestone comes next
- which completed v0.91.4 capabilities are safe to rely on
- which C-SDLC default-operation claims are proven, blocked, or deferred
- which open findings, follow-ons, or residual risks must be carried forward
- whether CodeFriend, product work, social-cognition work, or other roadmap
  items are ready to execute under the stabilized C-SDLC lane

## Current Planning Assumption

The immediate purpose of v0.91.4 is not to start the next product or research
wave early. It is to make future ADL software-development work run through a
tracked, auditable C-SDLC default lane.

The next milestone should therefore consume v0.91.4 outputs only after these
conditions are true:

- new ADL software-development issues default to `SIP -> STP -> SPP -> SRP -> SOR`
- durable cards, sprint state, review, proof, trace, closeout, and release
  evidence are tracked in Git
- `SPP` is tracked as issue-local operative execution-plan truth
- `SRP` records review-result truth and `SOR` records outcome/integration truth
- signed trace proof exists for durable C-SDLC runs, or blockers are explicit
- ObsMem consumes tracked evidence rather than untracked local lore
- sprint closeout cannot advance over stale child issue truth
- five-minute-sprint repeatability metrics are recorded without weakening
  governance or review

## Candidate Downstream Inputs

`WP-19` should inspect these sources before naming the next milestone work:

- v0.91.4 completed issue wave and closeout records
- v0.91.4 demo matrix and feature proof coverage
- v0.91.4 quality gate
- v0.91.4 internal and external review packets
- v0.91.4 remediation dispositions
- tracked C-SDLC workflow records under `workflow/c-sdlc/v0.91.4/`
- signed trace verification evidence
- ObsMem transition-memory ingestion evidence
- `docs/planning/ADL_FEATURE_LIST.md`
- `docs/planning/codefriend/`
- current backlog and open issue state

## Required WP-19 Update

During `WP-19`, replace this section with the actual handoff:

- selected next milestone
- selected scope
- non-goals
- issue-wave candidates
- feature docs or ADRs required before execution
- review and demo expectations
- known blockers
- residual risks
- explicit downstream owner issue or milestone package

## WP-20 Review Requirement

`WP-20` must re-review this handoff before the release ceremony.

That review should confirm:

- the handoff reflects actual v0.91.4 evidence
- the next milestone scope is not inflated by unresolved v0.91.4 work
- C-SDLC default-operation claims are supported by tracked proof
- any CodeFriend or product work remains separated from C-SDLC core claims
- deferred work has issue or backlog routing

## Non-Claims

This scaffold does not claim:

- v0.91.4 is complete
- the next milestone has already been selected
- CodeFriend execution is the next immediate milestone
- C-SDLC is already default for all future ADL work
- signed trace proof or ObsMem ingestion has already passed

## Review Notes

This document is intentionally conservative. Its job before v0.91.4 starts is
to make the handoff surface present, discoverable, and tied to the release
cycle. Its job during `WP-19` is to become the actual next-milestone planning
record.
