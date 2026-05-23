# v0.91.2 ADR Plan

## Status

Tracked ADR authoring plan for `v0.91.2`.

This plan did not accept any ADR by itself. It recorded the candidate ADR set
for review. ADR 0020 through ADR 0028 were promoted to accepted records in
`docs/adr/` during the v0.91.3 review tail.

## Purpose

`v0.91.2` turned several implicit architecture boundaries into explicit
decision surfaces. The milestone needed ADR candidates for the tooling split,
runtime proposal/commit boundaries, GWS promotion boundaries, workflow control
plane, productization, repo visibility, modernization, and the C-SDLC
tracked-state direction.

The growth in ADR count is a signal: ADL is making more architecture explicit,
reviewable, and durable.

## Existing Baseline

Accepted ADRs currently live in `docs/adr/` and run through ADR 0028.

Key inherited decisions:

- ADR 0015 records the historical v0.90.5 governed-tools umbrella.
- ADR 0018 records structured planning and review artifacts.
- ADR 0019 records Theory of Mind as bounded social cognition, not authority.

`v0.91.2` did not rewrite those records casually. It added new candidate ADRs
and, after review, promoted accepted records with narrow supersession notes.
That promotion happened during v0.91.3.

## Promoted ADR Set

| ADR | Title | Primary Boundary |
| --- | --- | --- |
| ADR 0020 | Universal Tool Schema As Portable Tool Description Standard | UTS is portable description, not authority; ADL adopts UTS while the standard moves toward a standalone repo. |
| ADR 0021 | ADL Capability Contract As Governed Runtime Authority Boundary | ACC is ADL-native runtime authority for capability exercise. |
| ADR 0022 | Speculative Decoding Deterministic Commit Boundary | Speculation may accelerate proposals, never silently commit side effects. |
| ADR 0023 | Google Workspace CMS Bridge And Canonical Repo Promotion Boundary | GWS is bounded collaboration infrastructure, not canonical repo truth. |
| ADR 0024 | Workflow Guardrails And Issue Lifecycle Control Plane | Conductor-first, worktree-bound, editor-only, reviewed, closeout-safe issue lifecycle is architecture policy. |
| ADR 0025 | CodeFriend Review Packet Product Boundary | CodeFriend is evidence-bound review/report workflow, not human-review replacement. |
| ADR 0026 | Repo Visibility Manifest And Linkage Layer | Repo visibility is manifest/linkage support, not full repo cognition. |
| ADR 0027 | Governed Code Modernization With Moderne/OpenRewrite LST | Modernization is deterministic, dry-run/review/approval bounded, not automatic mass rewrite. |
| ADR 0028 | C-SDLC Tracked Workflow State And Signed Trace Boundary | Durable C-SDLC truth becomes tracked/auditable and signed-trace-backed by the end of v0.91.4. |

Accepted files live in `docs/adr/`. The original candidate files remain in
`docs/architecture/adr/` as provenance.

## Supersession Plan

### ADR 0015

Do not delete or rewrite ADR 0015. It remains historical truth for the v0.90.5
governed-tools umbrella.

Now that ADR 0020 and ADR 0021 are accepted:

- mark ADR 0015 as superseded or partially superseded by ADR 0020 and ADR 0021
- point active UTS interpretation to ADR 0020
- point active ACC/governed-execution interpretation to ADR 0021
- preserve ADR 0015's original milestone context

### ADR 0018

Do not replace ADR 0018. It remains the artifact-contract decision for `SPP`
and `SRP` and the refined card lifecycle.

Now that ADR 0024 and ADR 0028 are accepted:

- point lifecycle-control-plane policy to ADR 0024
- point tracked workflow-state and signed trace migration policy to ADR 0028
- preserve ADR 0018 as the structured artifact baseline

## UTS Boundary

UTS should not remain ADL-only.

The ADL ADR should say:

- ADL adopts UTS
- ADL originated and benchmarks UTS
- ADL uses ACC as one companion governance model
- the normative UTS package should move toward the standalone
  `universal-tool-schema` repo

The ADL ADR should not say:

- UTS requires ADL runtime
- UTS validity grants execution authority
- ADL is the only possible governance companion for UTS

## C-SDLC Boundary

ADR 0028 is intentionally included even though the implementation spans
`v0.91.3` and `v0.91.4`.

The decision belongs in the v0.91.2 planning close because it sets the
architecture direction:

- C-SDLC is a general software-development model; ADL is implementing it first
  for its own workflow
- durable workflow state must become tracked
- C-SDLC records must be public, inspectable, auditable, and useful to ObsMem
- signed trace proof should land before C-SDLC becomes default operation

## Conditional ADRs Not Created Yet

No standalone ADR is created yet for:

- runtime/test-cycle recovery, unless it changes proof authority
- publication packets, unless they become durable claim-governance architecture
- review heuristics alone, because they are covered by ADR 0024 and ADR 0025
- rustdoc/doc cleanup, because it is documentation hygiene
- release evidence aggregation, unless it changes release authority

## Acceptance Criteria For This Issue

- All operator-approved v0.91.2 ADR candidates are represented.
- UTS and ACC are split.
- ADR 0015 and ADR 0018 have clear supersession relationships.
- C-SDLC tracked workflow state and signed traces are captured as ADR 0028.
- Candidate records were promoted only after human review and explicit
  promotion.
- The milestone decision surface points reviewers to this plan.
