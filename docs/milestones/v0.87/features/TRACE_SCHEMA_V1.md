

# TRACE SCHEMA v1

## Metadata
- Owner: `adl`
- Status: `promoted`
- Target milestone: `v0.87`
- Parent: `docs/architecture/TRACE_SYSTEM_ARCHITECTURE.md`

## Purpose

Define the **canonical schema and event vocabulary** for Trace v1.

This document specifies:
- the event schema (required/optional fields)
- the canonical event types
- validation rules
- minimal guarantees required for review-grade reconstruction

This is the **feature-owner doc** for trace schema in `v0.87`.

## Design Constraints (from Architecture)

- Trace is the **authoritative structural record**.
- Artifacts carry payload truth; trace references them.
- Spans are **explicit and required**.
- Schema must be **stable and deterministic**.
- Provider attribution must separate `model_ref` and `provider_model_id`.

## Core Schema

### TraceEvent (canonical)

Required fields:
- `event_id`: string (UUID or stable unique id)
- `timestamp`: string (ISO-8601)
- `temporal_anchor`: object (see Chronosense alignment; required for temporal grounding)
- `event_type`: enum (see below)
- `trace_id`: string
- `run_id`: string
- `span_id`: string
- `parent_span_id`: string (nullable only for root span)
- `actor`: object
- `scope`: object

Optional fields (but recommended when applicable):
- `inputs_ref`: string (artifact path)
- `outputs_ref`: string (artifact path)
- `artifact_ref`: string (artifact path for single-payload or non-input/output references)
- `decision_context`: object (required for `DECISION`, `APPROVAL`, `REJECTION`, `REVISION`)
- `provider`: object (required for MODEL_INVOCATION)
- `error`: object (required for ERROR)
- `contract_validation`: object (required for CONTRACT_VALIDATION)

### Actor

- `type`: enum (`agent`, `tool`, `provider`, `skill`, `system`)
- `id`: string

### Scope

- `level`: enum (`run`, `step`, `substep`, `tool`, `model`, `skill`)
- `name`: string

### Provider (for MODEL_INVOCATION)

- `vendor`: string (normalized)
- `transport`: string
- `model_ref`: string (ADL canonical)
- `provider_model_id`: string (raw provider identifier, optional but strongly recommended)

### Error

- `code`: string
- `message`: string
- `details`: object (optional)

### ContractValidation

- `contract_id`: string
- `result`: enum (`pass`, `fail`)
- `details`: object

### TemporalAnchor (Chronosense Alignment)

The `temporal_anchor` field aligns Trace with the Chronosense model and MUST
capture objective temporal grounding for every event.

Required fields:
- `observed_at_utc`: string (ISO-8601; canonical wall-clock time)
- `monotonic_order`: integer (strictly increasing within a run)
- `agent_age`: string (duration since agent birth / temporal ephemeris)

Optional but strongly recommended:
- `observed_at_local`: string (localized timestamp for interaction frame)
- `prior_event_delta`: string (duration since previous event in same span)
- `turn_index`: integer (narrative/event clock index)
- `temporal_confidence`: enum (`high`, `medium`, `low`)

Notes:
- `timestamp` and `observed_at_utc` SHOULD be identical unless explicitly justified
- `monotonic_order` MUST NOT decrease within a run
- `agent_age` MUST be derived from the agent's temporal ephemeris (birth)

This structure separates:
- absolute time (UTC)
- monotonic ordering
- lifetime-relative time
- optional narrative/event time

## Event Vocabulary (v1)

Canonical event types:
- `RUN_START`
- `RUN_END`
- `STEP_START`
- `STEP_END`
- `MODEL_INVOCATION`
- `TOOL_INVOCATION`
- `SKILL_EXECUTION`
- `MEMORY_READ`
- `MEMORY_WRITE`
- `CONTRACT_VALIDATION`
- `DECISION`
- `APPROVAL`
- `REJECTION`
- `REVISION`
- `ERROR`

### Event Semantics

- `RUN_*`: lifecycle of full execution
- `STEP_*`: logical planning/execution units
- `MODEL_INVOCATION`: LLM call (must include provider block)
- `TOOL_INVOCATION`: external tool execution
- `SKILL_EXECUTION`: reusable operational unit
- `MEMORY_*`: interaction with ObsMem
- `CONTRACT_VALIDATION`: schema/contract checks
- `DECISION`: branching or reasoning commitment
- `APPROVAL / REJECTION / REVISION`: inline control or review outcomes recorded as explicit trace events
- `ERROR`: failure event with structured payload

### DecisionContext

For `DECISION`, `APPROVAL`, `REJECTION`, and `REVISION`, `decision_context`
MUST include:
- `context`: what was being decided
- `outcome`: what was approved, rejected, revised, or committed
- `rationale`: optional but strongly recommended explanation or basis

## Span Requirements

- Every event MUST belong to a span
- Spans MUST form a valid tree (no cycles)
- Root span corresponds to the run
- Parent-child relationships MUST be explicit (no inference)

## Ordering Guarantees

Trace ordering is not merely a serialization concern. It establishes the
agent's grounding in time and enables higher-order reasoning.

Chronosense requirement:
- Ordering MUST be consistent across both `timestamp` and `monotonic_order`
- In case of clock skew or ambiguity, `monotonic_order` is authoritative for sequencing

### Structural Ordering

- Events MUST be totally ordered within a span
- Cross-span ordering is defined by timestamps + parent relationships
- Timestamps MUST be monotonic within a span

This ensures that execution can be reconstructed deterministically.

### Relative Duration and Temporal Structure

Trace MUST also preserve relative temporal relationships, not just ordering:

- Systems SHOULD capture or allow derivation of:
  - duration between events
  - latency of operations (model/tool/skill)
  - elapsed time across spans
- Temporal adjacency and gaps MUST be inferable from timestamps

This enables reasoning about:
- sequencing vs concurrency
- delay, staleness, and responsiveness
- temporal clustering of events

Without duration, ordering is insufficient for meaningful temporal reasoning.

### Grounding in Spacetime

Together, ordering and duration ground the agent in a coherent temporal frame:

- The trace defines the agent's trajectory through events
- Each event is located in a shared temporal coordinate system
- The agent is not stateless; it exists as a temporally extended process

This is a prerequisite for:
- replay
- evaluation
- identity continuity

### Implications for Causal Reasoning

Explicit temporal structure enables causal interpretation:

- Earlier events can be evaluated as potential causes of later events
- Decision to action to outcome chains become inspectable
- Counterfactual reasoning becomes possible when alternatives are recorded via `DECISION` and `REVISION`

Trace therefore supports:
- causal attribution
- dependency analysis
- debugging of reasoning paths

### Epistemic Coherence and Reasonableness

A well-formed trace provides a basis for evaluating reasonableness:

- Decisions must be temporally situated relative to their inputs
- Outcomes must follow from prior context in a coherent sequence
- Incoherent or contradictory sequences become detectable

This aligns with a coherence theory of truth:
- correctness is evaluated not only locally (per event)
- but globally (across the temporal structure of the trace)

### Persistence of Identity

Temporal continuity in trace underwrites identity:

- The agent's identity is constituted by its sequence of events over time
- Persistent spans and runs define continuity across operations
- Without ordered and temporally grounded events, identity collapses into stateless reactions

Trace is therefore not just logging. It is the structural basis for:
- continuity
- accountability
- historical memory integration (via ObsMem)

## Artifact References

- `inputs_ref`, `outputs_ref`, and `artifact_ref` MUST point to:
  `artifacts/<run_id>/...`
- References MUST resolve to actual stored files
- Large payloads MUST NOT be embedded in events

## Validation Rules

A trace is **valid (v1)** if:

1. All required fields are present for every event
2. All events MUST include a valid `temporal_anchor` with required fields
3. All event types are from the canonical vocabulary
4. Span tree is well-formed (single root, no orphan spans)
5. Required subtype fields are present:
   - MODEL_INVOCATION → `provider`
   - ERROR → `error`
   - CONTRACT_VALIDATION → `contract_validation`
   - DECISION / APPROVAL / REJECTION / REVISION → `decision_context`
6. Artifact references resolve
7. No missing required lifecycle events:
   - at least one `RUN_START` and `RUN_END`

## Minimal Completeness Requirements

For a run to be review-valid:

- All step boundaries MUST be traced (`STEP_START` / `STEP_END`)
- All model/tool/skill executions MUST emit events
- All contract checks MUST emit `CONTRACT_VALIDATION`
- All failures MUST emit `ERROR`
- All control decisions SHOULD emit `DECISION` (required for complex flows)
- All events MUST include chronosense-compliant temporal anchors (UTC + monotonic + lifetime)

## Validation Surface

Primary validation mechanisms:
- Demo matrix trace scenarios (v0.87)
- Output cards referencing event IDs
- Reviewer inspection of trace + artifact tree

Acceptance criteria:
- A reviewer can reconstruct execution solely from:
  (trace structure + artifact payloads)
- Event stream is schema-valid and complete
- Provider attribution is present for all model calls
- No implicit behavior (all control decisions are explicit events)

## Non-Goals (v1)

- Cross-run aggregation schemas
- Compression or storage optimization
- Signed/cryptographic trace
- Visualization/UI schema

## Open Questions

- Exact timestamp precision requirements (ms vs ns)
- Whether `DECISION` should be required or context-dependent
- Whether any additional decision subtype fields are needed beyond `decision_context`

## Next Steps

Dependent feature docs:
1. `TRACE_RUNTIME_EMISSION.md`
2. `TRACE_ARTIFACT_MODEL.md`
3. `TRACE_REVIEW_PIPELINE.md`
4. `TRACE_OBSMEM_INGESTION.md`

Each must conform strictly to this schema.
