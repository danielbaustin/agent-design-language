# V0.91.7 Integrated Resilience Failure Injection (#4784)

This packet is a bounded local proof for #4784. It exercises existing ADL long-lived-agent, remote-exec, delegation-policy, and resilience primitives under injected failures. It does not claim the #4783 scheduler/watcher/AEE resilience middleware path is available before that issue lands, and it does not claim complete product resilience or v0.92 readiness.

## What This Proves

This packet proves the currently available integrated ADL paths for #4784: long-lived-agent run/resume/stop control-plane behavior, remote-exec timeout behavior, retry, timeout, cancellation, circuit-terminal guard, rate/backpressure, bulkhead, degraded fallback, and negative auth/quota/policy classification.

## Reviewer Path

1. Inspect `integrated_resilience_failure_injection_proof.json`.
2. Inspect `failure_injection_matrix.json` and `blocker_register.json`.
3. Inspect `control_plane/long_lived_agent/resume_status_cycle3.json` and `control_plane/live_stop/stop_probe.json`.
4. Inspect `resilience/*.json`, `runtime_provider/remote_timeout_probe.json`, and `negative_cases/auth_quota_policy_terminal.json`.
5. Inspect `audit/artifact_safety_scan.json`.
