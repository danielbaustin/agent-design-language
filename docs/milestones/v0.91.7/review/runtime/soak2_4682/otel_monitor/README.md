# Soak 2 OTel Monitor Proof (#4634)

This packet proves the WP-07 daemon path can mirror ADL observability events
into a durable local OTel-shaped JSONL sink and a monitor-status file.

Command:

```bash
ADL_OBSERVABILITY_LOG=docs/milestones/v0.91.7/review/runtime/soak2_4682/otel_monitor/observability.log ADL_OTEL_LOG=docs/milestones/v0.91.7/review/runtime/soak2_4682/otel_monitor/otel.jsonl ADL_OTEL_STATUS=docs/milestones/v0.91.7/review/runtime/soak2_4682/otel_monitor/otel_status.json adl/target/debug/adl agent daemon --spec docs/milestones/v0.91.7/review/runtime/soak2_4682/otel_monitor/agent.yaml --max-restarts 1 --checkpoint-interval-secs 3 --no-sleep --json
```

Primary evidence:

- `otel.jsonl`: OTel-shaped local JSONL event sink.
- `otel_status.json`: monitor status with event count and last trace/span.
- `observability.log`: compatibility `adl_event` log.
- `state/daemon_status.json`: daemon terminal status.
- `state/operator_events.jsonl`: durable daemon operator events.

Claim boundary:

- Proven: local `ADL_OTEL_LOG` JSONL export and `ADL_OTEL_STATUS` monitor
  status for ADL-emitted daemon lifecycle events.
- Not claimed: network OTLP collector, hosted telemetry backend, or OS service
  manager integration.
