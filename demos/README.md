# ADL Demos

`demos/` is the canonical user-facing entrypoint for finding and running ADL demos from repository root.

## Start Here

If you want the fastest successful demo run:

```bash
cargo run -q --manifest-path swarm/Cargo.toml --bin adl -- swarm/examples/v0-3-fork-join-seq-run.adl.yaml --print-plan
```

If you want the active v0.8 flagship demo surface:

```bash
cargo run --manifest-path tools/transpiler_demo/Cargo.toml --quiet
```

## Demo Categories

- Runtime workflow demos live in `swarm/examples/`.
  These are the actual ADL files you can pass to the CLI today.
- Milestone runbooks and reviewer docs live here in `demos/`.
- Spec-only example artifacts live in `adl-spec/examples/` and are not the main starting point for runnable demos.

## Recommended Paths

### Quickstart runtime demos

- `swarm/examples/v0-3-fork-join-seq-run.adl.yaml`
- `swarm/examples/v0-3-on-error-retry.adl.yaml`
- `swarm/examples/v0-5-primitives-minimal.adl.yaml`

### Active v0.8 demos

- `rust-transpiler/README.md`
- `v0.8-bounded-critical-demos.md`
- `godel_failure_hypothesis_experiment.md`
- `adaptive_godel_loop_demo.md`
- `experiment_prioritization_demo.md`
- `cross_workflow_learning_demo.md`
- `promotion_eval_loop_demo.md`
- `aee-recovery/README.md`

Use `aee-recovery/README.md` for the bounded Adaptive Execution Engine recovery
demo: failure, explicit retry-budget recommendation, bounded overlay, and
successful recovery with replayable artifacts.

Use `swarm/tools/demo_aee_bounded_adaptation.sh` for the one-command v0.85
bounded adaptation demo that emits `learning/aee_decision.json` as the primary
adaptive decision artifact.

Use `godel_failure_hypothesis_experiment.md` for the concrete
`adl godel run` / `adl godel inspect` / `adl godel evaluate` review path and the
persisted Gödel schema/runtime artifacts it produces, including the
first-class `godel_hypothesis.v1.json` hypothesis record. For a one-command
review flow, run `swarm/tools/demo_godel_hypothesis_engine.sh`.

Use `promotion_eval_loop_demo.md` for the bounded WP-14 review path that proves
the prior Gödel artifacts now feed a structured evaluation report and an
explicit promotion decision through the persisted
`godel_eval_report.v1.json` and `godel_promotion_decision.v1.json` artifacts.
For a one-command review flow, run `swarm/tools/demo_promotion_eval_loop.sh`.

Use `adaptive_godel_loop_demo.md` for the bounded WP-11 policy-learning slice.
It shows a deterministic policy decision and before/after policy comparison
derived from the persisted hypothesis artifact. For a one-command review flow,
run `swarm/tools/demo_adaptive_godel_loop.sh`.

Use `experiment_prioritization_demo.md` for the bounded WP-12 prioritization
slice. It shows a deterministic ranked experiment output with explicit
confidence values and stable tie-break behavior derived from the hypothesis and
policy artifacts. For a one-command review flow, run
`swarm/tools/demo_experiment_prioritization.sh`.

Use `cross_workflow_learning_demo.md` for the bounded WP-13 review path that
proves workflow-A prioritization output changes a downstream workflow-B
decision through the persisted `godel_cross_workflow_learning.v1.json`
artifact. For a one-command review flow, run
`swarm/tools/demo_cross_workflow_learning.sh`.

Use `affect_godel_vertical_slice_demo.md` for the bounded WP-17 slice that
proves an affect transition changes the top Gödel-adjacent strategy ranking
through the persisted `godel_affect_vertical_slice.v1.json` artifact. For a
one-command review flow, run
`swarm/tools/demo_affect_godel_vertical_slice.sh`.

### Historical/runtime fixture inventory

- `../swarm/examples/README.md`

Use that README when you specifically want the full crate-local example inventory. For user-facing demo discovery, start here instead.
