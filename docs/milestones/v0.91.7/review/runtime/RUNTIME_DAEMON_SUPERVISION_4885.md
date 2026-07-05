# Runtime Daemon Supervision Proof Note (#4885)

Status: `implemented_local_proof`

Issue `#4885` adds a foreground supervised daemon mode for the integrated long-lived runtime path:

- Command: `adl agent daemon --spec <agent-spec.yaml>`.
- Durable daemon state: `state/daemon_status.json`.
- Recoverable agent state: existing `status.json`, `continuity_checkpoint.json`, `continuity_replay_manifest.json`, `cycle_ledger.jsonl`, and stop/lease records.
- Restart policy: ordinary child tick failures are classified, checkpointed, and restarted until the bounded restart budget is exhausted.
- Partial checkpoint cadence: `--checkpoint-interval-secs <n>` defaults to `3` and must be greater than zero.
- Stop semantics: daemon observes the existing `stop.json` control plane and exits through a recoverable stopped state.
- Observability: each daemon lifecycle event writes `operator_events.jsonl` and emits `adl_event` records with OTel-compatible `trace_id`, `span_id`, `parent_span_id`, and `otel_service_name` fields.

Covered daemon lifecycle events:

- `daemon_started`
- `child_spawn`
- `child_exit`
- `checkpoint_write`
- `restart_scheduled`
- `restart_attempted`
- `restart_budget_exhausted`
- `graceful_shutdown_requested`
- `stop_completed`
- `daemon_completed`

Truth boundary:

- This is not OS service-manager persistence.
- This does not claim survival across host reboot, operator `kill -9`, missing binaries, or host resource exhaustion.
- Those unsupported permanence claims are retained in `daemon_status.json` as explicit non-claims.

Focused validation:

- `cargo check --manifest-path adl/Cargo.toml --lib --bin adl`
- `cargo test --manifest-path adl/Cargo.toml daemon_partial_checkpoint --lib`
- `cargo test --manifest-path adl/Cargo.toml daemon_ --lib`
- `cargo test --manifest-path adl/Cargo.toml --test cli_smoke agent_daemon`
- `cargo test --manifest-path adl/Cargo.toml cli::agent_cmd::tests::agent_`
- `cargo test --manifest-path adl/Cargo.toml cli::agent_cmd::tests::agent_argument_validation_reports_missing_values_and_unknown_args`
- `cargo fmt --manifest-path adl/Cargo.toml --all --check`
- `ADL_PR_FAST_ALLOW_FULL_NEXTEST=1 bash adl/tools/run_pr_fast_test_lane.sh --changed-files .adl/generated-vpp-changed-files.txt`
- `bash adl/tools/test_run_pr_fast_test_lane.sh`
- `bash adl/tools/test_validation_manager.sh`

The smoke tests prove:

- daemon happy path writes `daemon_status.json`, continuity checkpoint/replay artifacts, operator events, and OTel-compatible observability fields
- daemon restart-budget failure leaves `status.json` in `failed` with `daemon_child_failed`
- restart scheduling, restart attempt, budget exhaustion, and checkpoint references are retained as operator events
- restart-backoff partial checkpoints preserve the child failure reason
- stop observed during restart backoff suppresses false `restart_attempted` evidence
- daemon cadence fails closed for explicit zero interval and defaults to 3 seconds when the spec omits heartbeat interval
- heartbeat partial checkpoints do not report restart backoff state

Escalated publication proof:

- The PR-fast validation manager classified the changed Rust surface as requiring explicit full nextest proof.
- Full nextest result: 19,383 tests passed, 18 skipped, 1 slow, and 1 leaky test reported.

Validation-selector repair:

- `adl/tools/run_pr_fast_test_lane.sh` now maps the daemon wave to focused `agent_cmd`, `cli_smoke_basics`, `long_lived_agent`, and `agent_cli_smoke` tokens.
- `adl/tools/test_run_pr_fast_test_lane.sh` covers the focused daemon-wave plan.
- `adl/tools/test_validation_manager.sh` covers manager-level publication sufficiency for this daemon wave plus the retained proof note.
