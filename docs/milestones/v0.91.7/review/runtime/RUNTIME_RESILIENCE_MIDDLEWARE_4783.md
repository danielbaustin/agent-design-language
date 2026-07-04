# Runtime Resilience Middleware Proof for #4783

Status: `implemented_with_integrated_runtime_trace_evidence`

Issue: `#4783`

## Scope

This packet records the v0.91.7 scheduler watcher and AEE resilience
middleware integration.

The implementation adds runtime resilience decision events to the real ADL
execution path. The events use the existing resilience vocabulary and are
retained through trace envelopes, action logs, normalized trace output,
obsmem indexing, and trace reports.

## Implemented Surfaces

- `adl/src/resilience.rs`
  - Adds `RuntimeResilienceTraceV1` and
    `RuntimeResilienceDispositionV1` under schema
    `adl.runtime.resilience_trace.v1`.

- `adl/src/execute/runner.rs`
  - Emits scheduler watcher admission and queued-backpressure decisions for the
    bounded concurrent executor.
  - Emits AEE middleware decisions for successful steps, degraded
    `continue_on_error` failures, terminal failures, timeouts, and cancellation.

- Trace and artifact surfaces:
  - `adl/src/trace/mod.rs`
  - `adl/src/trace/store.rs`
  - `adl/src/trace/report.rs`
  - `adl/src/cli/run_artifacts/runtime/trace_envelope.rs`
  - `adl/src/instrumentation.rs`
  - `adl/src/instrumentation/action_log.rs`
  - `adl/src/instrumentation/trace_formatting.rs`
  - `adl/src/instrumentation/trace_normalization.rs`
  - `adl/src/obsmem_indexing.rs`

## Proved Behavior

Focused tests exercise the real executor path and assert retained runtime
resilience decisions for:

- scheduler watcher admission under `max_concurrency`;
- queued backpressure when local concurrency is saturated;
- successful step completion;
- degraded continue after a step marked `continue_on_error`;
- terminal runtime failure;
- timeout classification;
- cancellation classification.
- called-workflow inner provider steps, so nested runtime work is not hidden
  behind a top-level call-step-only resilience record.

## Validation

Local validation run from the #4783 worktree after merging current `origin/main`:

```text
cargo fmt --manifest-path adl/Cargo.toml --all -- --check
cargo test --manifest-path adl/Cargo.toml runtime_resilience -- --nocapture
cargo test --manifest-path adl/Cargo.toml runner_concurrent -- --nocapture
cargo test --manifest-path adl/Cargo.toml runner_executes_called_workflow_success_path -- --nocapture
bash adl/tools/run_owner_validation_lane.sh runtime
git diff --check
```

Result:

- formatting: `PASS`
- runtime resilience focused tests: `PASS` (`2 passed`)
- concurrent runner integrated tests: `PASS` (`2 passed`)
- called-workflow inner-step resilience regression test: `PASS` (`1 passed`)
- runtime owner validation lane: `PASS`
- diff whitespace check: `PASS`

## Non-Claims

- This does not claim full Runtime Soak #2 completion; `#4682` owns the broader
  soak run.
- This does not claim the `#4784` failure-injection proof has consumed this PR
  until `#4783` is merged and `#4784` reruns against it.
- This does not claim durable hibernation, replay migration, or production
  scheduler service readiness beyond the retained runtime trace evidence above.
- This does not replace the landed `#4718` logging/OTel proof; it emits runtime
  resilience trace records that later Soak #2 matrix work can consume.
