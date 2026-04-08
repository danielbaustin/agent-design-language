# ADL Demos

`demos/` is the canonical user-facing entrypoint for finding and running ADL demos from repository root.

## Start Here

If you want the fastest successful demo run:

```bash
cargo run -q --manifest-path adl/Cargo.toml --bin adl -- adl/examples/v0-3-fork-join-seq-run.adl.yaml --print-plan
```

If you want the `v0.86` bounded cognitive review surface:

```bash
./adl/tools/demo_v086_review_surface.sh
```

If you want the historical `v0.8` flagship demo surface:

```bash
cargo run --manifest-path demos/transpiler_demo/Cargo.toml --quiet
```

If you want the current `v0.87` substrate demo program:

```bash
bash adl/tools/demo_v087_suite.sh
```

## Demo Categories

- Runtime workflow demos live in `adl/examples/`.
  These are the actual ADL files you can pass to the CLI today.
- Milestone runbooks and reviewer docs live here in `demos/`.
- Spec-only example artifacts live in `adl-spec/examples/` and are not the main starting point for runnable demos.

## Recommended Paths

### v0.87 demo program

- `v0.87/v087_demo_program.md`
- `../docs/milestones/v0.87/DEMO_MATRIX_v0.87.md`

Use `v0.87/v087_demo_program.md` for the canonical `v0.87` demo order and bounded
repo-local commands.

Use `bash adl/tools/demo_v087_suite.sh` for the one-command substrate review
path. It populates:
- `artifacts/v087/trace_v1/`
- `artifacts/v087/provider_portability/`
- `artifacts/v087/shared_obsmem/`
- `artifacts/v087/skills/`
- `artifacts/v087/control_plane/`
- `artifacts/v087/reviewer_package/`

### v0.86 demo program

- `../docs/milestones/v0.86/DEMO_MATRIX_v0.86.md`
- `../docs/milestones/v0.86/features/LOCAL_AGENT_DEMOS.md`

Use `../docs/milestones/v0.86/DEMO_MATRIX_v0.86.md` for the canonical milestone review order and proof-surface contract.

Use `./adl/tools/demo_v086_review_surface.sh` for the one-command reviewer entry point. It generates:
- `artifacts/v086/review_surface/demo_manifest.json`
- `artifacts/v086/review_surface/README.txt`
- `artifacts/v086/review_surface/index.txt`

Use `./adl/tools/demo_v086_control_path.sh` for the primary bounded cognitive-path proof surface. It generates:
- `artifacts/v086/control_path/demo-g-v086-control-path/summary.txt`
- `artifacts/v086/control_path/demo-g-v086-control-path/arbitration.json`
- `artifacts/v086/control_path/demo-g-v086-control-path/freedom_gate.json`
- `artifacts/v086/control_path/demo-g-v086-control-path/trace.jsonl`

### v0.85 demo program

- `v0.85/v085_demo_program.md`
- `v0.85/steering_queue_checkpoint_demo.md`
- `v0.85/hitl_editor_review_demo.md`
- `v0.85/godel_hypothesis_engine_demo.md`
- `aee-recovery/README.md`
- `v0.85/affect_godel_vertical_slice_demo.md`

Use `v0.85/v085_demo_program.md` for the canonical milestone review order.

Use `adl/tools/demo_steering_queue_checkpoint.sh` for the bounded
steering/checkpoint/resume proof surface. It emits:
- `.adl/runs/v0-85-hitl-steering-demo/pause_state.json`
- `.adl/runs/v0-85-hitl-steering-demo/run.json`
- `.adl/reports/demo-steering-queue-checkpoint/steer.json`

Use `adl/tools/demo_hitl_editor_review.sh` for the bounded editor/review
workflow proof surface. It emits:
- `.adl/reports/demo-hitl-editor-review/editor_review_demo_manifest.v1.json`

Use `v0.85/godel_hypothesis_engine_demo.md` for the milestone-facing WP-10 review
path. For a one-command review flow, run
`adl/tools/demo_godel_hypothesis_engine.sh`.

### Quickstart runtime demos

- `adl/examples/v0-3-fork-join-seq-run.adl.yaml`
- `adl/examples/v0-3-on-error-retry.adl.yaml`
- `adl/examples/v0-5-primitives-minimal.adl.yaml`

### Historical v0.8 demos

- `rust-transpiler/README.md`
- `v0.8/v0.8-bounded-critical-demos.md`
- `v0.8/godel_failure_hypothesis_experiment.md`
- `v0.85/adaptive_godel_loop_demo.md`
- `v0.85/experiment_prioritization_demo.md`
- `v0.85/cross_workflow_learning_demo.md`
- `v0.85/promotion_eval_loop_demo.md`
- `aee-recovery/README.md`

Use `aee-recovery/README.md` for the bounded Adaptive Execution Engine recovery
demo: failure, explicit retry-budget recommendation, bounded overlay, and
successful recovery with replayable artifacts.

Use `adl/tools/demo_aee_bounded_adaptation.sh` for the one-command v0.85
bounded adaptation demo that emits `learning/aee_decision.json` as the primary
adaptive decision artifact.

Use `v0.8/godel_failure_hypothesis_experiment.md` for the concrete
`adl godel run` / `adl godel inspect` / `adl godel evaluate` review path and the
persisted Gödel schema/runtime artifacts it produces, including the
first-class `godel_hypothesis.v1.json` hypothesis record. For a one-command
review flow, run `adl/tools/demo_godel_hypothesis_engine.sh`.

Use `v0.85/promotion_eval_loop_demo.md` for the bounded WP-14 review path that proves
the prior Gödel artifacts now feed a structured evaluation report and an
explicit promotion decision through the persisted
`godel_eval_report.v1.json` and `godel_promotion_decision.v1.json` artifacts.
For a one-command review flow, run `adl/tools/demo_promotion_eval_loop.sh`.

Use `v0.85/adaptive_godel_loop_demo.md` for the bounded WP-11 policy-learning slice.
It shows a deterministic policy decision and before/after policy comparison
derived from the persisted hypothesis artifact. For a one-command review flow,
run `adl/tools/demo_adaptive_godel_loop.sh`.

Use `v0.85/experiment_prioritization_demo.md` for the bounded WP-12 prioritization
slice. It shows a deterministic ranked experiment output with explicit
confidence values and stable tie-break behavior derived from the hypothesis and
policy artifacts. For a one-command review flow, run
`adl/tools/demo_experiment_prioritization.sh`.

Use `v0.85/cross_workflow_learning_demo.md` for the bounded WP-13 review path that
proves workflow-A prioritization output changes a downstream workflow-B
decision through the persisted `godel_cross_workflow_learning.v1.json`
artifact. For a one-command review flow, run
`adl/tools/demo_cross_workflow_learning.sh`.

Use `v0.85/affect_godel_vertical_slice_demo.md` for the bounded WP-17 slice that
proves an affect transition changes the top Gödel-adjacent strategy ranking
through the persisted `godel_affect_vertical_slice.v1.json` artifact. For a
one-command review flow, run
`adl/tools/demo_affect_godel_vertical_slice.sh`.

### Historical/runtime fixture inventory

- `../adl/examples/README.md`

Use that README when you specifically want the full crate-local example inventory. For user-facing demo discovery, start here instead.
