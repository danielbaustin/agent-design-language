# v0.8 Documentation Pass and Review Convergence

This document records the current v0.8 docs-convergence state for the review tail.

It is a review-packet status artifact for `#707` and does not add runtime scope.

## Convergence Outcome

Status: **Partially converged; not yet ready for third-party handoff**

Primary result:
- milestone navigation and core planning surfaces exist and are cross-linked,
- bounded runtime/demo work is now represented in the packet,
- recovery-tail docs have been refreshed toward current repository truth,
- reviewer-facing blockers still remain and must be resolved before external handoff.

## Surfaces Reviewed

| Surface | Current status | Notes |
|---|---|---|
| README/navigation index | refreshed | navigation is canonical; review packet needs explicit runnable-vs-inspect guidance |
| recovery audit | refreshed | now reflects bounded implemented runtime/demo surfaces |
| internal readiness review | refreshed | still useful, but remains a blocker-oriented internal gate |
| execution/dependency docs | present | useful as planning/reference surfaces |
| Gödel loop docs/templates | present | schema/template spine exists and is cross-referenced |
| authoring/reviewer planning docs | present | contract surfaces exist; not all are runtime-implemented |
| demo/review/release-tail docs | partially converged | runnable-vs-inspect boundary and version truth still need explicit reviewer clarity |

## Current Convergence Truth

### Converged enough to keep as canonical
- `docs/milestones/v0.8/README.md`
- `docs/milestones/v0.8/EXECUTION_ORDER_V0.8.md`
- `docs/milestones/v0.8/GODEL_SCHEMA_DELIVERY_ORDER_V0.8.md`
- `docs/milestones/v0.8/BOUNDED_AEE_V1_SCOPE_V0.8.md`
- schema/spec docs under `docs/milestones/v0.8/`

### Refreshed in this review-tail cycle
- `docs/milestones/v0.8/RECOVERY_AUDIT_V0.8.md`
- `docs/milestones/v0.8/DOCS_CONVERGENCE_V0.8.md`
- `docs/milestones/v0.8/INTERNAL_READINESS_REVIEW_V0.8.md`
- `docs/milestones/v0.8/DEMOS_V0.8.md`

### Not yet converged for external handoff
- version/release-status messaging across reviewer-facing surfaces
- final third-party review artifact packaging
- final review packet simplification for an external reader with no prior context

## Remaining Reviewer-Facing Blockers

1. **Version story is still mixed**
   - `swarm/Cargo.toml` declares `0.8.0`.
   - `swarm/README.md` still describes the runtime as `v0.7.0`.

2. **Third-party handoff packet is incomplete**
   - `docs/milestones/v0.8/THIRD_PARTY_REVIEW_V0.8.md` is still absent.

3. **Review packet still requires a clean external entry flow**
   - Runnable demos versus inspect-only surfaces must remain explicit anywhere the review packet is handed to a third party.

## Handoff Guidance

Do not treat this doc as evidence that v0.8 is fully converged for review.

Instead, use it as a status checkpoint:
- docs packet is materially closer to repository truth,
- bounded runtime/demo work is now acknowledged,
- external handoff should wait until the remaining reviewer-facing blockers are resolved.

## Out of Scope

- implementing new runtime features,
- adding v0.85 work,
- declaring review readiness before the remaining blockers are removed.
