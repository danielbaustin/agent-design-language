# ADL v0.8 Release Notes (Draft)

## Metadata
- Product: Agent Design Language (ADL)
- Milestone: v0.8
- Release status: Draft / unreleased
- Tag: TBD

## Scope Truth (Important)
These are draft release notes for an in-progress milestone.
They describe intended v0.8 outcomes and currently implemented surfaces, but do not claim that v0.8 is released.

## Summary
v0.8 focuses on controlled experimentation and authoring on top of the released v0.7 runtime foundation. Current repository state includes bounded Gödel runtime and CLI surfaces, canonical runtime artifact emission for the core review loop, refreshed user-facing demo runbooks under `demos/`, and the bounded transpiler demo scaffold.

## What Is Implemented Today (Repository Truth)
- Canonical v0.8 planning/docs surfaces under `docs/milestones/v0.8/`.
- Bounded Gödel runtime and CLI surfaces, including:
  - seven-stage loop execution in the v0.8 demos
  - `adl godel run`
  - `adl godel inspect`
  - `adl godel evaluate`
- Canonical Gödel review artifacts emitted and validated in bounded runtime paths:
  - `canonical_evidence_view.v1`
  - `mutation.v1`
  - `evaluation_plan.v1`
  - `experiment_record.v1`
  - `tool_result.v1` sidecar emission for the current bounded export path
- User-facing v0.8 demo runbooks under `demos/`, including:
  - `demos/godel_failure_hypothesis_experiment.md`
  - `demos/aee-recovery/README.md`
- Bounded Rust transpiler demo scaffold artifacts:
  - `demos/rust-transpiler/workflow/rust_transpiler_demo.yaml`
  - `tools/transpiler_demo/`
  - `demos/rust-transpiler/`

## What Is Still In Progress
- Final external-review handoff packaging (`THIRD_PARTY_REVIEW_V0.8.md` and release-tail closeout).
- Remaining review-packet simplification so outside reviewers have a single compact entry surface.
- Final release-candidate validation and ceremony steps.

## Known Limitations
- Several v0.8 docs remain specification/planning surfaces rather than executable runtime features.
- The Adaptive Execution Engine remains bounded and demonstrative rather than fully finished.
- The Rust transpiler remains a bounded deterministic demo scaffold, not a production transpiler.
- Milestone completion requires additional convergence across docs, review findings, and implementation surfaces.

## Validation Notes
- Use `docs/milestones/v0.8/QUALITY_GATE_V0.8.md` for gate expectations.
- Use `docs/milestones/v0.8/RECOVERY_AUDIT_V0.8.md` for current repository-state reconciliation.

## References
- `docs/milestones/v0.8/README.md`
- `docs/milestones/v0.8/RELEASE_PLAN_V0.8.md`
- `docs/milestones/v0.8/RECOVERY_AUDIT_V0.8.md`
