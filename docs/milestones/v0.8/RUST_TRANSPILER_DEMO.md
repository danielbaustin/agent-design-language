# Rust Transpiler Demo (v0.8)

Status: Implemented bounded demo scaffold (not a production compiler)

Primary implementation issues:
- `#702` (fixture + Rust scaffold + runtime skeleton)
- `#703` (deterministic verification + evidence surface)

Integration/matrix alignment:
- `#704` (demo-matrix integration)
- `#759` (docs convergence pass)

## Purpose

This demo provides a bounded, reviewer-friendly proof surface showing how a minimal ADL workflow fixture maps to a deterministic Rust runtime skeleton.

It is intentionally small and deterministic:
- one fixture
- one Rust mapping scaffold
- one Rust runtime skeleton
- one verification artifact

## Canonical Demo Surfaces

1. Workflow fixture (input)
   - `demos/rust-transpiler/workflow/rust_transpiler_demo.yaml`
2. Rust-first transpiler scaffold (mapping check)
   - `demos/transpiler_demo/Cargo.toml`
   - `demos/transpiler_demo/src/main.rs`
3. Rust runtime/output skeleton
   - `demos/rust-transpiler/output/workflow_runtime.rs`
4. Verification evidence artifact
   - `demos/rust-transpiler/output/transpiler_verification.v0.8.json`
5. Demo docs
   - `demos/rust-transpiler/README.md`
   - `docs/milestones/v0.8/RUST_TRANSPILER_VERIFICATION_V0.8.md`

## Deterministic Execution Surface

Run the bounded verification scaffold:

`cargo run --manifest-path demos/transpiler_demo/Cargo.toml --quiet`

The scaffold verifies:
1. fixture path exists
2. runtime skeleton path exists
3. ordered workflow steps map one-to-one to ordered Rust runtime functions
4. mapping result is deterministic (`PASS`/`FAIL`)

## Implemented vs Illustrative Boundary

Implemented now:
- deterministic fixture-to-runtime mapping check
- stable evidence artifact path and schema/version fields
- bounded adaptive-execution reporting fields in evidence (`bounded_reporting_only`)

Illustrative / future work (not implemented in this demo):
- production transpiler/code-generation pipeline
- autonomous retry loop or policy learning
- generalized migration engine across arbitrary repositories

## Reviewer Quick Path

1. Inspect fixture:
   - `demos/rust-transpiler/workflow/rust_transpiler_demo.yaml`
2. Run scaffold command:
   - `cargo run --manifest-path demos/transpiler_demo/Cargo.toml --quiet`
3. Inspect runtime skeleton:
   - `demos/rust-transpiler/output/workflow_runtime.rs`
4. Inspect evidence artifact:
   - `demos/rust-transpiler/output/transpiler_verification.v0.8.json`
5. Verify acceptance boundary:
   - `docs/milestones/v0.8/RUST_TRANSPILER_VERIFICATION_V0.8.md`

## Scope Notes

This demo is a v0.8 documentation+verification surface for bounded deterministic mapping behavior. It should not be interpreted as a claim that ADL already ships a full Rust transpiler runtime.
