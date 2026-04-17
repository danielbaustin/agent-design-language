# Timeline Forks And Counterfactuals

## Status

Draft

Intended placement: `v0.90`

This document now lives in `docs/milestones/v0.90/`.
It fits better with the later fork/join, identity, and counterfactual reasoning band
than with the core `v0.88` chronosense package.

## Purpose

Define how ADL should reason about hypothetical branches, alternative timelines, and
 counterfactual paths without confusing them with the canonical continuity record.

---

## Why This Matters

ADL will eventually need to support:
- hypothetical planning
- branch-local evaluation
- replay-based alternatives
- reasoning about what would have happened under different choices

These require explicit treatment of non-canonical timelines.

---

## Core Principle

> Counterfactual reasoning is useful only if ADL can distinguish hypothetical branches from the authoritative continuity chain.

---

## Scope

This document defines:
- canonical vs hypothetical timelines
- branch identity concepts
- counterfactual reasoning boundaries
- continuity risks introduced by forks

This document does not define:
- full branch execution engine
- merge semantics
- complete identity resolution across forks

---

## Canonical Vs Hypothetical

ADL should distinguish:
- the canonical continuity record
- hypothetical branches created for analysis
- replay branches used for evaluation
- alternative futures proposed but not enacted

The system must not silently treat a hypothetical branch as if it were the canonical past.

---

## Fork Semantics

A fork creates a branch point where multiple possible continuations exist.

Important questions include:
- what was the fork point?
- what assumptions define the branch?
- is the branch purely analytical or executable?
- how does the branch relate to agent identity?

---

## Counterfactual Reasoning

Counterfactual reasoning should support questions like:
- what if this action had been refused?
- what if another provider had been chosen?
- what if the interruption had been handled differently?

These should remain explicitly marked as non-canonical reasoning results.

---

## Identity Risks

Forking complicates identity because:
- one prior continuity chain may produce multiple futures
- multiple branches may claim relation to the same prior self
- later reasoning may confuse simulated continuation with actual continuation

This feature therefore depends on stronger identity and continuity foundations.

---

## Trace And Retrieval Implications

Forks should remain queryable as:
- branch-scoped records
- replay/evaluation artifacts
- clearly marked hypothetical trajectories

The query layer should be able to distinguish:
- actual history
- replay history
- hypothetical projections

---

## Design Constraints

- canonical history must remain authoritative
- hypothetical branches must be explicitly marked
- branch provenance must be preserved
- identity claims across forks must remain conservative

---

## Non-Goals

This document does not define:
- full branch merge logic
- distributed fork coordination
- final identity law for branching selves

---

## Adjacent Feature Docs

- `TEMPORAL_QUERY_AND_RETRIEVAL.md`
- `TEMPORAL_ACCOUNTABILITY.md`
- `ADL_IDENTITY_ARCHITECTURE.md`
- `CONTINUITY_VALIDATION.md`

---

## Summary

Timeline forks and counterfactuals allow ADL to explore alternatives without corrupting
 the canonical continuity record.

> Hypothetical branches are useful only if they remain visibly hypothetical.
