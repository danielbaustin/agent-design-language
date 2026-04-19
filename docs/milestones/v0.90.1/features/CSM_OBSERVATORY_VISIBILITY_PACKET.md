# CSM Observatory Visibility Packet

## Purpose

Define the first contract for seeing the Cognitive SpaceTime Manifold.

The CSM Observatory must not scrape arbitrary runtime files or invent live state
from a UI. It reads one bounded visibility packet that separates artifact-backed
facts, fixture evidence, missing evidence, and future scope.

Schema name: adl.csm_visibility_packet.v1

## Design Goal

The packet should make the manifold legible before the static console exists.
Every consumer should be able to answer three questions:

- What is alive?
- What is changing?
- What requires judgment?

This contract is intentionally richer than a status endpoint. It is the
reviewable bridge between Runtime v2 artifacts and the later Kai
Krause-inspired Observatory console: manifold header, citizen constellation,
kernel pulse, Freedom Gate docket, trace ribbon, and operator action rail.

## Required Sections

The packet is a JSON object with these required top-level sections:

- schema
- packet_id
- generated_at
- source
- manifold
- kernel
- citizens
- episodes
- freedom_gate
- invariants
- resources
- trace
- operator_actions
- review

## Source Truth

The source section must make claim strength explicit.

Allowed modes:

- fixture
- captured_artifacts
- live_runtime

The first v0.90.1 fixture uses fixture mode. A fixture packet must say it is a
fixture and must not present itself as a live runtime capture.

Evidence level values:

- fixture_backed
- artifact_backed
- live_runtime_backed
- missing
- deferred

Every major section should expose enough evidence references or caveats for a
reviewer to understand whether it is backed by current Runtime v2 artifacts,
fixture data, or planned future work.

## Section Contract

### Manifold

Required fields:

- manifold_id
- display_name
- state
- lifecycle
- current_tick
- uptime
- policy_profile
- snapshot_status
- health
- evidence_refs

Allowed state values:

- initialized
- running
- quiescing
- sleeping
- sealed
- rehydrating
- degraded
- blocked

### Kernel

Required fields:

- scheduler_state
- trace_state
- invariant_state
- resource_state
- service_states
- active_guardrails
- pulse

Each service state must include service_id, service_kind, lifecycle_state,
last_event_sequence, and evidence_ref.

Allowed lifecycle_state values:

- registered
- ready
- active
- degraded
- blocked
- missing

### Citizens

Each citizen must include:

- citizen_id
- display_name
- role
- lifecycle_state
- continuity_status
- current_episode
- resource_balance
- recent_decisions
- capability_envelope
- alerts
- evidence_refs

Allowed lifecycle_state values:

- proposed
- active
- awake
- sleeping
- paused
- degraded
- blocked
- suspended
- migrating

### Episodes

Each episode must include:

- episode_id
- title
- state
- citizen_ids
- started_at
- last_event
- proof_surface
- blocked_reason

Allowed state values:

- planned
- active
- completed
- blocked
- deferred
- failed

### Freedom Gate

Required fields:

- recent_docket
- allow_count
- defer_count
- refuse_count
- open_questions
- rejected_actions

Each docket entry must identify the action, actor, decision, rationale,
evidence_ref, and whether the entry is fixture-backed or artifact-backed.

Allowed decision values:

- allow
- defer
- refuse

### Invariants

Each invariant must include:

- invariant_id
- name
- state
- severity
- last_checked
- evidence_ref

Allowed states:

- healthy
- warning
- violated
- blocked
- missing
- deferred

Allowed severities:

- info
- low
- medium
- high
- critical

### Resources

Required fields:

- compute_units
- memory_pressure
- queue_depth
- fairness_notes
- scarcity_events

### Trace

Required fields:

- trace_tail
- causal_gaps
- latest_operator_event
- latest_citizen_event
- latest_kernel_event

Trace events must keep repository-relative evidence references. Public packets
must not contain host-absolute paths, private endpoints, secrets, raw prompts, or
tool arguments.

### Operator Actions

Required fields:

- available_actions
- disabled_actions
- required_confirmations
- safety_notes

The v0.90.1 packet is read-only. Available actions may describe future affordance
intent, but disabled actions must explain why live mutation is unavailable.

### Review

Required fields:

- primary_artifacts
- missing_artifacts
- demo_classification
- caveats
- next_consumers

The demo classification for the initial fixture must be fixture_backed. The
review section must name later consumers: static console prototype, operator
report generator, CLI integration, and operator command packet design.

## Runtime v2 Mapping

Current Runtime v2 artifacts map into the packet as follows:

| Runtime v2 artifact | Packet section |
| --- | --- |
| runtime_v2/manifold.json | manifold |
| runtime_v2/kernel/service_registry.json | kernel.service_states |
| runtime_v2/kernel/service_state.json | kernel.service_states and kernel pulse |
| runtime_v2/kernel/service_loop.jsonl | trace.trace_tail and kernel pulse |
| runtime_v2/citizens/active_index.json | citizens |
| runtime_v2/citizens/pending_index.json | citizens |
| runtime_v2/citizens/*.json | citizens |
| runtime_v2/invariants/policy.json | invariants |
| runtime_v2/operator/control_report.json | operator_actions and review |
| future Freedom Gate decision packets | freedom_gate |
| future resource ledger | resources |
| future snapshot and rehydration packets | manifold.snapshot_status and trace |

Missing future artifacts should be recorded as missing or deferred evidence, not
filled in as if they already exist.

## Public Safety Rules

Public Observatory packets must not contain:

- absolute host paths
- local network endpoints
- API keys, bearer tokens, private keys, or secret-like values
- raw prompts or tool arguments
- private operator data
- live mutation claims when only fixture data exists

The validator for this issue enforces the required sections, core vocabularies,
fixture labeling, and obvious path or endpoint leakage.

## Proof Surfaces

- Schema-style contract: adl/schemas/csm_visibility_packet.v1.schema.json
- Fixture packet: demos/fixtures/csm_observatory/proto-csm-01-visibility-packet.json
- Validator: adl/tools/validate_csm_visibility_packet.py
- Focused test: adl/tools/test_csm_visibility_packet.sh

## Non-Goals

- Do not build the HTML Observatory in this issue.
- Do not implement live operator command execution.
- Do not claim v0.92 identity, migration, or first-birthday semantics.
- Do not turn the packet into a generic log dump.
