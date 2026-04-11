


# TRACE → OBSMEM INGESTION v1

## Metadata
- Owner: `adl`
- Status: `promoted`
- Target milestone: `v0.87`
- Parent: `docs/architecture/TRACE_SYSTEM_ARCHITECTURE.md`
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
- derived memory records MUST preserve explicit trace-event references using
  event IDs or a deterministic equivalent

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
- temporal_anchor fields MUST be preserved without loss
- multiple clock representations MUST remain distinguishable

This supports:
- replay-like reasoning
- causal analysis

### Chronosense Integration

ObsMem ingestion MUST preserve and expose chronosense as a first-class property
of memory.

Chronosense refers to an agent's ability to situate events in time in a way
that is meaningful for reasoning.

At a basic level, this means understanding what happened before and after what.

More formally, chronosense consists of three coupled aspects:

1. Ordering: the sequence of events
2. Duration: the relative spacing between events
3. Anchoring: grounding events in shared temporal reference frames

Ingestion MUST preserve all three aspects where available.

### Chronosense Model Alignment

Chronosense in ObsMem MUST align with the Substance of Time model.

This includes preserving multiple temporal frames simultaneously:

- wall-clock time (UTC / shared reference)
- monotonic order (strict execution ordering)
- lifetime-relative time (`agent_age` since temporal ephemeris)
- narrative or event time (`turn_index`, workflow step)

These frames MUST remain internally consistent and mappable across ingestion.
Loss of any one frame degrades temporal reasoning capability.

### Requirements

- temporal ordering MUST be reconstructible from ObsMem
- relative durations SHOULD be preserved when present in trace
- temporal anchors (UTC, local, agent_age, monotonic order) MUST be retained

### Implications

Chronosense is not only about ordering. It enables:

- causal reasoning by distinguishing sequence from causation
- reasonableness checks about whether outcomes follow plausibly from prior states
- coherence-based validation across time
- identity persistence through temporally ordered memory

Without chronosense, ObsMem degenerates into unordered records.
With it, ObsMem becomes a temporally grounded cognitive substrate.

### Subjective Temporal Modeling (Forward-Compatible)

While v1 ingestion focuses on objective temporal anchoring, the system SHOULD
remain compatible with future extensions that incorporate subjective temporal
signals.

These may include:
- event density
- attention-weighted duration
- episodic grouping
- reconstructed or simulated time

ObsMem MUST NOT assume that all temporal reasoning is derived solely from clock
time.

This keeps the ingestion surface compatible with future cognitive layers that
model mind time in addition to physical time.

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
- in the v0.87 runtime contract, that equivalent is a stable list of
  `trace_event_refs` carrying event sequence, event kind, and bounded step or
  delegation identity

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
