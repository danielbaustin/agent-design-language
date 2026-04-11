

# TRACE VALIDATION AND REVIEW v1

## Metadata
- Owner: `adl`
- Status: `promoted`
- Target milestone: `v0.87`
- Parent: `docs/architecture/TRACE_SYSTEM_ARCHITECTURE.md`
- Depends on:
  - `TRACE_SCHEMA_V1.md`
  - `TRACE_RUNTIME_EMISSION.md`
  - `TRACE_ARTIFACT_MODEL.md`

## Purpose

Define how traces are **validated and reviewed** in ADL.

This document specifies:
- what constitutes a valid trace
- how traces are inspected and reconstructed
- how reviewers (human or automated) interact with trace data
- how trace supports correctness, auditability, and control

This is the **feature-owner doc** for trace validation and review in `v0.87`.

## Core Principle

> Trace exists to make execution **legible, auditable, and reconstructable**.

Validation ensures correctness.
Review ensures understanding.

Boundary note:
- this doc defines what makes a trace valid and review-grade
- `TRACE_REVIEW_PIPELINE.md` defines how those checks and review outputs are
  orchestrated end-to-end
- this doc owns semantics and required visibility, not pipeline staging

## Validation Model

Validation operates at three levels:

### 1. Schema Validation

Every event MUST conform to `TRACE_SCHEMA_V1.md`.

Checks include:
- required fields present
- correct types
- valid enum values
- valid references (e.g., span_id, trace_id)

Failure mode:
- event is rejected or marked invalid

### 2. Structural Validation

The trace MUST form a coherent execution structure.

Checks include:
- span hierarchy is well-formed (no orphan spans)
- parent/child relationships are consistent
- spans open and close correctly
- event ordering is valid within spans

Failure examples:
- missing STEP_END
- invalid parent_span_id
- overlapping or broken span boundaries

### 3. Semantic Validation

Trace MUST reflect meaningful execution behavior.

Checks include:
- required events are present (e.g., CONTRACT_VALIDATION after step)
- decisions are explicitly recorded
- errors are captured as ERROR events
- provider attribution is present for MODEL_INVOCATION

This is the layer that prevents "silent execution".

## Review Model

Review is the process of **reconstructing execution from trace + artifacts**.

### Inputs to Review

- trace event stream
- artifact references (inputs/outputs)
- provider metadata

### Reviewer Goals

A reviewer must be able to:

- understand what happened
- understand why decisions were made
- verify correctness
- identify failures or inconsistencies

### Reconstruction Requirements

Trace MUST support:

- step-by-step replay (logical, not necessarily executable)
- identification of all decision points
- inspection of all inputs and outputs

## Review Surfaces

### Human Review

- reading trace logs
- inspecting artifacts
- correlating events with execution steps

### Automated Review

- validation checks (schema + structure + semantics)
- invariant enforcement
- anomaly detection (missing events, invalid flows)

Future:
- policy-driven review (Freedom Gate integration)

## Decision Visibility

All meaningful decisions MUST be visible in trace.

Required decision-related events:
- `DECISION`
- `APPROVAL`
- `REJECTION`
- `REVISION`

Each must include:
- context (what was being decided)
- rationale (if available)
- outcome

## Contract Visibility

Contract validation MUST be explicitly recorded.

`CONTRACT_VALIDATION` events must include:
- contract reference
- validation result (pass/fail)
- failure details (if any)

This is critical for auditability.

## Artifact Role in Review

Trace references artifacts; artifacts contain payload truth.

Reviewer workflow:
1. read event
2. follow artifact reference
3. inspect payload

Trace alone is insufficient without artifact resolution.

## Failure and Error Review

On failure:

- `ERROR` event MUST be present
- must include structured error information
- must be associated with correct span

Reviewer must be able to:
- identify where failure occurred
- understand cause
- trace upstream context

## Determinism and Reproducibility

Trace must support reproducibility at the logical level.

Reviewer should be able to:
- follow the same sequence of steps
- understand all inputs and outputs
- reproduce reasoning path (not necessarily model output)

## Validation Surface

Validation is exercised via:

- demo matrix scenarios (v0.87)
- output cards referencing trace
- automated validation checks

## Acceptance Criteria

- All events are schema-valid
- Span hierarchy is correct
- No missing required events
- All decisions are visible
- All contract validations are recorded
- Artifact references resolve correctly
- Reviewer can reconstruct execution without inference

## Non-Goals (v1)

- Full UI for trace visualization
- Distributed trace aggregation
- Advanced analytics or metrics dashboards

## Open Questions

- Should validation occur inline during execution or post-run?
- How strict should semantic validation be in v1?
- Standard tooling for trace inspection (CLI vs UI)

## Implementation Notes

Validation responsibilities likely split across:

- runtime (basic validation)
- tooling (deeper validation)
- CI / demo validation checks

Review is initially manual + CLI-based.

## Next Steps

Future extensions:
- policy enforcement (Freedom Gate)
- ObsMem ingestion validation
- replay tooling

Trace validation and review form the foundation for trust in ADL execution.
