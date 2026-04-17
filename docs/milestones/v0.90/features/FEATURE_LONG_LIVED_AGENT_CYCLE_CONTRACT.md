# Feature: Long-Lived Agent Cycle Contract

## Status

v0.90 sprint-ready feature doc.

## Goal

Define the contract for one long-lived agent cycle so every heartbeat is
bounded, reviewable, replay-compatible, and safe.

## Core Cycle

One cycle is:

```text
observe -> prepare decision request -> run bounded ADL workflow -> persist result -> audit -> update status
```

The cycle is not:

- an unbounded chat loop
- a hidden background process
- a mid-step checkpoint
- a replacement for the v0.92 identity system

## Cycle ID

Cycle IDs are monotonic and zero-padded:

```text
cycle-000001
cycle-000002
cycle-000003
```

The next cycle ID is derived from the existing cycle ledger, not from wall-clock
time alone.

## Artifact Layout

Path:

```text
<state_root>/cycles/<cycle_id>/
```

Required files:

```text
cycle_manifest.json
observations.json
decision_request.json
decision_result.json
run_ref.json
memory_writes.jsonl
guardrail_report.json
cycle_summary.md
```

Optional files:

```text
provider_roster.json
trace_refs.json
scores.json
operator_notes.md
```

## cycle_manifest.json

```json
{
  "schema": "adl.long_lived_agent_cycle_manifest.v1",
  "agent_instance_id": "stock-league-value-monk",
  "cycle_id": "cycle-000001",
  "status": "success",
  "started_at": "2026-04-16T10:00:00Z",
  "completed_at": "2026-04-16T10:00:12Z",
  "workflow_kind": "demo_adapter",
  "workflow_ref": "long_lived_stock_league_cycle",
  "input_hash": "sha256:...",
  "output_hash": "sha256:...",
  "previous_cycle_id": null,
  "next_cycle_hint": "sleep_until_next_heartbeat",
  "not_financial_advice": true
}
```

Required truth:

- `status` must be one of `success`, `failed`, `blocked`, or `stopped`.
- `previous_cycle_id` must point to the previous completed cycle or be `null`.
- `input_hash` and `output_hash` must be deterministic over public-safe cycle
  payloads.

## observations.json

Observation payloads are pre-decision facts.

```json
{
  "schema": "adl.long_lived_agent_observations.v1",
  "agent_instance_id": "stock-league-value-monk",
  "cycle_id": "cycle-000001",
  "observed_at": "2026-04-16T10:00:00Z",
  "sources": [
    {
      "source_id": "market_fixture",
      "kind": "fixture",
      "trust_level": "canonical_demo_fixture",
      "artifact_ref": "market/snapshots/2026-01-05.json"
    }
  ],
  "facts": [
    {
      "key": "MSFT.close",
      "value": "374.00",
      "as_of": "2026-01-05"
    }
  ]
}
```

Rules:

- observations must be written before decision execution
- observations must not include future outcomes for the same cycle
- fixture observations must say they are fixtures
- live observations must include source and timestamp

## decision_request.json

The request is what the workflow is allowed to know.

```json
{
  "schema": "adl.long_lived_agent_decision_request.v1",
  "agent_instance_id": "stock-league-value-monk",
  "cycle_id": "cycle-000001",
  "agent_context_ref": "../../continuity.json",
  "observations_ref": "observations.json",
  "memory_refs": [],
  "allowed_actions": ["hold", "open_position", "increase_position", "trim_position", "explain"],
  "forbidden_actions": ["execute_order", "connect_broker", "personalized_advice"]
}
```

## decision_result.json

The result records what the agent decided in that cycle.

```json
{
  "schema": "adl.long_lived_agent_decision_result.v1",
  "agent_instance_id": "stock-league-value-monk",
  "cycle_id": "cycle-000001",
  "status": "accepted",
  "decision": {
    "action": "open_position",
    "ticker": "MSFT",
    "paper_allocation_pct": 20,
    "thesis": "Fixture thesis text.",
    "risk_thesis": "Fixture risk text.",
    "disconfirming_evidence": ["fixture close below threshold"]
  },
  "provider": {
    "source": "fixture_or_model",
    "model": "gemma4:latest"
  },
  "not_financial_advice": true
}
```

Rules:

- rejected decisions still write `decision_result.json`
- decision result must not be edited after audit except by appending a new
  correction artifact
- result must separate `decision` from `audit`

## run_ref.json

For regular ADL workflow cycles:

```json
{
  "schema": "adl.long_lived_agent_run_ref.v1",
  "run_id": "stock-league-value-monk-cycle-000001",
  "workflow_id": "stock-league-cycle",
  "run_status_ref": ".adl/runs/stock-league-value-monk-cycle-000001/run_status.json",
  "trace_ref": ".adl/runs/stock-league-value-monk-cycle-000001/logs/trace_v1.json"
}
```

For demo-adapter cycles:

```json
{
  "schema": "adl.long_lived_agent_run_ref.v1",
  "adapter": "long_lived_stock_league_cycle",
  "adapter_artifact_ref": "decision_result.json"
}
```

## memory_writes.jsonl

Each line is one append-only memory write candidate:

```json
{"schema":"adl.long_lived_agent_memory_write.v1","cycle_id":"cycle-000001","memory_id":"mem-000001","summary":"Opened a paper MSFT position with valuation thesis.","tags":["agent:stock-league-value-monk","cycle:cycle-000001","paper-market"],"source_refs":["decision_result.json"],"write_policy":"append_only"}
```

The supervisor may later submit these to ObsMem. In the first slice, writing
the JSONL file is sufficient.

## guardrail_report.json

```json
{
  "schema": "adl.long_lived_agent_guardrail_report.v1",
  "agent_instance_id": "stock-league-value-monk",
  "cycle_id": "cycle-000001",
  "status": "pass",
  "checks": [
    {
      "check_id": "no_real_trading",
      "result": "pass"
    }
  ],
  "rejected_actions": []
}
```

Guardrail failures must be explicit and machine-readable.

## cycle_summary.md

Human-readable but not authoritative. It should summarize:

- observations used
- decision made
- guardrail result
- memory writes
- next-cycle note

## Stock-League Cycle Binding

For the paper stock league:

- each agent maps to one long-lived agent spec
- each cycle consumes one market snapshot or fixture day
- each decision is paper-only
- the portfolio ledger is append-only
- the cycle contract replaces the one-shot fixture replay as the live theater
  path

## Acceptance Criteria

- Every cycle writes all required files.
- Cycle IDs are monotonic across restarts.
- Observations are created before decisions.
- Guardrails can reject forbidden actions.
- Memory writes are append-only.
- A reviewer can reconstruct all inputs to a decision from cycle artifacts.
- The stock-league demo can produce at least three consecutive cycles for one
  agent and preserve all prior cycle summaries.
