# Reviewer Walkthrough

Run the soak with `cargo run --manifest-path adl/Cargo.toml --bin run_v0916_integrated_runtime_soak -- --out docs/milestones/v0.91.6/review/runtime/v0916_integrated_runtime_soak_4543`.

The review question is whether the runtime now leaves one honest, durable packet showing restart, a live stop between cycles, timeout, saturation/backpressure, degraded fallback, scheduler advisory evidence, remote-exec timeout semantics, and memory handoff under one bounded local proof surface without overclaiming ACIP, Unity/Observatory, or v0.92 readiness.
