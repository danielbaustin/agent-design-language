# Sprint Plan - v0.87.1

## Metadata
- Sprint plan: `v0.87.1`
- Milestone: `v0.87.1`
- Start date: `TBD`
- End date: `TBD`
- Owner: `Daniel Austin`

## How To Use
- Keep scope small enough to finish with green CI and merged PRs.
- List work items in planned execution order.
- Track blockers here (not scattered chat notes).

## Role Of This Document

This sprint plan defines the executable structure of `v0.87.1`.

It maps Work Packages from the WBS into ordered implementation and review phases that can be executed deterministically.

This document must remain fully consistent with:
- the WBS
- the milestone vision
- the milestone design
- the milestone README

Any divergence is a defect and must be corrected immediately.

## Working Rules

- Execute work in strict dependency order.
- Every WP must produce concrete runtime behavior, docs, artifacts, demos, or bounded review surfaces.
- Demos, traces, and structured review outputs are required evidence for major runtime claims.
- Keep all canonical documents aligned continuously.
- Do not defer consistency work to the end.
- Treat runtime completion as the milestone center of gravity.

## Sprint Goal
Complete the first full runtime milestone for ADL: execution environment, lifecycle, trace alignment, resilience, operator surfaces, demos, review, release, and handoff.

## Planned Scope
- runtime environment, lifecycle, and execution-boundary completion
- deterministic trace-aligned execution and review surfaces
- local runtime resilience, Shepherd preservation, and state discipline
- operator surfaces and a large multi-demo proof program
- docs/review convergence, internal review, external review prep, remediation, release, and next-milestone handoff

## Sprint Structure

### Sprint 1 - Runtime Foundations
Goal:
Establish canonical milestone truth, runtime environment, lifecycle, trace alignment, resilience, and operator surfaces.

Current scope:
- `WP-01` design pass - `#1435`
- `WP-02` runtime environment completion - `#1436`
- `WP-03` execution boundaries and lifecycle - `#1437`
- `WP-04` trace-aligned runtime execution - `#1438`
- `WP-05` local runtime resilience + Shepherd preservation - `#1439`
- `WP-06` operator surfaces - `#1440`
- `WP-07` runtime state / persistence discipline - `#1441`
- `WP-08` runtime review surfaces - `#1442`

Proof surfaces:
- one authoritative runtime-environment surface
- bounded lifecycle phases with real execution boundaries
- trace/runtime alignment that can be inspected by a reviewer
- local resilience, Shepherd preservation, and restart semantics visible in implementation or artifacts
- stable operator entrypoints and review surfaces

### Sprint 2 - Convergence, Demos, And Quality
Goal:
Prove the runtime through integrated demos, quality gates, and doc/review convergence.

Current scope:
- `WP-09` cross-document consistency pass
- `WP-10` acceptance criteria finalization
- `WP-11` sprint plan alignment
- `WP-12` checklist / release-gate completion
- `WP-13` demo matrix and integration demos
- `WP-14` coverage / quality gate
- `WP-15` docs + review pass

Proof surfaces:
- canonical docs no longer contradict the runtime implementation
- demo matrix defines and proves the runtime claims
- quality posture is auditable
- reviewer entry surfaces are coherent for an uninvolved reviewer

### Sprint 3 - Review, Release, And Handoff
Goal:
Review the completed runtime milestone, remediate findings, release it, and prepare the next milestone.

Current scope:
- `WP-16` internal review
- `WP-17` external / 3rd-party review preparation
- `WP-18` review findings remediation
- `WP-19` next milestone planning
- `WP-20` release ceremony

Proof surfaces:
- internal review findings are recorded and actionable
- external review package is legible and runnable
- accepted findings are remediated or explicitly deferred
- release artifacts are complete
- next-milestone planning package exists before closeout

## Cadence Expectations
- Use issue cards (`input`/`output`) for each item.
- Keep changes scoped per issue; use draft PRs until checks pass.
- Run required quality gates (`fmt`, `clippy`, `test`, demo validators, and milestone review checks) for the changed surfaces.
- Sprint 1 issue references are now assigned; later WPs may remain `TBD` until their planning pass opens.

## Risks / Dependencies
- Dependency: upstream v0.87 artifacts (trace, provider, skills) remain stable
  - Risk: doc drift or mismatch with implemented substrate
  - Mitigation: anchor language to v0.87 canonical docs and avoid speculative claims

- Dependency: runtime work can sprawl across lifecycle, state, review, and operator surfaces
  - Risk: the milestone lands partial runtime slices without one coherent proof story
  - Mitigation: keep demo matrix, quality gate, and review surfaces centered on the integrated runtime

## Demo / Review Plan
- Demo artifact: milestone doc walkthrough (VISION → DESIGN → WBS → SPRINT → CHECKLIST)
- Primary runtime proof program: `docs/milestones/v0.87.1/DEMO_MATRIX_v0.87.1.md`
- Review date: TBD
- Sign-off owners: Daniel Austin, internal review, 3rd party reviewer

## Exit Criteria
- All WPs (`WP-01` through `WP-20`) are complete or explicitly deferred with rationale.
- Linked issues/PRs updated and traceable.
- CI is green for merged work.
- Demo matrix, quality gate, and review surfaces are complete.
- Sprint summary is captured in milestone docs.
