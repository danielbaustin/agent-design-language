# Card Lifecycle Integration

## Status

Planned `v0.91.3` feature under `WP-03` / `#3201`.

## Purpose

Make the corrected C-SDLC card sequence operational inside the first
Cognitive State Transition slice.

The sequence is:

```text
SIP -> STP -> SPP -> SRP -> SOR
```

This feature exists because the first C-SDLC proof is not credible if the card
model is merely documented. The tooling, prompts, validators, and review
records must all preserve the same lifecycle truth.

## Scope

The first slice must define and prove:

- `SIP` as issue intent, scope, dependencies, and acceptance boundary
- `STP` as the selected task and work product target
- `SPP` as the issue-local operative execution plan
- `SRP` as the review prompt plus review results, findings, dispositions, and
  residual risk
- `SOR` as the outcome record for actual changes, validation, integration
  state, and final issue truth

## Acceptance Criteria

- New issue bundles created for the first-slice proof use only the canonical
  card sequence.
- Validators reject legacy `SRP` policy semantics for new C-SDLC bundles.
- Workflow-conductor routing can identify the next lifecycle state from card
  truth without relying on oral context.
- Editor skills remain the only allowed card-normalization path.
- The final proof packet shows the cards feeding review, merge readiness,
  closeout, and memory handoff.
- The proof preserves `SPP` as issue-local plan truth and does not use it as
  sprint orchestration, review-result truth, or outcome truth.
- Any durable C-SDLC `SPP` surface used by the proof is tracked, public, and
  auditable rather than only local `.adl/` state.
- The first-slice proof names `workflow/c-sdlc/v0.91.3/issues/` as the target
  tracked namespace for durable issue-local card records.

## Non-Goals

- This feature does not make C-SDLC default operation for all ADL issues.
- This feature does not migrate every historical card bundle.
- This feature does not bypass GitHub issue, PR, CI, or human review truth.
- This feature does not create sprint-scoped `SPP` as the C-SDLC orchestration
  primitive.
