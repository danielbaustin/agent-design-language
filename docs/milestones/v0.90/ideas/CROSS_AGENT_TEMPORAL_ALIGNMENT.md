# Cross-Agent Temporal Alignment

## Status

Draft

Intended placement: `v0.90`

This document now lives in `docs/milestones/v0.90/`.
It depends on shared-world and multi-agent temporal semantics that go beyond
the bounded single-agent chronosense package delivered in `v0.88`.

## Purpose

Define how multiple agents should align temporal records well enough to support shared
 cognitive spacetime, reconciliation, and multi-agent continuity reasoning.

---

## Why This Matters

Single-agent continuity is only the beginning.
If ADL supports multiple agents acting in a shared world, it must eventually answer:
- whose event happened first?
- which events are concurrent or incomparable?
- how do we reconcile partial observation and drift?

---

## Core Principle

> Shared cognitive spacetime requires explicit temporal alignment rules, not informal assumptions that every agent shares one perfect clock.

---

## Scope

This document defines:
- alignment needs across agents
- reconciliation concerns
- drift and partial-observation concerns

This document does not define:
- a final distributed clock protocol
- cross-host consensus system

---

## Alignment Problems

Cross-agent temporal alignment must eventually address:
- differing local clocks
- partial observability
- delayed trace arrival
- conflicting event reports
- uncertain ordering across agents

---

## Reconciliation Needs

The system may need to determine:
- whether two agents observed the same event
- whether records conflict
- whether one record supersedes another
- whether ordering is known, unknown, or disputed

---

## Shared-World Implications

Without temporal alignment:
- reputation can drift on inconsistent evidence
- delegation review becomes harder
- shared memory can become incoherent
- multi-agent narratives can diverge silently

---

## Design Constraints

- preserve uncertainty where alignment is incomplete
- do not force false total ordering
- support bounded local-first implementations before distributed maturity

---

## Non-Goals

This document does not define:
- full distributed systems guarantees
- final consensus algorithms

---

## Adjacent Feature Docs

- `TEMPORAL_QUERY_AND_RETRIEVAL.md`
- `TIMELINE_FORKS_AND_COUNTERFACTUALS.md`
- `REPUTATION_AND_TRUST.md`
- `SHARED_SOCIAL_MEMORY.md`

---

## Summary

Cross-agent temporal alignment is the feature surface that lets multiple agents inhabit a
 shared temporal world without pretending their records always arrive in perfect order.

> Shared memory without temporal alignment risks becoming shared confusion.
