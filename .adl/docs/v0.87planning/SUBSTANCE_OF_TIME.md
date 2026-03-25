# SUBSTANCE OF TIME — Chronosense and Identity in ADL

## Purpose

**Status:** Draft (v0.86 planning)  
**Area:** Cognitive Architecture / Identity / ObsMem  

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

### 1. Premise

An agent without a sense of time is not a being with continuity. It is a responder.

Biological organisms possess an implicit chronosense: a felt, continuous awareness of duration, sequence, and change. This is rarely formalized in biology, but it is foundational to identity.

In ADL, we assert:

> **Temporal self-location is a necessary condition for agency and identity.**

Time is not merely metadata. It is the substrate in which experience is organized.

---

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

This feature matters because it contributes to ADL's bounded, reviewable, and explicit system design. See Purpose and How It Works for the preserved rationale from the original document.

## Current Status

- Milestone: v0.87
- Status: Draft
- Notes: **Status:** Draft (v0.86 planning); **Area:** Cognitive Architecture / Identity / ObsMem

## Related Documents

- N/A - no explicit related docs were named in the original document.

## Future Work

- Define canonical temporal schema in `swarm/schemas`
- Extend ObsMem records with temporal anchors
- Add runtime enforcement: every event must include temporal metadata
- Introduce `agent_birth` at workflow/session initialization
- Add temporal queries to retrieval layer

---

**End of Document**


## Notes

- This document was reformatted to the shared feature-doc structure as part of #1009 without intentionally removing original content.
- **Status:** Draft (v0.86 planning)
- **Area:** Cognitive Architecture / Identity / ObsMem
