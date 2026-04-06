# ADL Runtime Environment

## Core Concept

The ADL runtime environment is not merely an execution engine. It is the **environment in which agents exist**.

This environment provides the fundamental conditions required for cognition:

- time (chronosense)
- memory (ObsMem)
- identity continuity
- causal structure (trace)
- interaction with other agents

Agents are not simply executed within the runtime—they are **born into it, operate within it, and persist through it**.

---

## Layer Boundaries (Owner Split)

To avoid overlap with adjacent documents, the ADL runtime environment is defined as the **substrate layer** only. It provides conditions, not policies or lifecycle decisions.

**Runtime Environment (this document) owns:**
- temporal substrate (chronosense clocks and ordering)
- causal substrate (trace emission and linkage)
- persistence primitives (checkpointing, storage surfaces)
- identity anchoring primitives (IDs, ephemeris hooks)
- execution context (process/container/runtime host)

**Runtime Environment does NOT own:**
- agent lifecycle policy (creation, suspension, termination semantics)
- recovery strategy (how to resume, when to resume)
- continuity guarantees beyond providing primitives
- intervention logic or decision-making

Those responsibilities are defined in adjacent documents:

- **Agent Lifecycle** → defines *states and transitions* of an agent
- **Shepherd Runtime Model** → defines *care, preservation, and recovery behavior*

The runtime is therefore the **world**, not the **governor** or the **caretaker**.

---
## From Infrastructure to Environment

Traditional systems treat runtime as infrastructure:

- process manager
- scheduler
- resource allocator

ADL treats runtime as an **environment**:

> A structured, persistent, causal space in which cognition unfolds.

This shift is essential because ADL agents are not passive computations—they are **active cognitive entities with continuity over time**.

---

## Cognitive Spacetime (Practical Form)

The runtime environment is the first practical implementation of what we have described as a **cognitive spacetime manifold**.

In concrete terms, the runtime provides:

- **Temporal ordering**
  - every event is timestamped
  - agents experience ordered time (chronosense)

- **Causal traceability**
  - all actions are part of a trace
  - reasoning can be replayed and inspected

- **State persistence**
  - cognitive state can survive interruption
  - partial reasoning is preserved

- **Identity anchoring**
  - agents maintain continuity across runs
  - identity is not tied to a single process

---

## Birth and Presence

From the perspective of an agent:

- The runtime is where it is instantiated (birth)
- The runtime is where it perceives time
- The runtime is where it acts

This leads to a simple but powerful statement:

> The runtime environment is the world in which agents exist.

---

## Implication for Design

Because the runtime is an environment:

- failures are exposed as **interruptions of cognition signals** (the runtime detects and surfaces them, but does not decide policy)
- recovery requires **continuity-preserving primitives** (provided by the runtime; strategies defined elsewhere)
- orchestration must respect **agency**, but enforcement and intervention are not owned by the runtime layer

This directly motivates:

- the Shepherd model (care and continuity)
- persistence and checkpointing
- identity and chronosense
- distributed evolution (future milestones)

---

## Relationship to Adjacent Documents

To maintain a clean architecture:

### Agent Lifecycle (AGENT_LIFECYCLE.md)
- defines lifecycle states (active, suspended, interrupted, terminated)
- defines valid transitions and invariants
- consumes runtime signals but does not implement substrate

### Shepherd Runtime Model (SHEPHERD_RUNTIME_MODEL.md)
- defines preservation and recovery behavior
- interprets interruptions as continuity problems
- uses runtime primitives (trace, checkpoints, identity anchors)

### Runtime Environment (this doc)
- provides the substrate both of the above depend on
- does not impose lifecycle or recovery policy

This separation ensures:
- no duplication of responsibility
- clear implementation boundaries
- composability of policies over a stable substrate

---

## Key Statement

The ADL runtime environment is:

> A persistent, causal, time-aware environment in which cognitive agents exist, evolve, and maintain continuity.

It is not a container for computation—it is the **condition for cognition**.
