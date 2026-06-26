# V0.91.6 Runtime Failure Injection Proof (#4547)

This review packet records the bounded integrated runtime resilience proof for issue `#4547`.

## Scope

- Prove one retained failure-injection register for retry, timeout, cancellation, partial failure, degraded fallback, and remote timeout.
- Prove one bounded resume-continuation path and one explicit non-continuity-after-stop path under the same integrated runtime packet.
- Produce one reviewer-readable artifact root that Soak #1 can consume without widening the claim surface.

## Non-Claims

- This does not prove interrupted restart recovery, full checkpoint/restore, migration, replay, or durable continuity.
- This does not prove Unity/Observatory integration.
- This does not prove full `v0.92` runtime readiness.

## Retained Packet

- Reviewer root: `docs/milestones/v0.91.6/review/runtime/v0916_runtime_failure_injection_4547`
- Machine-readable proof: `docs/milestones/v0.91.6/review/runtime/v0916_runtime_failure_injection_4547/runtime_failure_injection_proof.json`
- Failure register: `docs/milestones/v0.91.6/review/runtime/v0916_runtime_failure_injection_4547/runtime_failure_register.json`
- Resilience traces:
  - `docs/milestones/v0.91.6/review/runtime/v0916_runtime_failure_injection_4547/resilience/retry_execution.json`
  - `docs/milestones/v0.91.6/review/runtime/v0916_runtime_failure_injection_4547/resilience/timeout_execution.json`
  - `docs/milestones/v0.91.6/review/runtime/v0916_runtime_failure_injection_4547/resilience/cancellation_execution.json`
  - `docs/milestones/v0.91.6/review/runtime/v0916_runtime_failure_injection_4547/resilience/bulkhead_execution.json`
  - `docs/milestones/v0.91.6/review/runtime/v0916_runtime_failure_injection_4547/resilience/degraded_fallback_execution.json`
- Continuity evidence:
  - `docs/milestones/v0.91.6/review/runtime/v0916_runtime_failure_injection_4547/long_lived_agent/resume_status_cycle3.json`
  - `docs/milestones/v0.91.6/review/runtime/v0916_runtime_failure_injection_4547/long_lived_agent_stop_probe/stop_probe.json`
- Remote timeout evidence:
  - `docs/milestones/v0.91.6/review/runtime/v0916_runtime_failure_injection_4547/remote_exec/timeout_probe.json`

## Acceptance Mapping

- Every required injected failure mode now has expected behavior, observed behavior, and an evidence ref in `runtime_failure_register.json`.
- Retry/backoff is explicit rather than implied.
  - Observed attempt count: `3`
  - Final retry status: `succeeded`
- Timeout and cancellation are both negative-case proofs, not disguised success cases.
  - Timeout final status: `timed_out`
  - Cancellation final status: `cancelled`
- Partial failure and degraded behavior are classified truthfully.
  - Bulkhead final status: `saturated`
  - Degraded fallback final status: `degraded_success`
  - Degraded output flag: `true`
- Continuity and non-continuity are both bounded and explicit.
  - Resume-continuation path after cycle 3: `completed`
  - Stop-after-first-persisted-cycle final state: `stopped`
  - Second persisted cycle manifest present after stop: `false`
  - Interrupted restart recovery remains unclaimed and is explicitly excluded by the packet non-claims.
- Artifact safety scan passed.

## Validation

- `cargo run --manifest-path adl/Cargo.toml --bin run_v0916_runtime_failure_injection -- --out docs/milestones/v0.91.6/review/runtime/v0916_runtime_failure_injection_4547`
  - Generated the retained runtime failure-injection packet and reviewer-facing artifacts.
- `cargo test --manifest-path adl/Cargo.toml --bin run_v0916_runtime_failure_injection run_v0916_runtime_failure_injection_generates_expected_artifacts`
  - Verified the packet generator emits the expected retained proof artifacts.
- `cargo test --manifest-path adl/Cargo.toml --bin run_v0916_runtime_failure_injection runtime_failure_injection_helpers_remain_reviewable`
  - Verified the helper probes remain reviewable and that the negative-case traces preserve the expected classifications.

## Notes

- This packet is intentionally narrower than the broader integrated runtime soak packet: it focuses on resilience/failure-injection truth for `#4547` so the Soak #1 umbrella can consume it without inheriting unrelated claim surfaces.
