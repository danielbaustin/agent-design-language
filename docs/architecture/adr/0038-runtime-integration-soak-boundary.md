# ADR 0038 Candidate: Runtime Integration Soak Boundary

- Status: Candidate
- Target milestone: v0.91.6
- Related issues: #4177, #4241, #4245
- Related ADRs: ADR 0011, ADR 0024, ADR 0028
- Source evidence:
  - `docs/milestones/v0.91.6/RUNTIME_INTEGRATION_SOAK_SPRINT_v0.91.6.md`
  - `docs/milestones/v0.91.6/RUNTIME_FIRE_UP_PLAN_v0.91.6.md`
  - `docs/milestones/v0.91.6/features/TOKIO_RUNTIME_SUBSTRATE_v0.91.6.md`
  - `docs/milestones/v0.91.6/review/V0916_TOKIO_RUNTIME_SUBSTRATE_SPRINT_REVIEW_4177.md`
  - `docs/milestones/v0.91.6/review/V0916_RUNTIME_RESILIENCE_FOLLOW_ON_SPRINT_REVIEW_4241.md`
  - `docs/milestones/v0.91.6/review/runtime/V0916_INTEGRATED_RUNTIME_SOAK_PROOF_4245.md`

## Context

v0.91.6 implemented and reviewed several runtime pieces: Tokio substrate work,
resilience follow-on work, runtime fire-up planning, and an integrated soak
proof surface. The operator explicitly called out the remaining risk: separate
pieces on the floor are not the same as an integrated runtime.

Before v0.92 can claim runtime coherence, ADL needs integrated soak evidence
that proves the runtime can run for a meaningful duration with the relevant
components wired together.

## Decision

ADL should gate runtime-coherence claims on integrated soak evidence.

Soak #1 in v0.91.6 should prove a walking skeleton: Tokio runtime substrate,
bounded runtime fire-up, lifecycle-safe logging, resilience behavior, and
minimum integration proof.

Soak #2 in v0.91.7 should prove feature-list integration before v0.92,
including the runtime surfaces expected by the v0.92 activation map. If Soak
#2 exposes blockers that need another pass, Soak #3 may be created in v0.91.7
before v0.92 opens.

Component-level proof is necessary but insufficient for runtime coherence.

## Consequences

### Positive

- Prevents v0.92 from inheriting untested integration assumptions.
- Makes long-running runtime behavior an explicit proof surface.
- Gives runtime, ACIP, AEE, scheduler, observability, resilience, and provider
  work a shared convergence gate.

### Negative

- Runtime feature work may be delayed until soak proof catches up.
- Soak failures create real milestone blockers instead of being hidden as
  isolated component defects.
- Long-running proof needs careful resource and log hygiene.

## Alternatives Considered

### Accept component-level proof as sufficient

This is rejected. The runtime risk is specifically in integration behavior,
not only in individual modules.

### Defer soak until v0.92

This would make v0.92 the discovery milestone for integration failures, which
conflicts with the bridge-tranche purpose of v0.91.6 and v0.91.7.

## Validation Notes

Promotion should review the soak sprint plan, runtime fire-up plan, Tokio
feature doc, runtime sprint reviews, and retained soak proof. The accepted ADR
must not claim every feature is already integrated unless Soak #2 or #3 proves
that state.

## Non-Claims

- This ADR does not claim full v0.92 runtime readiness.
- This ADR does not implement AEE, ACIP, Observatory, scheduler, or provider
  integration by itself.
- This ADR does not make a short smoke test equivalent to a soak.
- This ADR does not require indefinite unattended runtime execution.
