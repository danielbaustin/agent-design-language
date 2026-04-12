# Release Notes - v0.87.1

## Metadata
- Product: `ADL`
- Version: `v0.87.1`
- Release date: `Pending release gate`
- Tag: `v0.87.1`

## How To Use
- Keep statements implementation-accurate and test-validated.
- Prefer concise bullets over marketing language.
- Explicitly separate shipped behavior from "What's Next."
- Treat these notes as a pre-release draft until the checklist, review, remediation, planning, and release ceremony gates are complete.

# `ADL` `v0.87.1` Release Notes

## Summary
`v0.87.1` is the release vehicle for ADL's first full runtime milestone. The milestone work has turned the seeded execution substrate from `v0.87` into a coherent runtime system with explicit lifecycle boundaries, trace-aligned execution, local resilience, operator surfaces, review surfaces, and a substantial bounded proof program; final release wording remains pending until the final release ceremony gate is complete or explicitly deferred.

## Highlights
- Runtime environment and lifecycle completion surfaces landed for `v0.87.1`
- Deterministic trace-aligned runtime and local resilience surfaces are part of the milestone scope
- Demo-matrix proof surfaces are aligned to a real runtime implementation milestone, while the final checklist and release ceremony actions remain release-gated until `WP-20` closes

## What's New In Detail

### Runtime Completion Milestone
- Established `v0.87.1` as the milestone for runtime completion rather than a documentation-only follow-on
- Aligned the canonical milestone docs (`VISION`, `DESIGN`, `WBS`, `SPRINT`, `DEMO_MATRIX`, `MILESTONE_CHECKLIST`, `RELEASE_PLAN`, `DECISIONS`, `RELEASE_NOTES`) to the real runtime implementation path, including `docs/milestones/v0.87.1/DEMO_MATRIX_v0.87.1.md`
- Kept release-note claims pre-release scoped until the remaining milestone tail issues are complete or explicitly dispositioned

### Runtime Scope
- Framed `v0.87.1` around runtime environment, execution boundaries, lifecycle, trace alignment, local resilience, operator surfaces, and runtime review surfaces
- Preserved later cognitive features as out of scope so the milestone stays centered on runtime completion

### Proof, Review, And Release Discipline
- Defined a substantial runtime demo program and reviewer entry surfaces for the milestone
- Preserved explicit internal review, external review, remediation, next-milestone planning, and release ceremony steps in the closeout tail, with only the final ceremony step still open

## Upgrade Notes
- `v0.87.1` is expected to ship real runtime implementation and proof surfaces, so upgrade notes should be finalized from the actual landed behavior before release
- Existing `v0.87` implementation surfaces remain the starting substrate, but `v0.87.1` is intended to supersede them with a completed runtime story

## Known Limitations
- Final release text must be updated from actual landed runtime behavior before publish
- Some runtime surfaces may still be intentionally deferred if bounded follow-on issues are recorded explicitly
- The milestone is not release-complete until the checklist and final release ceremony gates are closed

## Validation Notes
- Validation for this release must cover milestone docs, runtime demos, review surfaces, and standard repository quality gates
- `cargo build` is a baseline validation command, not the only proof surface for the milestone

## What's Next
- Carry forward deferred runtime work and the next milestone package after `v0.87.1` closeout
- Carry the project forward into `v0.88` once runtime substrate work is complete

## Exit Criteria
- Notes reflect only shipped behavior.
- Known limitations and future work are explicitly separated.
- Final text is ready to paste into GitHub Release UI without further editing.
