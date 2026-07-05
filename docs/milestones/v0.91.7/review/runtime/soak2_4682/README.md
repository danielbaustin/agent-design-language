# Runtime Soak 2 Final Rerun Status (#4880)

This packet records the final WP-07 Runtime Soak 2 rerun performed by `#4880`.
It supersedes the earlier `#4682` blocked-before-full-soak attempt while
preserving the legacy artifact root path for review continuity.

The current result is `final_rerun_completed_with_blockers`: the merged
`#4681` runtime path, `#4783` resilience middleware, `#4784` failure-injection
proof, `#4843` matrix, `#4718` observability proof, and `#4885` supervised
daemon mode were consumable from current `main`, and the local runtime/proof
harnesses emitted retained evidence. The `#4634` umbrella repair adds a local
durable OTel-shaped JSONL sink and monitor-status proof for the daemon path via
`ADL_OTEL_LOG` and `ADL_OTEL_STATUS`. Five activation-facing rows remain
blocked or require operator
disposition before `#4634` can close as full v0.92 runtime-coherence proof:

- Runtime AWS/signal bridge live heartbeat/SNS/SSM proof was not run.
- Unity editor live consumption was not run, although runtime-owned
  Observatory/Unity contract artifacts were produced.
- WP-12 ACIP/A2A activation remains outside the WP-07 rerun, although local
  ACIP positive/denied/malformed/failed-delivery cases were retained.
- Runtime-v2 capability envelope and security/CAV packets were emitted as
  static contract artifacts with `generated_at_utc: not_started`; they are not
  fresh integrated proof.

## Primary Artifacts

- `soak2_execution_status_4682.json`
- `blocker_register.json`
- `evidence_index.json`

## Proving Evidence

- `tokio_runtime_substrate/quickstart/runtime_marker.txt`
- `agent_lifecycle/integrated_runtime_soak_proof.json`
- `aee_memory/runtime_acip_aee_memory_proof.json`
- `resilience/runtime_failure_injection_proof.json`
- `daemon_supervision/state/daemon_status.json`
- `daemon_supervision/state/operator_events.jsonl`
- `daemon_supervision/state/continuity_checkpoint.json`
- `otel_monitor/otel.jsonl`
- `otel_monitor/otel_status.json`
- `otel_monitor/state/daemon_status.json`
- `security_cav_boundary/proof_packet.json` (static contract artifact only)
- `capability_envelope/operator_control_report.json` (static contract artifact only)
- `../../observability_4718/generated/proof_summary.json`

## Validation

```bash
bash adl/tools/validate_v0917_soak2_matrix.sh
bash adl/tools/validate_v0917_soak2_4682_status.sh
bash adl/tools/test_pr_v0917_integrated_observability_proof.sh
bash adl/tools/test_provider_demo_common.sh
adl/target/debug/adl agent daemon --spec docs/milestones/v0.91.7/review/runtime/soak2_4682/daemon_supervision/agent.yaml --max-restarts 1 --checkpoint-interval-secs 3 --no-sleep --json
ADL_OBSERVABILITY_LOG=docs/milestones/v0.91.7/review/runtime/soak2_4682/otel_monitor/observability.log ADL_OTEL_LOG=docs/milestones/v0.91.7/review/runtime/soak2_4682/otel_monitor/otel.jsonl ADL_OTEL_STATUS=docs/milestones/v0.91.7/review/runtime/soak2_4682/otel_monitor/otel_status.json adl/target/debug/adl agent daemon --spec docs/milestones/v0.91.7/review/runtime/soak2_4682/otel_monitor/agent.yaml --max-restarts 1 --checkpoint-interval-secs 3 --no-sleep --json
git diff --check
```

## Non-Claims

- This packet claims the local `ADL_OTEL_LOG` JSONL sink and
  `ADL_OTEL_STATUS` monitor file for ADL-emitted events; it does not claim a
  network OTLP collector, hosted telemetry backend, or external exporter
  service.
- This packet does not claim live AWS signal bridge readiness.
- This packet does not claim Unity editor live-consumption readiness.
- This packet does not claim WP-12 ACIP/A2A activation closure.
- This packet does not claim fresh runtime-v2 capability-envelope or
  security/CAV proof from packets whose generated timestamp is `not_started`.
- This packet does not claim daemon OS boot persistence, kill -9 resistance,
  host resource exhaustion resistance, or missing-binary resistance.
- Two Cargo security probes were accidentally started and interrupted; they
  are explicitly not validation proof.
