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
v0.8 focuses on controlled experimentation and authoring on top of the released v0.7 runtime foundation. Current repository state includes substantial v0.8 docs/spec work and bounded transpiler demo scaffolding, while key runtime integrations remain in progress.

## What Is Implemented Today (Repository Truth)
- Canonical v0.8 planning/docs surfaces under `docs/milestones/v0.8/`.
- Gödel schema/spec artifacts (ExperimentRecord, Evidence View, Mutation, EvaluationPlan, workflow template) as design-stage contracts.
- Bounded Rust transpiler demo scaffold artifacts:
  - `demos/rust-transpiler/workflow/rust_transpiler_demo.yaml`
  - `tools/transpiler_demo/`
  - `demos/rust-transpiler/`

## What Is Still In Progress
- Full runtime integration of v0.8 schema surfaces into execution paths.
- End-to-end milestone closure across review-tail issues and gates.
- Final release-candidate validation and ceremony steps.

## Known Limitations
- Several v0.8 artifacts are currently specification/planning surfaces rather than fully runtime-implemented features.
- Milestone completion requires additional convergence across docs, review findings, and implementation surfaces.

## Validation Notes
- Use `docs/milestones/v0.8/QUALITY_GATE_V0.8.md` for gate expectations.
- Use `docs/milestones/v0.8/RECOVERY_AUDIT_V0.8.md` for current repository-state reconciliation.

## References
- `docs/milestones/v0.8/README.md`
- `docs/milestones/v0.8/RELEASE_PLAN_V0.8.md`
- `docs/milestones/v0.8/RECOVERY_AUDIT_V0.8.md`
