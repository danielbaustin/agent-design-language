# Temporal Query And Retrieval

## Status

Promoted milestone feature doc for `v0.88`.

## Purpose

Define how ADL should support retrieval and querying over time-aware cognitive records.

Cluster role:
- this is the retrieval/query surface doc for the likely `v0.88` temporal package
- it owns temporal query primitives, retrieval semantics, staleness-oriented access, and indexing expectations
- it should not restate the full chronosense philosophy or schema contract in detail

Primary neighboring docs:
- `SUBSTANCE_OF_TIME.md`
- `TEMPORAL_SCHEMA_V01.md`
- `CHRONOSENSE_AND_IDENTITY.md`
- `COMMITMENTS_AND_DEADLINES.md`
- `TEMPORAL_CAUSALITY_AND_EXPLANATION.md`

This document formalizes the temporal query surface needed to move from:
- trace as ordered artifact output

to:
- trace and memory as time-aware retrieval substrates

It focuses on the bounded query layer, not the full chronosense theory.

---

## Why This Matters

Without temporal query support, a system may store ordered events but still be unable to
reason well about:
- what happened before a given event
- what changed over an interval
- what is stale
- what commitments remain open
- how a current state emerged

Temporal retrieval is therefore not a convenience layer.
It is part of the substrate required for continuity, explanation, accountability, and
future chronosense work.

---

## Core Principle

> A cognitive record is not fully usable until the system can query it as a temporal structure rather than a flat sequence of artifacts.

ADL should treat time-aware retrieval as a first-class capability of the record layer.

---

## Scope

This document defines:
- temporal query primitives
- retrieval semantics over time-anchored records
- staleness-oriented query behavior
- baseline indexing expectations

This document does not define:
- the full chronosense model
- full distributed clock synchronization
- full causal reasoning over time
- counterfactual branch semantics

Those belong in downstream feature docs.

---

## Query Primitives

The temporal query layer should support at least the following classes of questions.

### 1. Relative-order queries

Examples:
- what happened before `X`?
- what happened after `Y`?
- what happened immediately prior to this failure?

These depend on ordered event or record relationships, not only wall-clock timestamps.

### 2. Interval queries

Examples:
- what changed between `T1` and `T2`?
- what decisions were made during this run window?
- which artifacts were updated in this interval?

### 3. Staleness queries

Examples:
- what is stale?
- which observations are older than the current decision horizon?
- which memory items should be downweighted due to age or inactivity?

### 4. Continuity queries

Examples:
- where did this continuity chain last remain valid?
- what interruption boundaries exist in this run history?
- which state transitions threaten continuity?

### 5. Commitment-state queries

Examples:
- what obligations remain open?
- which deadline is approaching?
- which commitments were missed during this interval?

These queries may consume commitment surfaces defined elsewhere, but they belong in the
 temporal retrieval layer once such records exist.

---

## Temporal Anchors

Temporal retrieval should operate over explicit temporal anchors rather than ad hoc string
 matching.

Relevant anchors may include:
- `t_created`
- `t_observed`
- `t_effective`
- monotonic event order
- run-local sequence order
- continuity-chain identifiers

The exact schema fields may evolve, but the query model should assume that:
- multiple time notions coexist
- wall-clock time is not sufficient by itself
- event order and continuity order matter separately

---

## Retrieval Semantics

Temporal retrieval in ADL should follow these rules.

### 1. Order is not the same as importance

Recent items are often useful, but recency alone must not erase:
- commitments
- still-valid facts
- unresolved failures
- trust-relevant history

### 2. Time must be queryable, not only displayed

Timestamps in logs are not enough.
The system must be able to ask structured questions over time-aware records.

### 3. Multiple time views may be valid

The system may need to distinguish:
- wall-clock time
- event order
- subjective or continuity-relevant time

### 4. Retrieval should expose gaps honestly

If timestamps are missing, inconsistent, or ambiguous, the query layer should expose that
 uncertainty rather than silently inventing precision.

### 5. Staleness is contextual

An old item is not automatically useless.
Staleness should depend on:
- age
- task context
- change rate
- whether the item represents a durable commitment or invariant

---

## Indexing Expectations

The implementation should be able to support at least:
- lookup by time anchor
- lookup by interval
- ordering by monotonic sequence
- filtering by continuity-relevant boundaries
- retrieval of neighboring records around a focal event

The storage mechanism may vary, but the query surface should not depend on manual log
 scanning.

---

## Staleness And Aging

The temporal query layer should support staleness-aware retrieval.

Potential staleness considerations:
- time since last confirmation
- time since last use
- time since last state change
- change sensitivity of the underlying fact
- whether the item is a commitment, observation, or derived belief

This document does not require a single staleness function yet.
It requires that staleness become an explicit queryable property rather than an implicit
 intuition.

---

## Continuity And Explanation

Temporal retrieval is a prerequisite for continuity-preserving reasoning.

If the system cannot retrieve:
- what immediately preceded an interruption
- where continuity became uncertain
- what state changed between checkpoints

then continuity validation becomes theater rather than architecture.

Temporal retrieval is also necessary for explanation surfaces such as:
- why a decision changed
- what sequence led to a refusal
- which earlier event caused a later correction

---

## Design Constraints

- must work with bounded local runtime records first
- must not require distributed synchronization in v1
- must not collapse event order into wall-clock order
- must preserve honest uncertainty when time metadata is incomplete
- must remain compatible with trace and memory as separate but related substrates

---

## Non-Goals

This document does not define:
- full chronosense semantics
- global temporal truth across hosts
- complete causality reasoning
- branch or counterfactual timeline logic
- final commitment schema

---

## Implementation Implications

This feature likely requires:
- explicit temporal fields in trace and memory records
- bounded query APIs or helper surfaces
- continuity-aware retrieval helpers
- staleness-aware ranking or filtering

Likely adjacent docs:
- `TEMPORAL_SCHEMA_V01.md`
- `SUBSTANCE_OF_TIME.md`
- `TRACE_SCHEMA_V1.md`
- `LOCAL_RUNTIME_RESILIENCE.md`

---

## Summary

Temporal query and retrieval make time operational inside ADL.

They allow the system to ask not only:
- what is stored?

but:
- what happened before this?
- what changed?
- what is stale?
- what remains open?
- where did continuity weaken?

> If ADL records time but cannot query time, then its temporal substrate remains incomplete.
