# Docs Spine (v0.4)

Start here for contributor-oriented docs and planning context.

## Reading Order

1. `README.md`
- Project overview and demo entrypoint.

2. `swarm/tools/README.md`
- Operational workflow tools (`pr.sh`, burst helpers, report helpers).

3. `docs/milestones/v0.4/RELEASE_NOTES_v0.4.md`
- Official v0.4 capability and release summary.

4. `docs/milestones/v0.4/DESIGN_v0.4.md`
- Canonical architecture and execution semantics for v0.4.

5. `swarm/tools/BURST_PLAYBOOK.md`
- Sequential burst execution pattern and operating guardrails.

6. `adl-spec/README.md`
- Language-level specification entrypoint.

7. `.adl/reports/INDEX.md`
- Living index of generated reports and latest pointers.

## Contributor Entry Points

- Workflow default: `adl_pr_cycle` (`start -> codex -> finish -> report`)
- Runtime and CLI work: `swarm/`
- Language and schema docs: `adl-spec/`
- v0.4 milestone docs: `docs/milestones/v0.4/`
- Burst reporting outputs: `.adl/reports/`
- Demo command index: `docs/demos.md`

## Historical (v0.3) Concurrency Design

- `docs/concurrency/v0.3-core.md`: Core fork/join primitives, deterministic trace ordering, and v0.3 scope.
- `docs/concurrency/v0.3-failure-cancellation-replay.md`: Failure semantics, cancellation propagation, and replay invariants.
- `docs/concurrency/v0.3-state-materialization-api.md`: Deterministic state/materialization model and minimal runtime API contract.
- `docs/concurrency/v0.3-test-plan.md`: Unit/integration test matrix with concrete file paths.
