# V0.91.6 Integrated Runtime Soak

This integrated runtime soak is a bounded local proof surface. It does not claim autonomous v0.92 readiness, external-agent transport closure, or full Observatory/Unity product completion.

## What This Proves

This run proves a bounded integrated runtime slice for `#4543`: one long-lived-agent run/restart/inspection continuity path, one companion live stop-between-cycles probe, timeout classification, bulkhead/backpressure saturation, degraded fallback, one deterministic scheduler decision artifact, remote-exec timeout handling, one tracked ObsMem handoff path, and one explicit injected-lease contract probe all converge under one reviewer-readable artifact root.

## Reviewer Path

1. Inspect `integrated_runtime_soak_proof.json`.
2. Inspect `completion_classification.json` for integrated-proven versus blocked surfaces.
3. Inspect `long_lived_agent/state/status.json` and `long_lived_agent/state/cycle_ledger.jsonl`.
4. Inspect `inspection/latest.json`.
5. Inspect `long_lived_agent_stop_probe/stop_probe.json`.
6. Inspect `resilience/timeout_execution.json`, `resilience/bulkhead_execution.json`, and `resilience/degraded_fallback_execution.json`.
7. Inspect `scheduler/scheduler_plan.json`.
8. Inspect `remote_exec/timeout_probe.json`.
9. Inspect `obsmem/transition_memory_request.json`.
10. Inspect `audit/artifact_safety_scan.json`.
