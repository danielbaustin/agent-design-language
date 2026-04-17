# Feature: Long-Lived State And Continuity

## Status

v0.90 sprint-ready feature doc.

## Goal

Provide a durable state and continuity layer for long-lived agents before the
full `v0.92` identity/capability substrate lands.

## Boundary

This feature must not claim to implement `v0.92` identity.

It implements a smaller continuity handle that is good enough for sprint demos:

- stable agent instance naming
- durable cycle ledger
- append-only memory references
- model/provider binding history
- migration placeholder for future identity substrate

## Why This Exists

Long-lived agents need continuity before full identity exists.

Without a continuity layer, repeated runs are just repeated executions. They can
produce artifacts, but there is no stable place to answer:

- Which agent is this?
- What cycles has it completed?
- What did it remember?
- What model/provider was bound in each cycle?
- What state should the next cycle inherit?
- What future identity record should this migrate into?

## State Root

Default:

```text
.adl/long_lived_agents/<agent_instance_id>/
```

Demo mirror:

```text
artifacts/v090/<demo_name>/long_lived_agents/<agent_instance_id>/
```

Required files:

```text
agent_spec.locked.json
continuity.json
cycle_ledger.jsonl
status.json
cycles/
```

Optional files:

```text
provider_binding_history.jsonl
memory_index.json
operator_events.jsonl
stop.json
lease.json
```

## agent_spec.locked.json

The supervisor should normalize the operator YAML spec into a locked JSON file.

Purpose:

- preserve the exact initial runtime contract
- make restarts deterministic
- prevent accidental changes from silently changing the agent

Rule:

- If the operator changes the spec, the supervisor must either refuse or write a
  `spec_revision` event before continuing.

## continuity.json

```json
{
  "schema": "adl.long_lived_agent_continuity.v1",
  "agent_instance_id": "stock-league-value-monk",
  "display_name": "Value Monk",
  "created_at": "2026-04-16T10:00:00Z",
  "created_by": "operator",
  "continuity_kind": "pre_v0_92_handle",
  "status": "active",
  "state_root": ".adl/long_lived_agents/stock-league-value-monk",
  "memory_namespace": "stock-league/value-monk",
  "cycle_ledger_ref": "cycle_ledger.jsonl",
  "latest_cycle_id": null,
  "future_identity_ref": null,
  "non_claims": [
    "not_v0_92_identity_tuple",
    "not_capability_governance",
    "not_autonomous_legal_personhood"
  ]
}
```

Required behavior:

- created once at agent initialization
- updated only for current pointers such as `latest_cycle_id` and `status`
- semantic history goes to append-only ledgers, not destructive rewrites

## cycle_ledger.jsonl

Append one line per cycle:

```json
{"schema":"adl.long_lived_agent_cycle_ledger_entry.v1","cycle_id":"cycle-000001","status":"success","started_at":"2026-04-16T10:00:00Z","completed_at":"2026-04-16T10:00:12Z","manifest_ref":"cycles/cycle-000001/cycle_manifest.json","summary_ref":"cycles/cycle-000001/cycle_summary.md"}
```

Rules:

- append-only
- no deletion on rerun
- failed cycles remain in the ledger
- recovery cycles get new cycle IDs

## provider_binding_history.jsonl

Append when the runtime binds a model/provider:

```json
{"schema":"adl.long_lived_agent_provider_binding.v1","cycle_id":"cycle-000001","provider_id":"local_ollama","model":"gemma4:latest","binding_status":"available","source":"model_roster_discovery"}
```

This is important because long-lived behavior can change when the bound model
changes.

## memory_index.json

Small local index:

```json
{
  "schema": "adl.long_lived_agent_memory_index.v1",
  "memory_namespace": "stock-league/value-monk",
  "append_only": true,
  "local_memory_refs": [
    "cycles/cycle-000001/memory_writes.jsonl"
  ],
  "obsmem_export_status": "not_exported"
}
```

The first slice should not require a live ObsMem backend. It must write memory
candidate files that can later be indexed.

## Operator Events

Path:

```text
operator_events.jsonl
```

Events:

- `created`
- `started`
- `stopped`
- `spec_revision_requested`
- `spec_revision_accepted`
- `stale_lease_recovered`
- `manual_tick_requested`

Example:

```json
{"schema":"adl.long_lived_agent_operator_event.v1","event":"stopped","at":"2026-04-16T11:00:00Z","reason":"demo complete","operator":"local"}
```

## Migration To v0.92

When v0.92 identity lands, the migration should:

1. create a real identity tuple
2. attach the old `agent_instance_id` as legacy continuity evidence
3. import cycle ledger references
4. preserve memory namespace lineage
5. keep old artifacts readable

The v0.92 system must not need to reinterpret old cycles as if they already had
full identity semantics.

## Acceptance Criteria

- Initializing an agent creates `continuity.json`, `agent_spec.locked.json`, and
  `cycle_ledger.jsonl`.
- Multiple cycles append ledger entries without deleting prior entries.
- Restarting the supervisor finds the latest cycle from the ledger.
- Provider binding history records model changes across cycles.
- Memory write candidates are durable even without a live ObsMem backend.
- The continuity file clearly says it is a pre-v0.92 handle.
- No doc or artifact claims this is the full v0.92 identity substrate.
