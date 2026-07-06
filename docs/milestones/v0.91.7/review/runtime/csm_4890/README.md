# CSM Runtime Owner Binary Proof (#4890)

This packet records the runtime daemon path after separating daemon ownership
from the ADL compiler/control-plane CLI.

Proving command:

```bash
ADL_OBSERVABILITY_STDERR=0 \
ADL_OBSERVABILITY_LOG=docs/milestones/v0.91.7/review/runtime/csm_4890/observability.log \
ADL_OTEL_LOG=docs/milestones/v0.91.7/review/runtime/csm_4890/otel.jsonl \
ADL_OTEL_STATUS=docs/milestones/v0.91.7/review/runtime/csm_4890/otel_status.json \
ADL_OLLAMA_BIN=adl/tools/mock_ollama_v0_4.sh \
adl/target/debug/csm daemon \
  --spec docs/milestones/v0.91.7/review/runtime/csm_4890/agent.yaml \
  --max-restarts 1 \
  --checkpoint-interval-secs 3 \
  --no-sleep \
  --json
```

Proof claims:

- `csm daemon` is the runtime owner surface for daemon execution.
- `csm daemon` executes the configured concurrent `adl_workflow` DAG through
  the canonical ADL resolver/executor path; `adl` remains the tooling/control-
  plane binary.
- `adl agent daemon` is not retained as a public command.
- Daemon lifecycle events are emitted as `command=csm` with
  `process_class=csm_runtime_daemon`.
- CSM daemon status records integrated ChronoSense, AEE, scheduler watcher,
  resilience middleware, observability, and local OTel capabilities.
- `state/cycles/cycle-000001/csm_adl_run_status.json` retains step records,
  scheduler policy (`max_concurrency=2`, `source=run_default`), runtime-control
  state, and AEE/runtime-resilience trace summaries for the DAG run.
- Local OTel-shaped events use `service.name=csm-runtime-daemon` for CSM daemon
  lifecycle events.
- `ADL_OTEL_STATUS` records the retained event count and terminal CSM daemon
  trace/span identity.
- Partial checkpoint and recoverable runtime state artifacts remain present.

Non-claims:

- No hosted OTLP collector or network telemetry backend is claimed.
- This packet does not claim OS boot persistence, kill -9 resistance, host
  resource exhaustion resistance, or missing-binary resistance.
