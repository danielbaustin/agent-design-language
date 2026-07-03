# Cognitive Scheduler v1 Proof for #4671

## Scope

This packet records the v0.91.7 WP-05 proof for `#4671`, which owns the
cognitive scheduler v1 implementation and integrated CLI proof surface.

The scheduler implementation lives in `adl/src/scheduler.rs` and is exposed
through the operator-facing CLI path:

```bash
adl scheduler plan --input adl/tests/fixtures/scheduler/economics_inputs_v1.json --out docs/milestones/v0.91.7/review/scheduler/artifacts/cognitive_scheduler_v1_plan_4671.json
```

## Implemented Surface

The implemented scheduler path consumes the fixture-backed scheduler economics
bundle and emits a first-class `adl.scheduler.plan.v1` artifact with:

- selected lane per task
- alternatives considered and rejection or fallback reasons
- score breakdown
- dependency status
- manual override status
- confidence
- deterministic scheduling rank key
- recommended execution order

## v0.91.7 Fix

This issue found and fixed a scheduler ordering defect: work selected for the
`DELAYED` lane could rank ahead of schedulable work when it was delayed for
capacity rather than a hard blocker.

`adl/src/scheduler.rs` now includes an explicit `deferred` rank-key component.
The rank key keeps hard-blocked work last while also ensuring non-blocked
delayed work does not outrank schedulable local, cheap-remote, premium, or
governor work.

## Retained Machine-Readable Evidence

Primary artifact:

- `docs/milestones/v0.91.7/review/scheduler/artifacts/cognitive_scheduler_v1_plan_4671.json`

The retained artifact records this recommended order:

1. `release-authority`
2. `premium-code-repair`
3. `first-pass-review`
4. `docs-status-check`
5. `partial-dependency-review`
6. `low-urgency-cleanup`
7. `blocked-proof`

That proves:

- `GOVERNOR`, `PREMIUM`, `CHEAP_REMOTE`, `LOCAL`, and `DELAYED` lanes are all
  exercised.
- schedulable work is ordered before non-blocked delayed work.
- hard-blocked work remains last.
- the CLI can generate the retained plan artifact from the tracked fixture.

## Validation

Local validation:

- `python3 adl/tools/warm_rust_dependency_cache.py --source-target ../../adl/target --dest-target adl/target --manifest-path adl/Cargo.toml --dry-run --json`
  - Verified the warm-cache plan was safe before linking dependency artifacts.
- `python3 adl/tools/warm_rust_dependency_cache.py --source-target ../../adl/target --dest-target adl/target --manifest-path adl/Cargo.toml --json`
  - Linked 5,863 warm dependency artifacts into the issue worktree.
- `cargo fmt --manifest-path adl/Cargo.toml --all -- --check`
  - Verified Rust formatting after the scheduler change.
- `cargo test --manifest-path adl/Cargo.toml scheduler::tests --lib -- --nocapture`
  - Passed 17 scheduler tests, including lane selection, deterministic ordering,
    malformed input rejection, YAML parsing, blocked dependency handling, and
    Chronosense scheduler context behavior.
- `cargo test --manifest-path adl/Cargo.toml --bin adl scheduler_plan -- --nocapture`
  - Passed 2 CLI scheduler tests for valid artifact writing and malformed bundle
    rejection.
- `cargo build --manifest-path adl/Cargo.toml --bin adl`
  - Built the worktree CLI binary used to regenerate the retained artifact.
- `ADL_OBSERVABILITY_LOG=$TMPDIR/adl-4671-scheduler-plan-fixed.log adl/target/debug/adl scheduler plan --input adl/tests/fixtures/scheduler/economics_inputs_v1.json --out docs/milestones/v0.91.7/review/scheduler/artifacts/cognitive_scheduler_v1_plan_4671.json`
  - Regenerated the retained plan artifact from the fixed worktree binary.
- `python3 - <<'PY' ...`
  - Parsed the retained artifact and asserted schema version, first task,
    last task, and delayed-work ordering.

## Boundaries

This issue does not implement sibling WP-05 work:

- provider profile selection remains `#4672`
- model suitability selection proof remains `#4673`
- cheapest validated outcome policy remains `#4674`
- local-agent delegation readiness remains `#4675`

The scheduler is deterministic and integrated through the CLI/proof path. It
does not mutate GitHub, select a live provider, launch a local agent, or conduct
a sprint by itself.
