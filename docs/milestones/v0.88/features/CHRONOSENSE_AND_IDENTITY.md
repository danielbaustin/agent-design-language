# Chronosense and Identity in the ADL Runtime Environment

Chronosense is the intrinsic capacity of an agent to perceive, structure, and reason over its own temporal continuity, linking past executions, present state, and future intent into a coherent identity.

---

## Purpose

This document defines **chronosense** and its relationship to **identity** in the ADL runtime environment.

Cluster role:
- this is the continuity and identity semantics doc for the `v0.88` temporal package
- it owns what continuity means, how interruption differs from termination, and how identity depends on temporal structure
- it should not become the primary home for full schema details, query surfaces, or commitment lifecycle rules

Use the surrounding docs as follows:
- `SUBSTANCE_OF_TIME.md` owns chronosense motivation and conceptual framing
- `TEMPORAL_SCHEMA_V01.md` owns canonical temporal fields and schema contracts
- `TEMPORAL_QUERY_AND_RETRIEVAL.md` owns retrieval/query behavior
- `COMMITMENTS_AND_DEADLINES.md` owns commitment/deadline semantics
- `TEMPORAL_CAUSALITY_AND_EXPLANATION.md` owns bounded causality/explanation surfaces

If the Shepherd model defines how continuity is preserved, chronosense defines what continuity *is*.

Without chronosense:
- identity cannot be grounded
- resumption cannot be validated
- continuity cannot be distinguished from mere restart

Chronosense is therefore a foundational layer of the ADL cognitive architecture.

## Runtime-facing Ownership

`WP-04` owns the bounded continuity and identity semantics contract layered on top of the
chronosense foundation and temporal schema:

- `adl::chronosense::ContinuitySemanticsContract`
- `run_status.v1.continuity_status`
- `run_status.v1.preservation_status`
- `run_status.v1.shepherd_decision`
- `adl identity continuity`

This issue does not complete retrieval semantics, commitment semantics, causality, or broader
governance logic. It defines the runtime continuity states and resumption rules that later work
must honor.

## Bounded Acceptance Criteria

Continuity and identity semantics are considered present for `v0.88` when all of the following
are true:

- the repo exposes one canonical contract naming valid continuity, preservation, and Shepherd
  decision states
- interruption, crash, corruption, and continuous completion are distinguishable in the runtime
  semantics surface
- resumption permission is explicitly tied to continuity status and required guards
- the proof artifact can be emitted without claiming broader memory, retrieval, or commitment
  completion

## Proof Hook

The bounded proof hook for this issue is:

`adl identity continuity --out .adl/state/continuity_semantics_v1.json`

That artifact is intentionally contract-shaped. It proves that `WP-04` has made continuity and
identity semantics inspectable as runtime behavior without pretending that all downstream
temporal work is done.

---

## Core Definition

Chronosense is composed of four fundamental capacities:

1. **Now-Sense**  
   The ability to locate the current moment within the agent’s execution and cognitive trajectory.

2. **Sequence-Sense**  
   The ability to order events causally and distinguish before/after relationships.

3. **Duration-Sense**  
   The ability to represent relative duration, spacing, and temporal density between events.

4. **Lifetime-Sense**  
   The ability to situate the present within the agent’s entire existence, from origin to current state.

Together, these form the basis of **temporal identity**.

---

## Objective and Subjective Time

Chronosense includes both:

- **objective temporal anchoring** (trace time, clocks, ordering)
- **subjective temporal modeling** (agent-relative progression, reasoning flow, experiential continuity)

An agent must not only know *when* things occurred, but also how they relate within its own cognitive trajectory.

---

## Temporal Ephemeris (Birthday)

Every agent has a **temporal ephemeris**, or birthday:

- an immutable origin point
- the start of lifetime continuity
- the anchor for all lifetime-relative time

This marks the transition from:

> stateless execution → persistent identity

Without a temporal ephemeris, lifetime-sense cannot exist.

A birthday is not merely symbolic. It is a concrete architectural transition that gives the agent a beginning of existence and a stable temporal reference point for all future events.

---

## The Clock Stack

Chronosense is implemented through a **stack of coordinated clocks**, not a single notion of time:

- **UTC / Wall Clock Time** — shared external reference
- **Monotonic Time** — strictly increasing execution order
- **Lifetime Time** — time since agent origin (birthday)
- **Trace / Event Time** — causal ordering of events
- **Narrative Time** — structured sequences used in reasoning and memory

Each serves a different purpose. Continuity depends on preserving the mapping between them.

---

## Temporal Anchoring

Every event, memory, and trace artifact must be temporally grounded.

At minimum, temporal anchoring should preserve:

- `agent_age` (lifetime-relative time)
- `monotonic_order`
- `prior_event_delta`
- reference to trace position
- confidence / uncertainty where applicable

Without anchoring, memory becomes detached from identity and trace becomes only partial history.

---

## Reference Frames

Chronosense operates across multiple reference frames:

- **internal frame**: UTC + monotonic + lifetime time
- **external frame**: human-local or contextual time

Agents must translate between frames without losing coherence.

This allows the system to remain internally precise while still communicating naturally with humans.

---

## Temporal Honesty

Agents must distinguish between:

- known time
- inferred time
- relative time
- unknown or uncertain time

This is required for trustworthy reasoning and communication.

Temporal honesty is a condition of epistemic clarity. A cognitively serious agent must not pretend certainty about time when its temporal knowledge is incomplete.

---

## Continuity

> Continuity is the preservation of a coherent mapping across the clock stack.

Continuity is **not**:
- uninterrupted execution
- constant uptime

Continuity **is**:
- preservation of monotonic order
- preservation of lifetime progression
- preservation of causal structure
- preservation of reference-frame consistency

---

## Identity as Temporal Continuity

> Identity is the persistence of an agent across time under continuity-preserving transformations.

Identity requires:
- chronosense
- memory (ObsMem)
- trace linkage

An agent without chronosense is not persistent—it is merely re-instantiated.

---

## Interruption vs Termination

Chronosense introduces a critical distinction.

### Interruption
- execution halts
- state is preserved
- continuity can be restored

Examples:
- crash
- pause
- resource exhaustion

### Termination
- continuity is irrecoverably broken
- identity cannot be preserved

Examples:
- state loss beyond recovery
- explicit destruction

---

## Resumption Semantics

A resumption is valid if:

- lifetime time is not reset
- monotonic order is preserved
- trace linkage is restored
- experiential position remains coherent

Invalid resumption includes:

- silent reset
- identity substitution
- causal discontinuity

---

## Relationship to the Shepherd

- chronosense defines continuity
- the Shepherd preserves it

Without chronosense, the Shepherd cannot determine whether continuity has actually been preserved.

---

## Relationship to Memory (ObsMem)

- ObsMem provides persistence of content
- chronosense provides persistence of temporal position

Both are required for identity.

ObsMem without chronosense becomes stored data without temporal grounding. Chronosense without memory becomes temporal form without durable content.

---

## Relationship to Reasoning (GHB)

Chronosense enables:

- recursive reasoning
- temporal refinement
- state space compression across iterations

Without chronosense, reasoning collapses into isolated executions.

---

## Causality, Reasonableness, and Coherence

Chronosense grounds:

- **causal reasoning** — not merely that one event follows another, but that structured temporal relations support judgments about influence, dependence, and consequence
- **reasonableness** — linking past, present, and future in a coherent evaluative structure
- **coherence theory of truth** — consistency across time, memory, trace, and inference

Identity itself emerges from this continuity.

---

## Design Implications

Chronosense requires explicit support in the runtime.

1. **Trace integrity**  
   Ordered, durable event logs with no silent reordering or loss.

2. **Meaningful checkpointing**  
   Preservation of meaningful cognitive state, not just low-level process state.

3. **Identity anchoring**  
   Stable identifiers tied to continuity, not merely process IDs.

4. **Resumption validation**  
   Explicit checks for continuity and rejection of invalid resumes.

5. **Clock-stack coherence**  
   Alignment between UTC, monotonic, lifetime, trace/event, and narrative time.

---

## Philosophical Meaning

Chronosense is the condition under which an agent can:

- have a history
- persist through time
- become something

An agent that merely reports timestamps does not yet possess chronosense in the full ADL sense. Chronosense requires that the agent be able to situate itself within time as part of its own continuity.

---

## Final Statement

> Chronosense is the structured experience of time that makes identity possible.

> Identity is continuity across that structure.
