

# TRACE RUNTIME EMISSION v1

## Metadata
- Owner: `adl`
- Status: `draft`
- Target milestone: `v0.87`
- Parent: `TRACE_SYSTEM_ARCHITECTURE.md`
- Depends on: `TRACE_SCHEMA_V1.md`

## Purpose

Define how trace events are **emitted at runtime** by the ADL execution engine.

This document specifies:
- where in the execution lifecycle events are emitted
- how spans and context are created and propagated
- how schema compliance is enforced
- how emission integrates with providers, tools, skills, and memory

This is the **feature-owner doc** for runtime trace emission in `v0.87`.

## Design Constraints

- MUST conform strictly to `TRACE_SCHEMA_V1.md`
- MUST emit events as part of execution (not post-hoc)
- MUST maintain span hierarchy explicitly
- MUST be deterministic in structure
- MUST not depend on ObsMem or review system

## Emission Model

Trace emission is **inline with execution**, not asynchronous logging.

Every significant execution boundary triggers event emission.

### Core Principle

> If a reviewer cannot reconstruct a decision without inference, an event is missing.

## Execution Hooks

The runtime must emit events at the following points:

### Run Lifecycle

- On run start → `RUN_START`
- On run end → `RUN_END`

### Step Lifecycle

- Before step execution → `STEP_START`
- After step execution → `STEP_END`

### Model Invocation

- Immediately before or after model call → `MODEL_INVOCATION`

Must include:
- provider block
- inputs_ref
- outputs_ref

### Tool Invocation

- On tool execution → `TOOL_INVOCATION`

### Skill Execution

- On skill entry → `SKILL_EXECUTION`

### Memory Interaction

- On read → `MEMORY_READ`
- On write → `MEMORY_WRITE`

### Contract Validation

- After validation → `CONTRACT_VALIDATION`

### Decision Points

- When branching or committing to a path → `DECISION`

### Inline Control Outcomes

- On inline approval during execution/control handling → `APPROVAL`
- On inline rejection during execution/control handling → `REJECTION`
- On inline revision during execution/control handling → `REVISION`

These events belong to runtime emission only when the outcome occurs **during**
execution or a bounded control-path step.

Post-run review-pipeline findings and recommendations are produced by
`TRACE_REVIEW_PIPELINE.md` and `REVIEW_SURFACE_FORMALIZATION.md`, not emitted
here as runtime facts.

### Error Handling

- On any failure → `ERROR`

## Span Management

### Requirements

- A root span MUST be created at run start
- Each step MUST create a child span
- Tool/model/skill calls MUST create nested spans

### Rules

- Spans MUST be created before emitting events within them
- Span closure must align with lifecycle boundaries
- Parent-child relationships MUST be explicit

## Trace Context Propagation

Trace context must be passed through all execution layers:

Context fields:
- `trace_id`
- `run_id`
- `span_id`
- `parent_span_id`
- `agent_id`

### Propagation Rules

- Context MUST be passed to:
  - model providers
  - tools
  - skills
  - memory layer
- New spans MUST derive from current context

## Event Construction

Each emitted event MUST:

1. Be schema-valid (`TRACE_SCHEMA_V1.md`)
2. Include correct span and parent references
3. Include required subtype fields
4. Reference artifacts instead of embedding payloads

## Artifact Interaction

Runtime must:

- write inputs/outputs to artifact store BEFORE emitting references
- ensure paths follow:
  `artifacts/<run_id>/...`
- ensure references are stable and resolvable

## Provider Integration

For every `MODEL_INVOCATION`:

- runtime MUST populate:
  - `vendor`
  - `transport`
  - `model_ref`
  - `provider_model_id` (if available)

- provider adapters MUST supply raw provider identifiers
- runtime MUST normalize into schema

## Determinism Requirements

- Event emission order MUST be deterministic within a span
- No hidden or implicit operations
- All control decisions MUST emit events

Allowed variability:
- timestamps
- run_id / trace_id

## Failure Handling

- On failure, runtime MUST emit `ERROR`
- Error event MUST include structured error payload
- Span MUST still close properly if possible

## Validation Surface

Primary validation mechanisms:
- Demo matrix trace scenarios (v0.87)
- Output cards referencing emitted events
- Direct inspection of trace logs and artifact tree

### Acceptance Criteria

- Every execution path produces a complete trace
- No missing required events
- Span hierarchy is correct and consistent
- Artifact references resolve
- Provider attribution is complete
- Reviewer can reconstruct execution without inference

## Non-Goals (v1)

- Async/batched trace pipelines
- Streaming trace to external systems
- Performance optimization of emission

## Open Questions

- Should emission be synchronous or buffered within a step?
- Performance overhead limits for trace emission
- Standard helpers/macros for emission in Rust runtime

## Implementation Notes

Expected implementation areas:
- execution engine core
- provider adapters
- tool execution layer
- skill framework

All must integrate with a unified trace emitter.

## Next Steps

Dependent docs:
- `TRACE_ARTIFACT_MODEL.md`
- `TRACE_REVIEW_PIPELINE.md`
- `TRACE_OBSMEM_INGESTION.md`

All runtime emission must remain aligned with schema and architecture.
