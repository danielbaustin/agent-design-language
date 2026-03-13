# v0.8 Documentation Pass and Review Convergence

This document records the v0.8 docs-convergence pass performed before third-party review.

It is a documentation handoff artifact for `#707` and does not add runtime scope.

## Convergence Outcome

Status: **Partially converged; version story reconciled, final review handoff still pending**

Primary result:

- milestone navigation and core planning surfaces are aligned,
- README/manifests/reviewer-facing status language now describes one unreleased v0.8 main-branch story layered on top of the latest tagged v0.7.0 release,
- remaining review-tail blockers are explicitly listed rather than treated as non-blocking.

## Surfaces Reviewed

| Surface | Convergence status | Notes |
|---|---|---|
| README/navigation index | partially converged | navigation aligned; reviewer handoff still blocked by remaining review-tail work |
| execution/dependency docs | converged | execution order and boundaries aligned |
| Gödel loop docs/templates | converged | stage ordering and schema references aligned |
| authoring/reviewer planning docs | converged | sequencing and template-location docs present |
| bounded AEE + quality gate docs | converged | bounded scope and gate phases explicit in planning docs |
| demo/review/release-prep docs | partially converged | surfaces are present, but final third-party-review packet is still incomplete |

## Per-Document Consistency Audit

All markdown docs under `docs/milestones/v0.8/` and `docs/milestones/v0.8/incubation/` were reviewed for:

- internal link/path validity,
- milestone-slicing consistency (`v0.75`, `v0.8`, `v0.85+`),
- canonical-source consistency (no stale pre-canonical planning-path references),
- path hygiene (no absolute host paths).

Reviewed files:

- `docs/milestones/v0.8/ADAPTIVE_EXECUTION_ENGINE.md`
- `docs/milestones/v0.8/ARCHITECTURE_V0.8.md`
- `docs/milestones/v0.8/CANONICAL_EVIDENCE_VIEW_V1.md`
- `docs/milestones/v0.8/DECISIONS_V0.8.md`
- `docs/milestones/v0.8/DESIGN_V0.8.md`
- `docs/milestones/v0.8/DOCS_CONVERGENCE_V0.8.md`
- `docs/milestones/v0.8/EPIC_AUTHORING_SURFACES_v1.md`
- `docs/milestones/v0.8/EPIC_MAPPING_v0.8.md`
- `docs/milestones/v0.8/EVALUATION_PLAN_V1.md`
- `docs/milestones/v0.8/EXECUTION_ORDER_V0.8.md`
- `docs/milestones/v0.8/EXPERIMENT_RECORD_V1.md`
- `docs/milestones/v0.8/GODEL_AGENT_NOTES.md`
- `docs/milestones/v0.8/GODEL_EXPERIMENT_WORKFLOW_TEMPLATE_V1.md`
- `docs/milestones/v0.8/GODEL_LOOP_DIAGRAM.md`
- `docs/milestones/v0.8/GODEL_LOOP_INTEGRATION_V0.8.md`
- `docs/milestones/v0.8/GODEL_SCIENTIFIC_METHOD.md`
- `docs/milestones/v0.8/LAYER_8_PROVIDER.md`
- `docs/milestones/v0.8/MEMORY_MODEL_FOR_AI.md`
- `docs/milestones/v0.8/MILESTONE_CHECKLIST_V0.8.md`
- `docs/milestones/v0.8/MUTATION_FORMAT_V1.md`
- `docs/milestones/v0.8/OBSMEM_INDEXING_SURFACES_V1.md`
- `docs/milestones/v0.8/README.md`
- `docs/milestones/v0.8/RELEASE_NOTES_V0.8.md`
- `docs/milestones/v0.8/RELEASE_PLAN_V0.8.md`
- `docs/milestones/v0.8/RUST_TRANSPILER_DEMO.md`
- `docs/milestones/v0.8/SPRINT_V0.8.md`
- `docs/milestones/v0.8/STICKTOITTIVENESS.md`
- `docs/milestones/v0.8/TOOL_RESULT_CONTRACT_V1.md`
- `docs/milestones/v0.8/VISION_0.80.md`
- `docs/milestones/v0.8/WBS_V0.8.md`
- `docs/milestones/v0.8/incubation/GODEL_AGENT.md`
- `docs/milestones/v0.8/incubation/OBSMEM_BAYES.md`
- `docs/milestones/v0.8/incubation/STICKTOITTIVENESS.md`

## Resolved in Convergence Pass

1. Ensured canonical v0.8 navigation references core docs consistently.
2. Verified cross-doc references used for review/release planning are present.
3. Confirmed no absolute host-path leakage and no stale pre-canonical planning-path references in audited docs.
4. Corrected handoff references that pointed to non-existent docs to use existing canonical milestone surfaces.
5. Reconciled version-bearing reviewer-facing language so `README.md`, `swarm/README.md`, and v0.8 review docs all describe `main` as unreleased v0.8 work on top of the latest tagged v0.7.0 release.

## Deferred / Follow-up Candidates

The following remain real review-tail blockers or follow-up items rather than resolved convergence facts:

1. Final third-party review packet artifact creation/restoration.
2. Third-party findings triage resolution after `#707`.
3. Release ceremony finalization after review-tail completion.

## Handoff to Third-Party Review (`#707`)

Reviewers should treat this docs baseline as canonical once the remaining review-tail blockers above are cleared. At that point, use:

- `README.md` (navigation)
- `EXECUTION_ORDER_V0.8.md` (ordering/dependencies)
- `RUST_TRANSPILER_DEMO.md` (flagship demo contract)
- `WBS_V0.8.md` + `SPRINT_V0.8.md` (WP coverage and sequencing)
- `MILESTONE_CHECKLIST_V0.8.md` + `RELEASE_PLAN_V0.8.md` (quality/release gating criteria)

for milestone-level validation.

## Out of Scope

- implementing review findings,
- adding new v0.8 features,
- redefining already accepted schema contracts.
