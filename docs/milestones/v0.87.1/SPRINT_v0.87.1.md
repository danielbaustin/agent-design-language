# Sprint Plan - v0.87.1

## Metadata
- Sprint plan: `v0.87.1`
- Milestone: `v0.87.1`
- Start date: `TBD`
- End date: `TBD`
- Owner: `Daniel Austin`

## How To Use
- Keep scope small enough to finish with green CI and merged PRs.
- List work items in planned execution order and keep that order aligned to the WBS dependency chain.
- Track blockers here (not scattered chat notes).
- Use the WBS Acceptance Mapping as the canonical definition of what each phase is proving.

## Role Of This Document

This sprint plan defines the executable structure of `v0.87.1`.

It maps Work Packages from the WBS into ordered implementation and review phases that can be executed deterministically.

This document must remain fully consistent with:
- the WBS
- the milestone vision
- the milestone design
- the milestone README
- the release-tail issue graph and acceptance contract

Any divergence is a defect and must be corrected immediately.

## Working Rules

- Execute work in strict dependency order.
- Every WP must produce concrete runtime behavior, docs, artifacts, demos, or bounded review surfaces.
- Demos, traces, and structured review outputs are required evidence for major runtime claims.
- Keep all canonical documents aligned continuously.
- Do not defer consistency work to the end.
- Treat runtime completion as the milestone center of gravity.
- Do not start a downstream sprint slice until the documented handoff gate for the current slice is satisfied.

## Sprint Goal
Complete the first full runtime milestone for ADL: execution environment, lifecycle, trace alignment, resilience, operator surfaces, demos, review, release, and handoff.

## Planned Scope
- runtime environment, lifecycle, and execution-boundary completion
- deterministic trace-aligned execution and review surfaces
- local runtime resilience, Shepherd preservation, and state discipline
- operator surfaces and a large multi-demo proof program
- docs/review convergence, internal review, external / 3rd-party review, remediation, release, and next-milestone handoff

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

Execution order:
- `#1435` establishes canonical milestone truth before implementation slices begin.
- `#1436` through `#1442` then execute in dependency order so later lifecycle, trace, resilience, state, and review work consume one runtime substrate instead of inventing parallel ones.

Handoff gate to Sprint 2:
- Sprint 1 exits only when the runtime foundation issues have landed and the milestone docs can truthfully describe the implemented runtime surfaces consumed by WP-09 through WP-12.

### Sprint 2 - Convergence, Demos, And Quality
Goal:
Prove the runtime through integrated demos, quality gates, and doc/review convergence.

Current scope:
- `WP-09` cross-document consistency pass - `#1458`
- `WP-10` acceptance criteria finalization - `#1459`
- `WP-11` sprint plan alignment - `#1460`
- `WP-12` checklist / release-gate completion - `#1461`
- `WP-13` demo matrix and integration demos - `#1462`
- `WP-14` coverage / quality gate - `#1463`
- `WP-15` docs + review pass - `#1464`

Proof surfaces:
- canonical docs no longer contradict the runtime implementation
- demo matrix defines and proves the runtime claims
- quality posture is auditable
- reviewer entry surfaces are coherent for an uninvolved reviewer

Execution order:
- Convergence gate: `#1458` then `#1459`
  - `#1458` removes cross-document contradictions.
  - `#1459` finalizes the measurable acceptance contract that the rest of Sprint 2 must consume.
- Planning and release-surface gate: `#1460` then `#1461`
  - `#1460` aligns sprint sequencing to the accepted WBS dependency order and release-tail structure.
  - `#1461` turns that order into an executable checklist and release-gate surface.
- Integrated proof and reviewer-convergence gate: `#1462` then `#1463` then `#1464`
  - `#1462` defines and validates the milestone demo program against the accepted runtime claims.
  - `#1463` records the auditable quality posture for the implemented runtime and demo surfaces.
  - `#1464` converges reviewer entry docs only after demo and quality evidence are already concrete.

Handoff gate to Sprint 3:
- Sprint 2 exits only when docs, checklist/release gates, demo coverage, quality posture, and reviewer entry surfaces all point back to the same accepted runtime truth.

### Sprint 3 - Review, Release, And Handoff
Goal:
Review the completed runtime milestone, remediate findings, release it, and prepare the next milestone.

Current scope:
- `WP-16` internal review - `#1494`
- `WP-17` external / 3rd-party review preparation - `#1495`
- `WP-18` review findings remediation - `#1496`
- `WP-19` next milestone planning - `#1497`
- `WP-20` release ceremony - `#1498`

Proof surfaces:
- internal review findings are recorded and actionable
- external review package is legible and runnable
- accepted findings are remediated or explicitly deferred
- release artifacts are complete
- next-milestone planning package exists before closeout

Execution order:
- `#1494` records internal findings against the converged runtime proof package.
- `#1495` prepares the external review surface after internal review has exposed obvious defects.
- `#1496` remediates accepted findings or records explicit deferrals before release work begins.
- `#1497` prepares next-milestone planning before release closeout so follow-on work is captured while the review context is still fresh.
- `#1498` performs the final release ceremony only after the planning package and remediation state are explicit.

Release-tail rule:
- Sprint 3 must preserve the WBS release-tail order `internal review -> external / 3rd-party review -> findings remediation -> next milestone planning -> release ceremony` without collapsing those steps into one undocumented closeout pass.

## Cadence Expectations
- Use issue cards (`input`/`output`) for each item.
- Keep changes scoped per issue; use draft PRs until checks pass.
- Run required quality gates (`fmt`, `clippy`, `test`, demo validators, and milestone review checks) for the changed surfaces.
- Sprint 1, Sprint 2, and Sprint 3 issue references are now assigned; later sprints should remain review-ready until their documented handoff gates are satisfied.

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
- Acceptance anchor: `docs/milestones/v0.87.1/WBS_v0.87.1.md`
- Review date: TBD
- Sign-off owners: Daniel Austin, internal review, 3rd party reviewer

## Exit Criteria
- All WPs (`WP-01` through `WP-20`) are complete or explicitly deferred with rationale.
- Linked issues/PRs updated and traceable.
- CI is green for merged work.
- Demo matrix, quality gate, and review surfaces are complete.
- Sprint summary is captured in milestone docs.
