# ADL v0.8 Release Notes

## Metadata
- Product: Agent Design Language (ADL)
- Milestone: v0.8
- Status: Unreleased (release-candidate finalization)
- Tag: `v0.8.0` (planned)

## Summary
v0.8 delivers a bounded Gödel runtime loop, user-visible Gödel CLI commands, canonical runtime artifact emission for reviewable execution, and refreshed demo/runbook surfaces under `demos/`.

This milestone remains intentionally bounded: it does not claim autonomous policy learning, unconstrained self-modification, or production graduation of the Rust transpiler scaffold.

## Delivered In v0.8 (Repository Truth)

### Bounded Gödel Runtime Loop
- Runtime implements a bounded seven-stage scientific loop used in v0.8 demos:
  `failure -> hypothesis -> mutation -> experiment -> evaluation -> record -> indexing`
- Design and implementation references:
  - `docs/adr/0008-godel-stage-loop-v08.md`
  - `swarm/src/godel/stage_loop.rs`

### New CLI Surfaces
- Added bounded Gödel CLI commands:
  - `adl godel run`
  - `adl godel inspect`
  - `adl godel evaluate`
- CLI implementation surfaces live under:
  - `swarm/src/cli/godel_cmd.rs`
  - `swarm/src/cli/commands.rs`

### Canonical Runtime Artifacts
- Runtime paths now emit and validate core v0.8 review artifacts:
  - `mutation.v1`
  - `canonical_evidence_view.v1`
  - `evaluation_plan.v1`
  - `experiment_record.v1`
- `tool_result.v1` sidecar emission is also integrated for bounded export paths.

### Demos And Reviewer Runbooks
- Canonical user-facing demo index is under `demos/README.md`.
- v0.8 reviewer runbooks include:
  - `demos/godel_failure_hypothesis_experiment.md`
  - `demos/aee-recovery/README.md`
  - `demos/rust-transpiler/README.md`
  - `docs/milestones/v0.8/DEMOS_V0.8.md`

### Bounded Rust Transpiler Scaffold
- v0.8 includes a deterministic transpiler demo scaffold and verification flow:
  - `examples/transpiler_demo/`
  - `demos/rust-transpiler/workflow/rust_transpiler_demo.yaml`
  - `demos/rust-transpiler/output/`
- This remains a bounded demo surface, not a production transpiler pipeline.

## Not Claimed As Delivered In v0.8
- Fully finished Adaptive Execution Engine behavior.
- Autonomous policy learning or open-ended self-modification.
- Production-ready transpiler execution and deployment workflow.

## Release-Readiness Notes
- See `docs/milestones/v0.8/MILESTONE_CHECKLIST_V0.8.md` for current release-gate status.
- See `docs/milestones/v0.8/QUALITY_GATE_V0.8.md` and `docs/milestones/v0.8/RECOVERY_AUDIT_V0.8.md` for validation and repository-truth alignment.
- See `CHANGELOG.md` for cross-milestone summary context.

## References
- `docs/milestones/v0.8/README.md`
- `docs/milestones/v0.8/RELEASE_PLAN_V0.8.md`
- `docs/milestones/v0.8/DEMOS_V0.8.md`
- `docs/milestones/v0.8/GODEL_LOOP_INTEGRATION_V0.8.md`
- `docs/milestones/v0.8/RECOVERY_AUDIT_V0.8.md`
