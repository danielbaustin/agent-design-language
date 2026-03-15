# Changelog

All notable project-level changes are summarized here by milestone/release.

## v0.8 (Active Development Milestone)

Status: In progress.

Summary:
- Bounded Godel runtime and demo surfaces now exist on `main`, including the explicit seven-stage loop:
  `failure -> hypothesis -> mutation -> experiment -> evaluation -> record -> indexing`
- Canonical runtime artifacts for the Godel review loop are now emitted and validated, including:
  `mutation.v1`, `canonical_evidence_view.v1`, `evaluation_plan.v1`, and `experiment_record.v1`
- New user-facing CLI and demo surfaces were added for bounded Godel execution and inspection, alongside the v0.8 demo matrix
- New reviewer-facing demo runbooks under `demos/` cover the bounded Gödel CLI flow and bounded AEE recovery flow
- The Rust transpiler remains a bounded demo scaffold for deterministic fixture-to-runtime verification, not a production transpiler
- Major review-tail work landed to align milestone docs, schemas, and release-facing repository truth with current implementation

References:
- `docs/milestones/v0.8/RELEASE_PLAN_V0.8.md`
- `docs/milestones/v0.8/RELEASE_NOTES_V0.8.md`
- `docs/milestones/v0.8/MILESTONE_CHECKLIST_V0.8.md`
- `docs/milestones/v0.8/RECOVERY_AUDIT_V0.8.md`
- `docs/milestones/v0.8/DEMOS_V0.8.md`
- `docs/milestones/v0.8/GODEL_LOOP_INTEGRATION_V0.8.md`

Not yet claimed in v0.8:
- fully finished Adaptive Execution Engine behavior
- unconstrained self-modification or autonomous policy learning
- production graduation of the Rust transpiler demo

## v0.75 (Previous Milestone)

Status: prior milestone reference.

References:
- `docs/milestones/v0.75/RELEASE_PLAN_0.75.md`
- `docs/milestones/v0.75/RELEASE_NOTES_0.75.md`
- `docs/milestones/v0.75/MILESTONE_CHECKLIST_0.75.md`

## v0.7.0 (Released)

Status: Released (`v0.7.0`).

Summary:
- Foundation runtime hardening for deterministic, replayable execution.
- Security envelope and trust/signing surfaces integrated into core execution flows.
- Runtime identity migration to canonical `adl` naming with compatibility-window shims.

References:
- `docs/milestones/v0.7/RELEASE_NOTES_v0.7.md`
- `docs/milestones/v0.7/RELEASE_PLAN_v0.7.md`
