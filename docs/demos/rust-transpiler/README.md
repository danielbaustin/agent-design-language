# Rust Transpiler Demo Scaffold (v0.8)

## Purpose

This is a bounded demonstration artifact for v0.8 showing how a minimal ADL-style
workflow fixture maps to a deterministic Rust runtime skeleton.

It is not a production compiler or full transpiler.

The goal is confidence and clarity: one fixture, one mapping scaffold, one runtime
skeleton, and one explicit path to deterministic verification in `#703`.

## Demo Surfaces

1. Workflow fixture (input contract)
   - `examples/workflows/rust_transpiler_demo.yaml`
2. Rust transpiler scaffold (mapping demonstration entrypoint)
   - `tools/transpiler_demo/Cargo.toml`
   - `tools/transpiler_demo/src/main.rs`
3. Rust output skeleton (compiled/runtime shape)
   - `demos/rust_output/workflow_runtime.rs`
4. This explainer
   - `docs/demos/rust-transpiler/README.md`

## Fixture to Rust Mapping

The fixture defines a tiny deterministic pipeline with three steps:

- `step_prepare_input`
- `step_normalize_payload`
- `step_finalize_output`

The Rust skeleton mirrors this one-to-one:

- one function per workflow step
- `run_workflow()` executes steps in order
- data flow is explicit and deterministic:
  `source_token -> payload -> normalized_payload -> result_token`

## How to Inspect the Mapping

Run:

`cargo run --manifest-path tools/transpiler_demo/Cargo.toml --quiet`

The scaffold prints:

- fixture and Rust artifact paths
- ordered step-to-function mapping
- `PASS`/`FAIL` order consistency check
- artifact layout status

No code generation occurs in this demo.

## Verification Hand-off to #703

Issue `#703` verifies:

1. deterministic correspondence between fixture step order and Rust function order
2. replay-compatibility expectations for this bounded demo surface
3. stability of artifact paths and mapping for identical repository state
