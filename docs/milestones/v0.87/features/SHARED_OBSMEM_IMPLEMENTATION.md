

# SHARED OBSMEM IMPLEMENTATION v1

## Metadata
- Owner: `adl`
- Status: `draft`
- Target milestone: `v0.87`
- Purpose: Define the bounded implementation scope for shared ObsMem in `v0.87` as a real substrate layer tied to trace and review surfaces.

## Purpose

Define the **shared ObsMem implementation scope** for `v0.87`.

This document specifies:
- what shared ObsMem means in `v0.87`
- what is in scope for the first real implementation
- how shared ObsMem relates to trace, artifacts, and review
- what later memory/social/governance features are explicitly deferred

This is the implementation-facing scope doc for the shared-memory substrate in `v0.87`.

This doc is the bounded `v0.87` implementation owner for shared memory.
It intentionally stops short of the later-layer social-memory semantics explored
in `SHARED_SOCIAL_MEMORY.md`.

## Core Principle

> Shared ObsMem in `v0.87` is a bounded, inspectable, trace-linked shared memory substrate.

It is **not** yet:
- full social memory
- governance-aware memory
- implicit long-term learning
- open-ended summarization

In `v0.87`, shared ObsMem exists to make execution-derived observations available across runs and surfaces in a controlled, reviewable way.

## Why It Belongs in `v0.87`

`v0.87` is a substrate milestone. Shared ObsMem belongs here because later milestones depend on a real shared-memory base.

Without a real shared-memory substrate:
- trace remains isolated from memory
- replay/review cannot connect execution history to persistent observations
- later identity, reputation, and social memory layers would float without foundation

So the `v0.87` goal is not to finish memory as a whole. The goal is to land the first **credible shared-memory layer**.

## Scope Summary

### In scope for `v0.87`
- shared-memory records derived from trace
- deterministic ingestion from completed execution traces
- bounded indexing and retrieval
- explicit record identity and provenance
- trace-linked reviewability
- storage discipline for shared, reusable observations across runs

### Out of scope for `v0.87`
- social memory semantics
- governance / citizenship / reputation reasoning
- autonomous summarization or memory rewriting
- adaptive policy or learning behavior
- identity-rich longitudinal world models
- full reasoning-graph integration

## Definition of Shared ObsMem in `v0.87`

Shared ObsMem in `v0.87` means:
- memory records are not tied only to one ephemeral run context
- multiple later runs or review surfaces can query them
- each memory record remains grounded in source trace and artifact references

This is a **shared substrate**, not yet a fully social substrate.

## Shared ObsMem Record Model

Each shared ObsMem record should minimally include:
- `record_id`
- `record_type`
- `trace_ref` (event_id or equivalent)
- `run_id`
- `agent_id` (when available)
- `timestamp`
- `model_ref` (when applicable)
- `artifact_refs` (if payloads are external)
- `summary` or structured observation fields

For the bounded `v0.87` substrate, the runtime contract uses a deterministic
`trace_event_refs` array as the concrete "event_id or equivalent" surface.
Each item records the event sequence plus bounded step/delegation identity so a
reviewer can walk from memory back to execution truth without hidden inference.

### Example record types
- `obs.model_call`
- `obs.decision`
- `obs.validation_result`
- `obs.error`
- `obs.step_outcome`

These are implementation-oriented observation records, not later social/reputation records.

## Relationship to Trace

Shared ObsMem is derived from trace.

Rules:
- trace remains the authoritative execution record
- ObsMem stores structured observations derived from trace
- every shared ObsMem record MUST be traceable back to a source event or bounded event set

This means a reviewer must be able to answer:
- where did this memory come from?
- what execution produced it?
- what artifact or event supports it?

## Relationship to Artifacts

ObsMem does not absorb artifact payloads wholesale.

Rules:
- large payloads remain in artifact storage
- ObsMem stores references to payloads when needed
- bounded summaries/observation fields may be stored directly

This preserves:
- storage discipline
- reviewability
- replay compatibility

## Storage and Access Model

### Storage

For `v0.87`, storage may be simple, but it must be explicit and bounded.

Acceptable early characteristics:
- deterministic record schema
- stable keys/IDs
- explicit storage location/backend
- no hidden mutation

The backend may still evolve later, but the substrate contract must be real now.

### Access

Shared ObsMem access in `v0.87` should support bounded retrieval by:
- `run_id`
- `agent_id`
- `record_type`
- `trace_ref`
- simple time ordering

This is enough for substrate truth and demoability without overcommitting to later retrieval intelligence.

## Ingestion Model

Shared ObsMem ingestion should follow the trace ingestion rules already defined elsewhere.

For `v0.87`:
- ingestion happens from completed trace + artifact truth
- mappings from trace event types to ObsMem record types must be explicit
- ingestion must be deterministic and idempotent

Examples:
- `MODEL_INVOCATION` → `obs.model_call`
- `DECISION` → `obs.decision`
- `CONTRACT_VALIDATION` → `obs.validation_result`
- `ERROR` → `obs.error`

## Retrieval Model

Retrieval in `v0.87` must remain bounded and inspectable.

The goal is not “smart memory.” The goal is:
- deterministic queryability
- traceable provenance
- support for review, demos, and later systems

Retrieval should therefore:
- return structured records
- preserve provenance fields
- never hide source trace/artifact context

## Determinism Requirements

Shared ObsMem implementation is acceptable only if:
- the same trace produces the same memory records
- record identity and record shape are stable
- retrieval ordering is explicit and explainable
- no hidden summarization or mutation occurs

Allowed variability:
- runtime-generated IDs if their generation rules are stable and documented

Not allowed:
- probabilistic summarization in v1
- silent record rewriting
- memory records with no trace provenance

## Reviewability Requirements

A reviewer must be able to:
- inspect a memory record
- locate the source trace event
- follow any artifact reference
- understand why the record exists

This is mandatory for `v0.87` because the milestone is about external credibility and reviewer-facing proof surfaces.

## Demo Surface for `v0.87`

The shared ObsMem demo should prove:
- memory records are created from real execution traces
- records can be queried in a bounded way
- records preserve provenance back to trace/artifacts

Expected artifact family:
- `artifacts/v087/shared_obsmem/README.md`
- `artifacts/v087/shared_obsmem/memory_entries.json`
- `artifacts/v087/shared_obsmem/trace_links.json`

Primary proof surface:
- `artifacts/v087/shared_obsmem/memory_entries.json`

Secondary proof surface:
- `artifacts/v087/shared_obsmem/trace_links.json`

For the current in-repo runtime/demo implementation, the equivalent proof lives
in the ObsMem learning artifacts where each returned entry carries
`trace_event_refs` and the index summary reports the trace-reference set used to
justify the stored record.

## Acceptance Criteria

The shared ObsMem implementation is acceptable for `v0.87` when:
- a real shared-memory record model exists
- records are derived deterministically from trace
- records preserve provenance to trace and artifacts
- bounded retrieval works over real stored records
- demo surfaces prove the shared-memory substrate without inflating later social/governance claims
- the implementation is clearly a foundation layer, not a disguised later-milestone memory system

## Open Questions

- what exact storage backend should be used first?
- what minimal indexing set is required for the first usable implementation?
- should summaries be fully structured in v1, or is limited textual summary acceptable when provenance is preserved?
- what is the first canonical in-repo location or module boundary for shared ObsMem implementation?

## Non-Goals (v1)

- social relationship memory
- citizenship / reputation memory
- governance-aware memory transformations
- probabilistic learning from memory
- full reasoning-graph or hypothesis-engine integration

## Next Steps

Derive or align the following from this doc:
- shared ObsMem schema / record-format doc
- ingestion implementation issue(s)
- retrieval/query surface doc or issue
- demo implementation for `D3`
- later linkages into identity, reputation, and reasoning-graph work

Shared ObsMem in `v0.87` is the bridge from isolated execution history to persistent, shared, and reviewable cognitive substrate memory.
