# Commitments And Deadlines

## Status

Promoted milestone feature doc for `v0.88`.

## Purpose

Define how ADL should represent future obligations, deadlines, and missed commitments as
 first-class cognitive and operational records.

Cluster role:
- this is the commitment/deadline semantics doc for the likely `v0.88` temporal package
- it owns future obligations, deadline states, persistence of commitments, and missed-commitment detection
- it should not become the primary home for generic chronosense motivation or full identity theory

Primary neighboring docs:
- `SUBSTANCE_OF_TIME.md`
- `TEMPORAL_SCHEMA_V01.md`
- `CHRONOSENSE_AND_IDENTITY.md`
- `TEMPORAL_QUERY_AND_RETRIEVAL.md`
- `TEMPORAL_CAUSALITY_AND_EXPLANATION.md`

This document formalizes the future-oriented side of the record layer:
- what has been promised
- what is due
- what remains open
- what has been missed

---

## Why This Matters

A system that records only what already happened cannot fully support:
- planning
- accountability
- continuity across sessions
- responsible delegation

ADL needs a bounded way to represent obligations that persist through time and can be
 checked later against actual behavior.

---

## Core Principle

> A commitment is not real in ADL unless it can persist, be queried, and later be evaluated against what actually happened.

---

## Scope

This document defines:
- commitment concepts
- deadline concepts
- lifecycle states
- persistence expectations
- missed-commitment detection semantics
- the bounded `v0.88` urgency slice that belongs with commitments/deadlines

This document does not define:
- the final schema format
- negotiation or acceptance protocol
- full scheduling/automation machinery

---

## Definitions

### Commitment

A commitment is a recorded obligation or intended action that the system, agent, or office
 has accepted as outstanding.

### Deadline

A deadline is a temporal boundary after which a commitment changes status if still unmet.

### Missed Commitment

A missed commitment is an obligation whose fulfillment conditions were not met before its
 valid deadline or review boundary.

---

## Commitment Lifecycle

Commitments should support at least these states:
- proposed
- accepted
- active
- fulfilled
- deferred
- canceled
- expired
- missed

The exact names may evolve, but the lifecycle must clearly distinguish:
- not yet accepted
- accepted and open
- intentionally deferred
- no longer valid
- failed to complete in time

---

## What A Commitment Must Capture

At minimum, a commitment record should preserve:
- the obligation or intended action
- who or what accepted it
- the applicable office or authority
- the time it was created
- any deadline or review window
- current status
- fulfillment conditions where applicable

If a commitment is changed, the record should preserve enough history to explain:
- what changed
- when
- why

---

## Persistence Across Sessions

Commitments matter precisely because they outlive a single execution slice.

ADL should therefore support commitments that:
- remain queryable across runs
- survive bounded interruption
- can be resumed, re-evaluated, or canceled honestly

If a system loses all memory of open commitments between sessions, it cannot claim serious
 continuity of conduct.

---

## Deadline Semantics

Deadlines may be expressed in more than one way:
- wall-clock deadlines
- event-count or stage deadlines
- review-gate deadlines
- continuity-relative deadlines

The system should not assume every commitment uses the same clock.

### Deadline Principle

> A deadline is meaningful only relative to an explicit temporal frame.

---

## Bounded v0.88 Urgency Slice

For `v0.88`, the accepted urgency contribution is deliberately narrow:

- temporal/deadline pressure
- obligation pressure from accepted open commitments
- explicit visibility when an obligation becomes overdue or missed

This is the part of the broader urgency/prioritization concept that belongs in the
chronosense package and was accepted through the bounded support issue `#1614`.

In repo truth, this bounded slice is delivered through `WP-06 / #1651`, which made the
commitment/deadline contract explicit and reviewable.

This feature does **not** claim:

- dynamic reprioritization during execution
- interruption/rescheduling machinery
- general scheduler policy
- broader governance-aware priority behavior

Those remain later-band work and should not be smuggled into `v0.88`.

---

## Missed-Commitment Detection

ADL should support explicit detection of:
- overdue active commitments
- commitments whose fulfillment conditions were not met
- commitments invalidated by interruption or context change

Missed commitments should not silently disappear into log history.

They should remain visible for:
- review
- accountability
- reputation and trust surfaces
- planning correction

---

## Relation To Trace And Retrieval

Commitments should be:
- queryable from temporal retrieval surfaces
- trace-linked to acceptance, deferral, fulfillment, and failure events
- available for continuity and accountability review

This allows the system to answer:
- what remains open?
- what was promised here?
- what became overdue?
- what was fulfilled late?

---

## Design Constraints

- commitments must persist across bounded runtime interruption
- deadline semantics must be explicit
- missed commitments must remain visible
- deferral must be distinguishable from silent neglect
- cancellation must be distinguishable from fulfillment

---

## Non-Goals

This document does not define:
- calendar integration
- automation scheduling
- social negotiation over commitments
- final trust/reputation interpretation

---

## Adjacent Feature Docs

- `TEMPORAL_QUERY_AND_RETRIEVAL.md`
- `DECISION_SURFACES.md`
- `DECISION_SCHEMA.md`
- `TEMPORAL_ACCOUNTABILITY.md`

---

## Runtime-facing Ownership

For `v0.88`, this feature owns the bounded commitment/deadline contract and proof surface.

Primary owned surfaces:
- `adl::chronosense::CommitmentDeadlineContract`
- `adl::chronosense::CommitmentLifecycleContract`
- `adl::chronosense::DeadlineSemanticsContract`
- `adl::chronosense::MissedCommitmentDetectionContract`
- `adl identity commitments`

This `v0.88` slice does not claim full scheduling or negotiation machinery.
It makes commitment and deadline semantics explicit and reviewable against the temporal
contracts already introduced in the earlier sprint work.

---

## Bounded Acceptance Criteria

`WP-06` is complete for `v0.88` when:
- the repo exposes a bounded, reviewable commitment/deadline contract
- commitment lifecycle states, open/terminal distinctions, and missed-state visibility are explicit
- deadline frames are named explicitly rather than implied as one universal clock
- persistence/history requirements are visible in the contract
- a proof hook emits the contract as a repo-local artifact
- tests cover the contract and proof-hook surface

This milestone does not require:
- calendar integration
- automation scheduling
- social negotiation over commitments
- full trust or reputation interpretation

---

## Proof Hook

- Command: `adl identity commitments --out .adl/state/commitment_deadline_semantics_v1.json`
- Output: `.adl/state/commitment_deadline_semantics_v1.json`

---

## Summary

Commitments and deadlines give ADL a way to represent the future as an inspectable
 obligation surface rather than a vague intention.

> If ADL cannot remember what it has committed to, it cannot honestly claim continuity of conduct.
