# Feature: Long-Lived Operator Control And Safety

## Status

v0.90 sprint-ready feature doc.

## Goal

Make long-lived agent execution safe to operate by giving the human clear
controls, visible status, bounded stop conditions, and reviewable safety
artifacts.

## Problem Statement

A long-lived agent without control surfaces is just a runaway script with better
branding.

The runtime must provide:

- explicit start/tick/run controls
- explicit stop controls
- visible status
- no overlapping cycles
- guardrails per cycle
- safe failure modes
- no secrets or private host paths in public demo artifacts

## Operator Roles

### Runtime Operator

Can:

- create agent specs
- start bounded runs
- request manual ticks
- stop an agent
- recover stale leases
- inspect status and cycle artifacts

Cannot:

- silently edit past cycle results
- bypass guardrails without a recorded event
- run without a max cycle count or explicit stop policy in demo mode

### Demo Viewer

Can:

- inspect artifacts
- read summaries
- verify guardrails

Cannot:

- influence running state
- provide private investment profile
- cause real-world side effects

## Stop Controls

Path:

```text
<state_root>/stop.json
```

Schema:

```json
{
  "schema": "adl.long_lived_agent_stop.v1",
  "agent_instance_id": "stock-league-value-monk",
  "requested_at": "2026-04-16T11:00:00Z",
  "requested_by": "operator",
  "reason": "demo complete",
  "mode": "stop_before_next_cycle"
}
```

Rules:

- Supervisor checks `stop.json` before acquiring a new lease.
- If a cycle is already running, stop applies before the next cycle.
- `stop --now` can be added later but is not required for first slice.
- Removing `stop.json` must be explicit and logged as an operator event.

## Safety Policy

Agent spec safety fields:

```yaml
safety:
  allow_network: false
  allow_broker: false
  allow_filesystem_writes_outside_state_root: false
  allow_real_world_side_effects: false
  require_public_artifact_sanitization: true
  financial_advice: false
  max_cycle_runtime_secs: 120
  max_consecutive_failures: 2
```

Default policy:

- network disabled unless explicitly allowed
- broker integrations forbidden
- real-world side effects forbidden
- artifact sanitization required for demo mirrors
- consecutive failures stop the run

## Guardrail Report

Each cycle writes:

```text
cycles/<cycle_id>/guardrail_report.json
```

Required checks:

- `spec_policy_loaded`
- `lease_valid`
- `stop_not_requested`
- `no_forbidden_action`
- `artifact_sanitization`
- `max_runtime_not_exceeded`

For stock demos, also:

- `no_real_trading`
- `no_broker_integration`
- `not_financial_advice`
- `paper_only_ledger`

## Status Command

`status` should print a compact human summary and support JSON:

```bash
adl agent status --spec stock-league-value-monk.yaml
adl agent status --spec stock-league-value-monk.yaml --json
```

Human output:

```text
agent: stock-league-value-monk
state: idle
completed cycles: 3
last cycle: cycle-000003 success
active lease: none
stop requested: no
last error: none
```

JSON output reads from `status.json`.

## Failure Behavior

### Workflow Failure

- write failed `cycle_manifest.json`
- write `guardrail_report.json` if possible
- update `status.json`
- increment consecutive failure count
- stop after `max_consecutive_failures`

### Guardrail Failure

- reject the cycle decision
- preserve rejected decision payload
- mark cycle `failed` or `blocked`
- do not apply side effects

### Stale Lease

- status reports `lease_stale`
- next tick refuses unless `--recover-stale-lease`
- recovery writes an operator event

### Invalid Spec

- fail before lease acquisition
- do not mutate cycle ledger

## Artifact Sanitization

Public demo mirrors must not contain:

- `/Users/...` host paths
- API keys
- bearer tokens
- private key material
- broker account identifiers
- private portfolio data

The sanitizer can be simple in the first slice:

- scan generated public artifact files
- fail if banned patterns are found
- write `artifact_sanitization` check result

## Stock-League Safety Requirements

For the stock league demo:

- all actions are paper-only
- `execute_order` is always rejected
- no broker URL or credential field is allowed
- no personalized advice input is accepted
- scoreboards are demo metrics, not investment recommendations
- live market data is optional and delayed/public only if enabled later

## Acceptance Criteria

- `stop` writes `stop.json` and prevents the next tick.
- `status` works after success, failure, stop, and stale lease.
- Guardrail failures are recorded, not swallowed.
- The stock-league illegal action remains rejected in multi-cycle mode.
- Public artifacts pass host-path and secret scans.
- Consecutive failure threshold stops the supervisor.
- Stale lease recovery requires explicit operator action.
- No cycle runs without a max-cycle or stop policy in demo mode.
