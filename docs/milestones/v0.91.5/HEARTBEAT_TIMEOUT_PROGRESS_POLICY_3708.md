# Heartbeat, Timeout, And Progress Policy (#3708)

Issue: #3708  
Parent sprint: #3703  
Captured: 2026-06-15  
Status: implemented_first_slice

## Summary

This policy defines the first bounded `#3708` answer to the operator question:
"is it hung, waiting, or still making progress?"

The policy does not try to make every ADL surface noisy. It separates:

- short CLI commands that should normally emit only started/completed or
  started/failed truth;
- bounded long-running subprocesses that should emit heartbeat/progress while
  waiting; and
- long-lived processes that should stay visibly alive without pretending every
  internal cycle detail belongs in one terminal stream.

## Core Rules

1. Fast commands stay quiet.
   - If a command finishes before the heartbeat interval, it should not emit
     repeated heartbeat noise.
   - Started/completed or started/failed surfaces remain acceptable.

2. Heartbeats are interval-based, not loop-spammed.
   - The default operator-facing heartbeat interval is `5000 ms`.
   - Tests may override the interval with `ADL_OBSERVABILITY_HEARTBEAT_MS`.
   - The first heartbeat should appear only after the interval elapses.

3. Long waits must expose classification.
   - When ADL waits on a subprocess or provider call, the log must state what
     class of work is in flight.
   - Examples in this slice:
     - `subprocess_class=shell_validation`
     - `subprocess_class=rust_validation`
     - `provider=<provider>`
     - `runtime_surface=<surface>`

4. Timeout outcomes must be stable and actionable.
   - Timeouts should emit `result=timeout` and a bounded `reason_code`.
   - Timeout records should also carry one bounded `next_action_hint`.
   - In this slice, hosted/local provider waits use:
     - `reason_code=provider_timeout`
     - `next_action_hint=check_provider_or_increase_timeout_ms`

5. Long-lived processes have different expectations from short CLI commands.
   - Operator-facing long-lived commands such as `adl agent run` should emit
     process-liveness heartbeat events while the run stays active.
   - Canonical per-cycle detail remains in the long-lived-agent status, lease,
     and cycle-ledger artifacts rather than being duplicated into a chatty
     terminal stream.

## Covered Surfaces In This Slice

### Finish Validation Subprocesses

`pr finish` local validation subprocesses now emit bounded observability with:

- `command=finish`
- `stage=validation_subprocess`
- `program=<program>`
- `subprocess_class=<classification>`
- `argv_excerpt=<bounded command excerpt>`
- `elapsed_ms=<duration>`

This is intended to answer:

- what validation subprocess is running;
- whether it is still alive; and
- whether it completed or failed.

### Provider Adapter Calls

`adl-provider-adapter` now emits bounded provider-call heartbeat/timeout
observability with:

- `command=provider_adapter`
- `stage=provider_call`
- `provider=<provider>`
- `runtime_surface=<surface>`
- `provider_model_id=<model>`
- `request_id=<request>`
- `timeout_ms=<policy>`
- `elapsed_ms=<duration>`

Timeouts emit:

- `result=timeout`
- `reason_code=provider_timeout`
- `next_action_hint=check_provider_or_increase_timeout_ms`

### Long-Lived Agent Runs

`adl agent run`, `adl agent tick`, and `adl agent status` now emit
operator-facing observability with:

- `command=agent`
- `stage=agent_run` / `agent_tick` / `agent_status`
- `process_class=long_lived`
- `spec=<bounded spec path>`
- `elapsed_ms=<duration>`

For `agent run`, the surface additionally records:

- `max_cycles=<requested cycles>`
- `interval_secs=<requested or spec default interval>`
- `no_sleep=<true|false>`

This is intended to answer whether the long-lived process is still alive
without replacing the canonical cycle/status artifacts.

## Bounded Non-Claims

- This policy does not claim that every ADL command now has heartbeat coverage.
- It does not implement OpenTelemetry export; that remains `#3709`.
- It does not redefine runtime action logs, provider JSONL logs, or
  long-lived-agent ledgers as one merged truth source.
- It does not promise per-cycle or per-attempt progress detail for every
  internal step of a long-lived run.

## Follow-On Consumers

- `#3709` should preserve these event/result semantics when defining the OTEL
  boundary.
- `#3710` should consume these fields as the Observatory-facing liveness and
  timeout baseline rather than inventing a second incompatible heartbeat model.
