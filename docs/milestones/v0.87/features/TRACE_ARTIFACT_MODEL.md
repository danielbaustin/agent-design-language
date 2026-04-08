

# TRACE ARTIFACT MODEL v1

## Metadata
- Owner: `adl`
- Status: `promoted`
- Target milestone: `v0.87`
- Parent: `TRACE_SYSTEM_ARCHITECTURE.md`
- Depends on:
  - `TRACE_SCHEMA_V1.md`
  - `TRACE_RUNTIME_EMISSION.md`

## Purpose

Define the **artifact model** for ADL trace.

This document specifies:
- what artifacts are
- how artifacts are stored and referenced
- how artifacts relate to trace events
- how artifacts support replay, validation, and review

This is the **feature-owner doc** for artifact handling in `v0.87`.

## Core Principle

> Trace describes execution. Artifacts contain execution payloads.

Artifacts are the material substrate of execution; trace provides its temporal
and causal structure. Together they form the complete representation of agent
experience.

Execution truth is the combination of:
- trace (structure, decisions, causality)
- artifacts (inputs, outputs, data)

Neither alone is sufficient.

## Artifact Definition

An artifact is any **persisted payload** referenced by trace events.

Examples:
- model inputs
- model outputs
- tool inputs/outputs
- skill intermediate data
- contract validation payloads

Artifacts MUST NOT be embedded directly in trace events (except small metadata).

## Storage Model

Artifacts are stored in a deterministic directory structure:

```
artifacts/<run_id>/<scope>/<artifact_id>.<ext>
```

### Required Properties

- paths MUST be deterministic
- artifacts MUST be immutable once written
- artifact IDs MUST be unique within a run
- artifact identity SHOULD be derivable from deterministic context (for example
  event_id, role, index)
- content hashing MAY be used for integrity validation

### Scope Examples

- `steps/`
- `models/`
- `tools/`
- `skills/`
- `contracts/`

## Artifact Types

Artifacts SHOULD be typed via metadata.

Common types:
- `model_input`
- `model_output`
- `tool_input`
- `tool_output`
- `memory_snapshot`
- `contract_payload`

Type information MAY be included in trace event fields.

When multiple artifacts are associated with a single event, their semantic role
MUST be explicit (for example input, output, intermediate).

## Artifact References

Trace events reference artifacts via fields such as:

- `inputs_ref`
- `outputs_ref`
- `artifact_ref`

### Requirements

- references MUST be resolvable
- references MUST point to existing artifacts
- references MUST be stable across review and replay
- artifact references MUST correspond to the emitting event's temporal context
- artifacts are considered part of the event's temporal anchor by association

## Write Semantics

Artifacts MUST be written **before** the trace event that references them.

### Rules

- no dangling references
- write must succeed before event emission
- failures MUST produce ERROR events
- artifact writes MUST be atomic (no partial files visible)
- failed writes MUST NOT leave partially readable artifacts

## Read Semantics

Consumers of trace (reviewers, tools) MUST:

- resolve artifact paths
- load payloads on demand
- avoid assuming payloads are embedded in trace

## Relationship to Trace

Each relevant trace event MUST reference artifacts when payloads exist.

Examples:

- `MODEL_INVOCATION` → input + output artifacts
- `TOOL_INVOCATION` → input + output artifacts
- `CONTRACT_VALIDATION` → validation payload artifact

## Determinism Requirements

- artifact paths MUST be reproducible from trace context
- artifact naming MUST be consistent across runs (modulo run_id)

Allowed variability:
- content of model outputs

## Replay Support

Artifacts enable **logical replay** of execution.

Replay requires:
- trace structure
- artifact payloads

Replay system MUST:
- follow trace order
- load artifacts at each step
- reconstruct decision flow
- artifact loading MUST follow monotonic_order
- artifacts MUST NOT be accessed out of temporal order during replay

## Validation Requirements

Artifact validation includes:

- referenced artifacts exist
- artifact format is readable
- artifact matches expected type

Failures MUST be surfaced during validation.

## Review Workflow

Reviewer flow:

1. inspect trace event
2. follow artifact reference
3. examine payload
4. correlate with subsequent events

Artifacts are essential for understanding model behavior.

## Size and Performance Considerations

- artifacts MAY be large
- runtime SHOULD avoid copying large payloads unnecessarily
- compression MAY be applied (future)
- implementations SHOULD support streaming or chunking for large artifacts
  (future-compatible)

## Security Considerations

- artifacts MAY contain sensitive data
- access control is out of scope for v1 but must be considered
- artifact paths must not allow traversal vulnerabilities

## Non-Goals (v1)

- distributed artifact storage
- remote artifact retrieval
- deduplication across runs

## Open Questions

- standard serialization formats (JSON vs others)
- artifact size limits
- streaming artifacts vs full writes

## Implementation Notes

Artifact handling must be integrated with:

- runtime execution engine
- provider adapters
- tool and skill layers

A shared artifact utility layer is recommended.

## Acceptance Criteria

- all payloads are stored as artifacts
- all artifact references resolve correctly
- no payload duplication inside trace events
- artifact paths are deterministic
- reviewer can access all payloads via references

## Next Steps

Future extensions:
- artifact indexing for ObsMem
- artifact compression strategies
- integration with replay tooling

Artifact modeling completes the trace + artifact execution truth model.
