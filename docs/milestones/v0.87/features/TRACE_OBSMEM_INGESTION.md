


# TRACE → OBSMEM INGESTION v1

## Metadata
- Owner: `adl`
- Status: `draft`
- Target milestone: `v0.87`
- Parent: `TRACE_SYSTEM_ARCHITECTURE.md`
- Depends on:
  - `TRACE_SCHEMA_V1.md`
  - `TRACE_RUNTIME_EMISSION.md`
  - `TRACE_ARTIFACT_MODEL.md`
  - `TRACE_VALIDATION_AND_REVIEW.md`

## Purpose

Define how **trace data is ingested into ObsMem (Observational Memory)**.

This document specifies:
- what data from trace is persisted into memory
- how events are transformed into memory records
- how artifacts are referenced or summarized
- how ingestion supports retrieval, learning, and reasoning

This is the **feature-owner doc** for trace → ObsMem ingestion in `v0.87`.

For `v0.87`, this ingestion surface feeds bounded shared ObsMem records, not
the later-layer social-memory semantics described in `SHARED_SOCIAL_MEMORY.md`.

## Core Principle

> ObsMem does not store raw execution—it stores **structured observations derived from execution**.

Trace is the authoritative execution record.
ObsMem is a **derived, queryable memory layer**.

## Ingestion Model

Ingestion is a **post-execution pipeline** that consumes:

- trace events
- artifact references

and produces:

- ObsMem records

### Key Constraint

Ingestion MUST NOT modify or reinterpret trace data destructively.

## What Gets Ingested

Not all trace data is stored directly.

### Categories of ingestion

1. **Event-derived observations**
2. **Decision records**
3. **Outcome summaries**
4. **Error records**

### Examples

- MODEL_INVOCATION → model usage record
- DECISION → decision node in memory graph
- CONTRACT_VALIDATION → validation outcome record
- ERROR → failure record

## Transformation Rules

Trace events are transformed into **ObsMem records**.

### Requirements

- transformation MUST be deterministic
- mapping MUST be explicit per event type
- no implicit inference in v1

### Example Mapping

- `MODEL_INVOCATION` → `obs.model_call`
- `DECISION` → `obs.decision`
- `ERROR` → `obs.error`

## Artifact Handling

Artifacts are not fully copied into ObsMem.

### Rules

- ObsMem MUST store references to artifacts
- selective summarization MAY occur (future)
- large payloads MUST remain external

## Memory Structure

ObsMem records SHOULD support:

- indexing by run_id
- indexing by agent_id
- indexing by event type
- graph relationships (decision chains, step sequences)

This aligns with future reasoning graph work.

## Identity Integration

ObsMem ingestion MUST include identity context:

- agent_id
- run_id
- model_ref (where applicable)

This enables:
- continuity
- longitudinal reasoning
- identity-based retrieval

## Temporal Structure

ObsMem records MUST preserve temporal order.

### Requirements

- timestamps preserved
- ordering derivable from trace

This supports:
- replay-like reasoning
- causal analysis

## Determinism Requirements

- same trace MUST produce identical ObsMem records
- ingestion MUST be idempotent
- no hidden randomness

## Validation Requirements

Ingestion MUST validate:

- source trace is valid
- artifact references resolve
- required fields exist

Failures MUST:
- be logged
- not corrupt existing memory

## Review and Audit

ObsMem MUST remain traceable back to source trace.

### Requirements

- each record MUST include trace reference (event_id or equivalent)
- reviewer MUST be able to trace memory → execution

## Non-Goals (v1)

- learning or model adaptation
- probabilistic summarization
- cross-run inference

## Open Questions

- schema for ObsMem records (flat vs graph-first)
- storage backend (sqlite, redb, etc.)
- indexing strategies

## Implementation Notes

Initial implementation may be:

- ingestion pipeline integrated with review pipeline
- simple record store

Future separation likely as ObsMem evolves.

## Acceptance Criteria

- ingestion runs on completed traces
- records are deterministic and reproducible
- all key events produce corresponding ObsMem records
- artifact references are preserved
- memory records can be traced back to source events

## Next Steps

Future extensions:
- reasoning graph integration (v0.9+)
- hypothesis engine inputs
- retrieval APIs

ObsMem ingestion is the bridge from execution to persistent cognitive memory.
