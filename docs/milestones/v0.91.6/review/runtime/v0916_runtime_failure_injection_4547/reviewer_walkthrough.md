# Reviewer Walkthrough

Run the proof with `cargo run --manifest-path adl/Cargo.toml --bin run_v0916_runtime_failure_injection -- --out docs/milestones/v0.91.6/review/runtime/v0916_runtime_failure_injection_4547`.

The review question is whether the runtime now leaves one honest, durable packet showing retry, timeout, explicit cancellation, partial failure classification, degraded fallback, remote timeout semantics, one bounded resume-continuation proof, and one explicit non-continuity-after-stop proof without overclaiming interrupted restart recovery, checkpoint/restore, migration, replay, or v0.92 readiness.
