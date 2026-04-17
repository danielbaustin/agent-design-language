# Feature: Long-Lived Supervisor And Heartbeat

## Status

v0.90 sprint-ready feature doc.

## Goal

Add a bounded supervisor that can run one agent repeatedly over time while
preserving ADL's existing execution model: each cycle is a finite, replayable
workflow execution with explicit artifacts.

## User Story

As an ADL operator, I want to start a long-lived agent with a clear schedule,
state root, and stop condition so that the agent can wake up, run one bounded
cycle, persist its result, and repeat without hiding work in an unobservable
background process.

## Why This Exists

The current runtime can run workflows and resume paused workflows, but it does
not own the outer loop that turns repeated bounded runs into a long-lived agent.

For demos such as the paper stock league, we need the agent to:

- wake up on a heartbeat
- look at new observations
- make a bounded decision
- persist a cycle record
- go back to sleep or stop

The supervisor is the missing runtime surface.

## Scope

The first implementation should support:

- manual single-cycle `tick`
- bounded `run --max-cycles <n>`
- interval-based `run --interval-secs <n> --max-cycles <n>`
- status reporting
- operator stop file
- per-agent lease file
- cycle artifact creation
- invocation of one configured ADL workflow or demo adapter per cycle

## Non-Goals

- No hidden always-on daemon in the first slice.
- No OS service installer.
- No distributed scheduler.
- No mid-step checkpointing.
- No autonomous execution without explicit `max-cycles`, `until`, or stop file.
- No full v0.92 identity tuple substrate.

## Command Contract

Preferred final CLI:

```bash
adl agent tick --spec <agent-spec.yaml>
adl agent run --spec <agent-spec.yaml> --max-cycles <n>
adl agent run --spec <agent-spec.yaml> --interval-secs <n> --max-cycles <n>
adl agent status --spec <agent-spec.yaml>
adl agent stop --spec <agent-spec.yaml> --reason <text>
```

Acceptable first-sprint tool path:

```bash
python3 adl/tools/long_lived_agent_supervisor.py tick --spec <agent-spec.yaml>
python3 adl/tools/long_lived_agent_supervisor.py run --spec <agent-spec.yaml> --max-cycles <n>
python3 adl/tools/long_lived_agent_supervisor.py status --spec <agent-spec.yaml>
python3 adl/tools/long_lived_agent_supervisor.py stop --spec <agent-spec.yaml> --reason <text>
```

The tool path is acceptable only if the artifact contract is stable and the
feature can later move behind `adl agent` without rewriting demo state.

## Agent Spec

Minimal YAML:

```yaml
schema: adl.long_lived_agent_spec.v1
agent_instance_id: stock-league-value-monk
display_name: Value Monk
state_root: .adl/long_lived_agents/stock-league-value-monk
workflow:
  kind: demo_adapter
  name: long_lived_stock_league_cycle
heartbeat:
  interval_secs: 300
  max_cycles: 3
  stale_lease_after_secs: 900
safety:
  require_operator_stop_file: false
  allow_network: false
  allow_broker: false
  financial_advice: false
memory:
  namespace: stock-league/value-monk
  write_policy: append_only
```

For a regular ADL workflow:

```yaml
workflow:
  kind: adl_workflow
  path: adl/examples/some-workflow.adl.yaml
  run_args:
    trace: true
    allow_unsigned: true
```

## Supervisor State Machine

States:

- `not_started`
- `idle`
- `leased`
- `running_cycle`
- `sleeping`
- `stopping`
- `stopped`
- `failed`
- `completed`

Allowed transitions:

```text
not_started -> leased -> running_cycle -> idle
idle -> leased
idle -> sleeping -> leased
idle -> stopped
running_cycle -> failed
running_cycle -> idle
running_cycle -> completed
failed -> leased only with explicit recovery flag
stopped -> no further cycles unless stop file is removed with explicit operator action
completed -> no further cycles unless a new run session is started
```

## Lease Semantics

Each agent has exactly one lease file:

```text
<state_root>/lease.json
```

Lease fields:

```json
{
  "schema": "adl.long_lived_agent_lease.v1",
  "agent_instance_id": "stock-league-value-monk",
  "lease_id": "lease-stock-league-value-monk-000003",
  "cycle_id": "cycle-000003",
  "owner_pid": 12345,
  "hostname": "local",
  "started_at": "2026-04-16T10:00:00Z",
  "expires_at": "2026-04-16T10:15:00Z",
  "status": "active"
}
```

Rules:

- A tick must create the lease before starting a cycle.
- A tick must refuse to run if an active lease exists.
- A stale lease must produce a clear recoverable error unless
  `--recover-stale-lease` is supplied.
- Lease cleanup must happen after cycle finalization.
- If the process crashes, the next status call must report the stale lease.

## Cycle Scheduling

`tick`:

- runs exactly one cycle
- exits after writing cycle and status artifacts

`run --max-cycles n`:

- repeatedly calls the same cycle path
- waits `interval_secs` between cycles unless `--no-sleep` is supplied for test
  mode
- exits with `completed` when `n` cycles are completed

`run --until <timestamp>`:

- optional later addition
- not required for the first slice

## Status Artifact

Path:

```text
<state_root>/status.json
```

Minimum fields:

```json
{
  "schema": "adl.long_lived_agent_status.v1",
  "agent_instance_id": "stock-league-value-monk",
  "state": "idle",
  "last_cycle_id": "cycle-000003",
  "last_cycle_status": "success",
  "completed_cycle_count": 3,
  "active_lease": null,
  "stop_requested": false,
  "last_error": null,
  "updated_at": "2026-04-16T10:15:00Z"
}
```

## Failure Policy

Failure classes:

- `spec_invalid`
- `lease_active`
- `lease_stale`
- `workflow_failed`
- `guardrail_failed`
- `artifact_validation_failed`
- `operator_stop_requested`

Policy:

- workflow failure writes a failed cycle and status `failed`
- guardrail failure writes a failed cycle unless the guardrail is configured as
  warning-only
- operator stop exits cleanly with status `stopped`
- invalid spec fails before acquiring a lease

## Acceptance Criteria

- A single `tick` creates a lease, runs one cycle, writes status, and removes
  the active lease.
- `run --max-cycles 3 --no-sleep` produces exactly three cycles.
- Two simultaneous ticks cannot both run.
- Status is readable while idle, running, failed, stopped, and completed.
- Stop file prevents the next tick from running.
- The supervisor can invoke the stock-league cycle adapter.
- The implementation has regression tests for active and stale lease handling.
