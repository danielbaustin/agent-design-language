# Heartbeat, Timeout, And Progress Proof (#3708)

Issue: #3708  
Parent sprint: #3703  
Captured: 2026-06-15  
Status: implementation_refreshed

## Summary

This packet records the refreshed `#3708` slice after the logging mini-sprint
was re-audited against current repo truth.

The repo already had:

- control-plane `adl_event` baselines (`#3609`, `#3706`);
- provider JSONL run logs; and
- long-lived-agent status, lease, and cycle-ledger artifacts.

The remaining operator pain was narrower:

1. long-running validation subprocesses could still look silent while waiting;
2. provider adapter calls could time out without an operator-facing heartbeat
   stream or bounded next-action hint; and
3. long-lived agent commands could be alive for a while without a clear
   operator-facing liveness signal.

## Implemented Slice

This issue adds one shared operator-facing heartbeat helper and uses it in three
high-pain paths:

- `pr finish` validation subprocesses
- `adl-provider-adapter` provider-call execution
- `adl agent run` / `tick` / `status`

The helper is intentionally bounded:

- default heartbeat interval is `5000 ms`
- tests may override it with `ADL_OBSERVABILITY_HEARTBEAT_MS`
- fast commands finish before the interval and therefore do not emit heartbeat
  spam

## Proof Commands

### Shared heartbeat helper behavior

```bash
cargo test --manifest-path adl/Cargo.toml cli::observability -- --nocapture
```

Expected proof:

- slow operations emit `started`, `heartbeat`, and `completed`
- fast operations emit `started` and `completed` without heartbeat spam
- heartbeat interval override works deterministically in tests

### Validation subprocess heartbeat and classification

```bash
cargo test --manifest-path adl/Cargo.toml finish_validation_emits_subprocess_heartbeat_and_classification -- --nocapture
```

Expected proof:

- `pr finish` validation subprocess waits emit
  `stage=validation_subprocess`
- the emitted log captures the program and bounded subprocess class
- a delayed subprocess emits at least one heartbeat before completion

### Provider timeout heartbeat and next-action hint

```bash
cargo test --manifest-path adl/Cargo.toml cli_run_emits_heartbeat_and_timeout_diagnostics_for_slow_provider_calls -- --nocapture
```

Expected proof:

- the provider adapter emits heartbeat events while a request is still waiting
- timeout exits emit `result=timeout`
- timeout exits emit `reason_code=provider_timeout`
- timeout exits emit
  `next_action_hint=check_provider_or_increase_timeout_ms`

### Long-lived process heartbeat

```bash
cargo test --manifest-path adl/Cargo.toml agent_run_emits_heartbeat_for_long_lived_processes -- --nocapture
```

Expected proof:

- `adl agent run` emits operator-facing heartbeat events while the process is
  still active
- the completed event records a long-lived process classification rather than
  pretending the command is an ordinary short validation step

## Non-Claims

- This issue does not claim complete heartbeat coverage for every ADL command.
- It does not add OpenTelemetry export or collector integration.
- It does not merge runtime action logs, provider JSONL logs, and
  long-lived-agent ledgers into one canonical record.
- It does not claim that long-lived-agent internal cycle details belong in the
  same terminal/event stream as the operator-facing liveness heartbeat.
