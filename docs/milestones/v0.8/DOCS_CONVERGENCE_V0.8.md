# v0.8 Documentation Pass and Review Convergence

This document records the current v0.8 docs-convergence state for the review tail.

It is a review-packet status artifact for `#707` and does not add runtime scope.

## Convergence Outcome

Status: **Partially converged; version story mostly reconciled, final review handoff still pending**

Primary result:

- milestone navigation and core planning surfaces are aligned,
- bounded runtime/demo work is now represented in the packet,
- recovery-tail docs have been refreshed toward current repository truth,
- README/manifests/reviewer-facing status language now describes one unreleased v0.8 main-branch story layered on top of the latest tagged v0.7.0 release,
- remaining review-tail blockers are explicitly listed rather than treated as non-blocking.

## Surfaces Reviewed

| Surface | Current status | Notes |
|---|---|---|
| README/navigation index | partially converged | navigation is canonical, but reviewer handoff is still blocked by remaining review-tail work |
| recovery audit | refreshed | now reflects bounded implemented runtime/demo surfaces |
| internal readiness review | refreshed | still useful, but remains a blocker-oriented internal gate |
| execution/dependency docs | converged | execution order and boundaries are aligned |
| Gödel loop docs/templates | converged | stage ordering and schema references are aligned |
| authoring/reviewer planning docs | converged | sequencing and template-location docs are present |
| bounded AEE + quality gate docs | converged | bounded scope and gate phases are explicit in planning docs |
| demo/review/release-tail docs | partially converged | runnable-vs-inspect boundary is clearer, but final handoff is still incomplete |

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

## Convergence Work Completed

1. Ensured canonical v0.8 navigation references core docs consistently.
2. Verified cross-doc references used for review/release planning are present.
3. Confirmed no absolute host-path leakage and no stale pre-canonical planning-path references in audited docs.
4. Corrected handoff references that pointed to non-existent docs to use existing canonical milestone surfaces.
5. Refreshed the review-tail packet so bounded runtime/demo work is represented explicitly.
6. Moved reviewer-facing status language toward one unreleased-v0.8 story layered on top of the latest tagged v0.7.0 release.

## Remaining Reviewer-Facing Blockers

1. **Version story is still mixed**
   - `swarm/Cargo.toml` declares `0.8.0`.
   - `swarm/README.md` still describes the runtime as `v0.7.0`.

2. **Third-party handoff packet is incomplete**
   - `docs/milestones/v0.8/THIRD_PARTY_REVIEW_V0.8.md` is still absent.

3. **Review packet still requires a clean external entry flow**
   - Runnable demos versus inspect-only surfaces must remain explicit anywhere the review packet is handed to a third party.

4. **Review-tail closeout is still pending**
   - Third-party findings triage after `#707` and final release-tail cleanup have not been completed yet.

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
