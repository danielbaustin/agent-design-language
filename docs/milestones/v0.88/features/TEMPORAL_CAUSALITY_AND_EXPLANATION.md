# Temporal Causality And Explanation

## Status

Promoted milestone feature doc for `v0.88`.

## Purpose

Define how ADL should represent and explain causal relationships over time, beyond simple
 event ordering.

Cluster role:
- this is the bounded causality/explanation doc for the likely `v0.88` temporal package
- it owns the distinction between order, dependency, and explanation
- it should not become the primary home for full temporal schema details or broad identity theory

Primary neighboring docs:
- `SUBSTANCE_OF_TIME.md`
- `TEMPORAL_SCHEMA_V01.md`
- `CHRONOSENSE_AND_IDENTITY.md`
- `TEMPORAL_QUERY_AND_RETRIEVAL.md`
- `COMMITMENTS_AND_DEADLINES.md`

---

## Why This Matters

Ordered records answer:
- what came first

But they do not automatically answer:
- what caused what
- which dependency mattered
- why a later correction happened

ADL needs bounded causal explanation surfaces for review, audit, and learning.

---

## Core Principle

> Sequence is not causality, but ADL should make causal claims only where the supporting structure is explicit.

---

## Scope

This document defines:
- sequence vs dependency
- causal explanation needs
- bounded causal-link representation

This document does not define:
- full causal inference theory
- probabilistic scientific causality engines

---

## Sequence Vs Dependency

Two events occurring in order does not by itself establish that one caused the other.

ADL should distinguish:
- temporal succession
- declared dependency
- inferred causal contribution
- unknown relationship

---

## Explanation Surfaces

ADL should eventually support questions like:
- why did this decision change?
- what earlier event triggered this refusal?
- which observation caused this correction?
- what dependency failure produced this interruption?

These are explanation questions, not just logging questions.

---

## Causal Links

Bounded causal representation may need to support:
- source event or condition
- target event or state
- relation type
- confidence or certainty class
- explanation note

The exact schema can be defined later.

---

## Design Constraints

- do not overclaim causality from mere order
- preserve uncertainty where causality is unclear
- allow explanation to cite dependencies and prior state changes

---

## Non-Goals

This document does not define:
- full scientific causal modeling
- global probabilistic reasoning graphs

---

## Adjacent Feature Docs

- `TEMPORAL_QUERY_AND_RETRIEVAL.md`
- `DECISION_SCHEMA.md`
- `TRACE_SCHEMA_V1.md`

---

## Summary

Temporal causality and explanation give ADL a bounded way to say not only what happened,
 but why later states emerged from earlier ones.

> A trustworthy temporal substrate should distinguish order from explanation.
