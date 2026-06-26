# V0.91.6 Runtime Failure Injection

This runtime failure-injection packet is a bounded local proof for #4547. It proves reviewer-readable resilience behavior under one integrated runtime path and does not claim full v0.92 runtime readiness, checkpoint/restore, migration, replay, or Unity/Observatory completion.

## What This Proves

This run proves a bounded integrated runtime slice for `#4547`: one long-lived-agent run/resume/stop continuity packet plus retry, timeout, explicit cancellation, partial failure via bulkhead saturation, degraded fallback, and remote timeout classification under one reviewer-readable artifact root.

## Reviewer Path

1. Inspect `runtime_failure_injection_proof.json`.
2. Inspect `runtime_failure_register.json`.
3. Inspect `long_lived_agent/resume_status_cycle3.json` and `long_lived_agent_stop_probe/stop_probe.json`.
4. Inspect `resilience/retry_execution.json`, `resilience/timeout_execution.json`, `resilience/cancellation_execution.json`, `resilience/bulkhead_execution.json`, and `resilience/degraded_fallback_execution.json`.
5. Inspect `remote_exec/timeout_probe.json`.
6. Inspect `audit/artifact_safety_scan.json`.
