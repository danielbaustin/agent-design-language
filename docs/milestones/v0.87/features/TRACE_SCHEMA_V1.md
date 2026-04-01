

# TRACE SCHEMA v1

## Metadata
- Owner: `adl`
- Status: `draft`
- Target milestone: `v0.87`
- Parent: `TRACE_SYSTEM_ARCHITECTURE.md`

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

- Events MUST be totally ordered within a span
- Cross-span ordering is defined by timestamps + parent relationships
- Timestamps MUST be monotonic within a span

## Artifact References

- `inputs_ref`, `outputs_ref`, and `artifact_ref` MUST point to:
  `artifacts/<run_id>/...`
- References MUST resolve to actual stored files
- Large payloads MUST NOT be embedded in events

## Validation Rules

A trace is **valid (v1)** if:

1. All required fields are present for every event
2. All event types are from the canonical vocabulary
3. Span tree is well-formed (single root, no orphan spans)
4. Required subtype fields are present:
   - MODEL_INVOCATION → `provider`
   - ERROR → `error`
   - CONTRACT_VALIDATION → `contract_validation`
   - DECISION / APPROVAL / REJECTION / REVISION → `decision_context`
5. Artifact references resolve
6. No missing required lifecycle events:
   - at least one `RUN_START` and `RUN_END`

## Minimal Completeness Requirements

For a run to be review-valid:

- All step boundaries MUST be traced (`STEP_START` / `STEP_END`)
- All model/tool/skill executions MUST emit events
- All contract checks MUST emit `CONTRACT_VALIDATION`
- All failures MUST emit `ERROR`
- All control decisions SHOULD emit `DECISION` (required for complex flows)

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
