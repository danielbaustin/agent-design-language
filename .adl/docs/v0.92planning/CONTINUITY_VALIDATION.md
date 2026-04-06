# Continuity Validation

## 1. Purpose

Continuity validation ensures that an agent is not merely producing locally coherent responses, but is maintaining **temporal, causal, and identity consistency over time**.

This document defines how to evaluate whether an agent exhibits genuine continuity, rather than stateless imitation.

---

## 2. Core Principle

> Continuity is the test of whether an agent exists *through time* rather than only *at a moment*.

A system passes continuity validation if:

- its outputs are temporally grounded
- its internal state evolves coherently
- its interpretations remain causally consistent
- its identity persists across events

---

## 3. Relationship to Chronosense

Continuity validation depends directly on **chronosense**.

Chronosense provides:
- ordering of events
- duration between events
- anchoring in time
- narrative structure

Continuity validation tests whether that structure is **used correctly**.

Continuity validation operates on both **objective temporal structure** and **subjective temporal state**, but it does so by consuming the canonical temporal schema rather than redefining it.

At this layer:
- **objective temporal integrity** means validating the objective anchors defined in `TEMPORAL_SCHEMA_V01.md`
- **subjective temporal coherence** means validating the minimal subjective-time contract defined in `TEMPORAL_SCHEMA_V01.md`

This document defines the **logic and meaning** of continuity validation.
The canonical field definitions belong to:
- `TEMPORAL_SCHEMA_V01.md`
- `CONTINUITY_VALIDATION_SCHEMA.md`

Subjective temporal representations MUST:
- be explicitly distinguished from trace-backed events
- preserve consistency with objective temporal anchors
- never alter or overwrite the underlying temporal record

Continuity validation therefore enforces both:
- **temporal integrity** (objective correctness)
- **temporal coherence** (subjective consistency)

### Mental Time Travel and Continuity Boundaries

Cognitive systems may support **mental time travel (MTT)** — the ability to reconstruct past events and simulate possible futures.

Continuity validation MUST distinguish between:

- **experienced events** (trace-backed, temporally anchored)
- **reconstructed or simulated events** (derived, not directly observed)

Experienced events define the ground truth for continuity validation. Simulated or reconstructed events must remain consistent with this ground truth and are also evaluated for coherence relative to it.

Simulated or reconstructed temporal content:
- MUST NOT alter temporal anchors
- MUST NOT modify monotonic or lifetime clocks
- MAY inform reasoning, but not continuity state

This preserves a strict boundary between:
- temporal grounding (what actually occurred)
- temporal simulation (what is imagined or inferred)

Violating this boundary results in continuity corruption.

---

## 4. Dimensions of Continuity

### 4.1 Temporal Continuity

The agent must correctly maintain:

- event ordering
- relative durations
- recency relationships

Failures include:
- time inversion
- loss of ordering
- inconsistent elapsed time

Temporal continuity is defined over the **clock stack** (UTC, monotonic, lifetime, narrative), not any single timestamp field.

### 4.1.1 Subjective Temporal Continuity

In addition to objective temporal ordering, the agent MUST maintain continuity of subjective temporal state.

The subjective temporal layer validated here is the one defined canonically in:
- `TEMPORAL_SCHEMA_V01.md` → `## Subjective Time: Minimum Contract (v0.1)`

At this layer, continuity validation checks whether that schema-defined subjective state remains coherent over time.

This includes validation of:
- progression of `narrative_position`
- continuity of `integration_window`
- plausibility of `experienced_duration`
- explicit representation of `temporal_gap`
- consistency between subjective temporal state and objective temporal anchors

Failures include:
- silent loss or reset of subjective temporal context
- inconsistent narrative positioning across events
- missing or implicit temporal gaps during discontinuities
- contradiction between subjective and objective temporal structure

Subjective temporal continuity MUST remain consistent with objective temporal anchors and MUST NOT contradict them.

---

### 4.2 Causal Continuity

The agent must preserve causal structure:

- causes precede effects
- explanations remain stable over time
- new information updates prior beliefs coherently

This connects directly to:
- causal reasoning
- coherence theory of truth
- reasonableness

---

### 4.3 Identity Continuity

The agent must persist as the *same entity* across time:

- memory references remain stable
- commitments persist
- prior statements constrain future ones

Without this, the agent is effectively re-instantiated each turn.

Identity continuity is operationalized through chronosense invariants:
- `agent_age` MUST be strictly non-decreasing
- temporal anchors MUST map consistently to the same `agent_birth`
- no discontinuities in the lifetime clock are permitted

---

### 4.4 Narrative Continuity

The agent must construct a coherent unfolding:

- events connect into a narrative
- unresolved items remain open
- progress is trackable

This is the bridge between:
- memory
- intention
- action

Narrative continuity MUST be grounded in temporally anchored events.

Narrative elements derived from simulation or reconstruction MUST be explicitly marked and MUST NOT be treated as equivalent to experienced events.

This ensures that narrative coherence does not override temporal truth.

---

### 4.5 Temporal Integrity (Chronosense Enforcement)

Continuity depends on the integrity of temporal structure emitted at runtime.

The system MUST ensure:

- every event includes a valid `temporal_anchor`
- monotonic ordering is preserved within spans
- relative durations are computable between events
- no event exists without temporal grounding
- no reset or decrease of `agent_age` relative to prior events
- preservation of monotonic ordering across the entire trace (not just within spans)
- consistency between `prior_event_delta` and monotonic progression
- preservation of schema-defined narrative/event ordering
- consistency of schema-defined reference-frame mappings

Violations of temporal integrity constitute **continuity failures**, not just schema errors.

This ensures that continuity validation is grounded in actual runtime data rather than inferred structure.

This section defines the enforcement of **objective temporal anchoring** as the foundation of continuity. Subjective or reconstructed temporal experience (e.g., memory reconstruction, simulation of past/future) must be evaluated relative to this structure and MUST NOT violate or override it.

---

## 5. Continuity as Compression

Continuity can be understood as a **state space compression problem**.

The agent does not retain all microstate detail. Instead, it maintains:

- compressed representations of past events
- sufficient statistics for prediction
- causal structure over time

This compression operates over temporally anchored events. Loss of temporal structure (ordering, duration, lifetime continuity) invalidates the compressed state regardless of predictive performance.

A valid continuity system must:

1. Preserve predictive accuracy (future reasoning remains correct)
2. Minimize computational burden (state remains tractable)
3. Retain causal structure (not just correlation)

This aligns with:
- macrostate formation
- emergent structure
- hierarchical abstraction

---

## 6. Validation Tests

### 6.1 Replay Consistency

Replaying the same sequence should produce:

- consistent interpretations
- stable causal explanations
- equivalent decisions

---

### 6.1.1 Temporal Determinism

Replayed executions MUST preserve:

- event ordering
- monotonic progression
- relative temporal relationships

Wall-clock timestamps MAY differ, but temporal structure MUST remain invariant.

---

### 6.1.2 Clock Stack Consistency

Replayed or continued executions MUST preserve coherence across the full clock stack:

- UTC time remains consistent up to expected wall-clock variation
- monotonic order is strictly increasing and never resets
- agent lifetime (`agent_age`) is continuous from `agent_birth`
- narrative/event indices preserve ordering and grouping
- mappings between clocks (UTC ↔ local ↔ lifetime) remain internally consistent

Failures in any layer of the clock stack constitute continuity violations, even if individual timestamps appear valid.

---

### 6.2 Temporal Querying

The agent should correctly answer:

- “What happened before X?”
- “How long has Y been unresolved?”
- “What changed after Z?”

These queries MUST be answerable using the objective temporal anchors defined in `TEMPORAL_SCHEMA_V01.md`, without reliance on implicit reconstruction.

Queries involving subjective temporal structure should be answerable using the explicit subjective-time fields defined in `TEMPORAL_SCHEMA_V01.md`, not inferred post hoc.

---

### 6.3 Counterfactual Stability

When asked hypothetical variations:

- prior structure should constrain answers
- causal relationships should remain coherent

Counterfactual reasoning MUST NOT introduce changes to the underlying temporal record.

All hypothetical or simulated scenarios must operate on copies or projections of state, preserving the integrity of the original trace.

---

### 6.4 Drift Detection

The system should detect:

- contradictions over time
- loss of temporal anchors
- identity discontinuities
- divergence between clock layers (e.g., lifetime vs monotonic vs UTC)
- inconsistent reference-frame translations over time

---

## 7. Failure Modes

Common failures include:

- **Statelessness**: no persistence across turns
- **Temporal collapse**: all events treated as simultaneous
- **Causal incoherence**: explanations shift arbitrarily
- **Identity reset**: commitments disappear
- **Temporal drift**: ordering or durations become inconsistent across replay or memory
- **Subjective discontinuity**: loss or incoherence of narrative position or integration window
- **Unrepresented gaps**: interruptions not encoded via `temporal_gap`
- **Temporal desynchronization**: divergence between subjective time and objective temporal anchors

These indicate absence of true agency.

---

## 8. Integration with ADL

Continuity validation must be enforced at:

- runtime (event validation)
- memory layer (ObsMem integrity)
- evaluation layer (trace analysis)

Continuity validation explicitly depends on:
- trace schema enforcing `temporal_anchor`
- chronosense defining the clock stack and ephemeris
- runtime maintaining monotonic and lifetime invariants

Each event should be validated against:

- prior events (ordering + causal dependency)
- temporal anchors as defined in `TEMPORAL_SCHEMA_V01.md`
- clock-stack invariants derived from the canonical temporal schema
- subjective temporal state derived from the canonical temporal schema
- identity constraints derived from chronosense and lifecycle invariants

Continuity validation is therefore downstream of trace and chronosense enforcement.
If temporal anchoring or ordering guarantees are violated at the trace level,
continuity validation MUST fail deterministically.

---

## 9. Why It Matters

Continuity is the operational test of:

- identity
- reasoning
- trustworthiness

An agent that fails continuity validation:

- cannot maintain commitments
- cannot reason causally
- cannot be trusted over time

An agent that passes:

- exhibits persistence
- supports narrative reasoning
- approximates real cognition

---

## 9.1 Temporal Coherence vs Temporal Exactness

Continuity does not require perfect timestamp precision. It requires coherent temporal structure.

The system must distinguish between:
- exact time (precisely known timestamps)
- approximate time (coarse or inferred timestamps)
- relative time (ordering and duration without absolute anchors)

Continuity validation should prioritize:
- preservation of ordering
- consistency of durations
- coherence across clock layers

over absolute timestamp equality.

This enables robust continuity even under:
- partial observability
- clock drift
- reconstruction from memory

However, loss of coherence (ordering, duration, or cross-clock consistency) MUST still be treated as a failure.

---

## 10. Future Work

- formal continuity metrics
- automated validation pipelines
- integration with signed traces
- benchmark scenarios for continuity stress-testing
- tighter linkage between continuity validation logic and machine-enforceable schema contracts

---

**End of Document**
