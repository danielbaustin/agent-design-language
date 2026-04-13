# Temporal Causality And Explanation

## Status

Promoted milestone feature doc for `v0.88`; bounded runtime surface implemented for WP-07.

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
- the reviewable `TemporalCausalityExplanationContract`
- the proof hook `adl identity causality --out .adl/state/temporal_causality_explanation_v1.json`

This document does not define:
- full causal inference theory
- probabilistic scientific causality engines
- planning or governance policy

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

The bounded causal surface now supports:
- source event or condition
- target event or state
- relation type
- confidence or certainty class
- explanation note

The current relation classes are:
- temporal succession
- declared dependency
- causal contribution
- unknown relationship

The current contract preserves the rule that sequence alone is insufficient evidence for causality.

---

## Design Constraints

- do not overclaim causality from mere order
- preserve uncertainty where causality is unclear
- allow explanation to cite dependencies and prior state changes
- require explanation records to cite bounded evidence rather than narrative speculation

## Runtime Surface

The current `v0.88` owned surface is:
- `adl::chronosense::TemporalCausalityExplanationContract`
- `adl identity causality`

The proof artifact is:
- `.adl/state/temporal_causality_explanation_v1.json`

The explanation surface requires:
- source event or condition
- target event or state
- relation type
- confidence
- explanation note

The contract keeps explicit uncertainty classes:
- high
- medium
- low
- unknown

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
