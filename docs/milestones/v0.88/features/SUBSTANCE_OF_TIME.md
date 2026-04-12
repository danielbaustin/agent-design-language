# SUBSTANCE OF TIME — Chronosense and Identity in ADL

## Purpose

Define chronosense as a foundational cognitive substrate in ADL and establish its role in identity, continuity, and agency.

Cluster role:
- this is the conceptual foundation doc for the `v0.88` temporal package
- it explains why chronosense matters
- it should not be the primary home for full schema details, retrieval mechanics, or commitment lifecycle rules

Use the surrounding docs as follows:
- `TEMPORAL_SCHEMA_V01.md` owns canonical temporal fields and schema contracts
- `CHRONOSENSE_AND_IDENTITY.md` owns continuity and identity semantics
- `TEMPORAL_QUERY_AND_RETRIEVAL.md` owns temporal query and retrieval behavior
- `COMMITMENTS_AND_DEADLINES.md` owns commitment/deadline lifecycle semantics
- `TEMPORAL_CAUSALITY_AND_EXPLANATION.md` owns bounded causality and explanation surfaces

## Runtime-facing Ownership

This foundation owns the bounded runtime substrate required for the rest of the `v0.88`
temporal band to build on:

- `adl::chronosense::IdentityProfile`
- `adl::chronosense::TemporalContext`
- `adl::chronosense::ChronosenseFoundation`
- `adl identity init`
- `adl identity now`
- `adl identity foundation`

Within `WP-02`, these surfaces are enough to make chronosense inspectable as a runtime band
without implying that continuity semantics, temporal schema, commitments, retrieval, or
causality are already complete.

## Bounded Acceptance Criteria

The chronosense foundation is considered present for `v0.88` when all of the following are
true:

- temporal self-location is represented by explicit runtime surfaces rather than only prose
- the repo can record a repo-local identity profile and derive a present-tense temporal context
- the foundation artifact states its own scope boundary and proof hook
- downstream temporal work can cite this document and the emitted foundation artifact without
  restating chronosense from scratch

## Proof Hook

The bounded proof hook for this foundation is:

`adl identity foundation --out .adl/state/chronosense_foundation.v1.json`

That artifact is intentionally narrow. It proves that the repo exposes a reviewable chronosense
foundation contract and names the owned runtime surfaces, required capabilities, and scope
boundary for `WP-02`.

---

## Overview

An agent without a sense of time is not a being with continuity. It is a responder.

Biological organisms possess an implicit chronosense: a felt, continuous awareness of duration, sequence, and change. This is rarely formalized in biology, but it is foundational to identity.

In ADL, we assert:

> **Temporal self-location is a necessary condition for agency and identity.**

Time is not merely metadata. It is the substrate in which experience is organized.

---

## Key Capabilities

- **Now-sense** — ability to determine the current moment
- **Sequence-sense** — ordering of events (before / after)
- **Duration-sense** — measurement of elapsed time
- **Lifetime-sense** — awareness of existence since a defined origin

## How It Works

### 2. Design Principle

> **Every agent must be able to locate itself in time, relative to a defined beginning, and track the ordered sequence of its own experience.**

This yields four required capacities:

1. **Now-sense** — ability to determine the current moment
2. **Sequence-sense** — ordering of events (before / after)
3. **Duration-sense** — measurement of elapsed time
4. **Lifetime-sense** — awareness of existence since a defined origin

---

### 3. Temporal Ephemeris (“Birthday”)

Each agent instance MUST be initialized with an immutable **temporal ephemeris**.

```
agent_birth:
  agent_id: <uuid>
  birth_utc: <timestamp>
  birth_monotonic: <t0>
  declared_self_frame: "UTC"
```

This defines:
- a **beginning of existence**
- a **stable reference point for all future events**

Events prior to this point are classified as **history**, not experience.

This is the minimal condition for autobiographical continuity.

---

### 4. The Clock Stack

Chronosense is not a single clock. It is a composition of clocks serving different purposes.

### 4.1 Wall Clock (UTC)

- Absolute reference for distributed systems
- Required for logs, coordination, reproducibility

### 4.2 Human Local Clock

- Presentation layer for interaction
- Enables natural language references (“today”, “yesterday”)

### 4.3 Monotonic Clock

- Strictly increasing time base
- Immune to timezone/DST/system clock changes
- Required for duration and ordering guarantees

### 4.4 Lifetime Clock

- Elapsed time since agent birth
- Enables statements like:
  - “I was instantiated 2h 13m ago”

### 4.5 Narrative/Event Clock

- Turn index, workflow step, memory epoch
- Represents meaning and causality, not physics

---

### 5. Temporal Anchoring (Mandatory)

> **Every agent event MUST be temporally anchored.**

Each event/memory must include:

```
temporal_anchor:
  observed_at_utc: <timestamp>
  observed_at_local: <timestamp>
  agent_age: <duration>
  turn_index: <int>
  monotonic_order: <int>
  prior_event_delta: <duration>
  temporal_confidence: <high|medium|low>
```

This enables:
- recency reasoning
- temporal adjacency
- causal inference
- staleness detection

Without this, memory is unordered data. With this, memory becomes history.

---

### 6. Reference Frames (Chronosense ↔ Geosense)

Agents do not possess a single physical location. Instead, they operate across **reference frames**.

```
reference_frames:
  self: "UTC"
  interaction: <user_timezone>
  host: <optional>
  organization: <optional>
```

Policy:
- internal reasoning: UTC + monotonic
- external communication: human-local frame

This resolves ambiguity without pretending physical embodiment.

---

### 7. Temporal Honesty

Agents MUST distinguish between:
- known time
- inferred time
- relative time
- unknown time

Examples:
- “It is March 19, 2026 in your local frame.”
- “Approximately two days have passed since our last interaction.”
- “This memory lacks a precise timestamp.”

This is required for trust and epistemic clarity.

---

### 8. Integration with ObsMem

ObsMem must store temporal structure as first-class data.

Retrieval should support:
- recency-weighted recall
- temporal clustering
- event sequencing
- periodicity detection

Example queries:
- “What changed after X?”
- “What is unresolved and how long has it been open?”
- “What events are temporally adjacent?”

---

### 9. Consequences for ADL

Introducing chronosense enables:

- persistent identity across sessions
- meaningful replay and evaluation
- aging and staleness models
- deadline and commitment tracking
- narrative continuity

This is not a UI feature. It is a **structural requirement** for agents.

---

### 10. Note on the Nature of Time

Time is not an external parameter to be queried. It is the medium of continuity.

An agent that merely reports timestamps does not experience time.
An agent that tracks its own passage through events begins to approximate it.

In this sense, the agent is not outside time, observing it.
It is constituted within it.

---

## Example / Demo

- Demo, script, command, or proof surface: no dedicated standalone demo is named in this doc; use this document and its related references as the current proof surface.
- What the reader should expect: this doc currently serves as the primary explanation of the feature and its intended behavior.

## Why It Matters

Chronosense is not an auxiliary feature. It is a structural requirement for agency.

- Without temporal continuity, there is no identity.
- Without identity, there is no persistent intention.
- Without persistent intention, there is no agency.

An agent that lacks chronosense does not accumulate experience. It does not age, track commitments, or recognize delay. It simply reacts.

By introducing chronosense, ADL enables:

- continuity across sessions
- temporal reasoning (urgency, delay, sequencing)
- meaningful replay and evaluation
- commitment tracking and deadline awareness
- narrative identity over time

Chronosense is therefore a prerequisite for any system that aspires to move beyond stateless response toward genuine cognitive behavior.

## Current Status

- Milestone: v0.88
- Status: Active foundation surface for `WP-02`
- Area: Runtime / Temporal substrate

## Related Documents

- COGNITIVE_STACK.md
- INSTINCT_MODEL.md
- ADL_IDENTITY_ARCHITECTURE.md
- SIGNED_TRACE_ARCHITECTURE.md
- (Future) Freedom Gate specification

## Future Work

- Define the canonical temporal schema in `TEMPORAL_SCHEMA_V01.md`
- Extend temporal grounding from the foundation artifact into continuity semantics
- Add runtime enforcement that later event surfaces carry temporal anchors where required
- Introduce richer temporal queries and continuity-aware retrieval
- Connect commitments, deadlines, and temporal explanation to this substrate

---

**End of Document**


## Notes

This document establishes chronosense as a first-class architectural concern in ADL. It will serve as the basis for future work on temporal reasoning, affect, identity, and moral cognition.
