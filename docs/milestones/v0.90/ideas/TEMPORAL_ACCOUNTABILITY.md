# Temporal Accountability

## Status

Draft

Intended placement: `v0.90`

This document now lives in `docs/milestones/v0.90/`.
It belongs with the later trust / identity / governance band rather than with
the bounded temporal substrate work delivered in `v0.88`.

## Purpose

Define how ADL should reason about responsibility, auditability, and accountability over
 time.

---

## Why This Matters

If ADL records commitments, decisions, and continuity-bearing history, it must eventually
 address:
- who is responsible for past actions
- how later interpretations relate to earlier decisions
- what accountability survives interruption, delegation, or replay

---

## Core Principle

> Accountability over time depends on preserving both the historical record and the boundaries under which that record should be interpreted.

---

## Scope

This document defines:
- responsibility over time
- audit interpretation concerns
- relation to signed history and continuity

This document does not define:
- legal policy
- social punishment systems
- full constitutional law for ADL societies

---

## Accountability Questions

ADL should eventually support bounded answers to questions like:
- who accepted this obligation?
- who made this decision?
- was the later state a continuation or a new start?
- can a prior action be reinterpreted in light of new evidence?

---

## Historical Interpretation

Temporal accountability should preserve the distinction between:
- what was known then
- what is known now
- what changed in between

This matters because fair review should not collapse historical context into hindsight.

---

## Signed History And Continuity

Accountability is stronger when the system can preserve:
- signed or authenticated records
- continuity boundaries
- delegation boundaries
- review and override history

Without these, the system can still log events, but accountability claims remain weaker.

---

## Design Constraints

- preserve context, not just verdicts
- distinguish continuation from replacement
- make delegation and override visible
- support later review without erasing earlier uncertainty

---

## Non-Goals

This document does not define:
- final trust scores
- citizenship law
- moral philosophy in full

---

## Adjacent Feature Docs

- `COMMITMENTS_AND_DEADLINES.md`
- `DECISION_SURFACES.md`
- `TIMELINE_FORKS_AND_COUNTERFACTUALS.md`
- `REPUTATION_AND_TRUST.md`

---

## Summary

Temporal accountability gives ADL a way to preserve responsibility through time without
 flattening history into raw event sequence alone.

> A system that remembers actions but not accountability context remains incomplete.
