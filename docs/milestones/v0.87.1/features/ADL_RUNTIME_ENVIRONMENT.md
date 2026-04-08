# ADL Runtime Environment

## Core Concept

The ADL runtime environment is not merely an execution engine. It is the **environment in which agents exist**.

This environment provides the bounded substrate conditions later runtime layers depend on:

- runtime root and run-artifact roots
- causal structure (trace)
- persistence primitives
- execution context
- future hooks for richer temporal and identity layers

Agents are not merely executed within the runtime. They operate inside a persistent local substrate with explicit artifact roots and continuity primitives.

---

## Layer Boundaries (Owner Split)

To avoid overlap with adjacent documents, the ADL runtime environment is defined as the **substrate layer** only. It provides conditions, not policies or lifecycle decisions.

**Runtime Environment (this document) owns:**
- runtime root and environment bring-up contract
- run-artifact roots and layout
- causal substrate (trace emission and linkage)
- persistence primitives (checkpointing, storage surfaces)
- execution context (process/container/runtime host)
- bounded identity and temporal hooks needed by later layers

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

This shift matters because ADL is building toward continuity-bearing agents and needs explicit substrate contracts before richer lifecycle and Shepherd behavior can exist safely.

---

## v0.87.1 Runtime Contract

For `v0.87.1`, the runtime environment becomes concrete as one authoritative local contract:

- runtime root:
  - default `.adl/`
  - override via `ADL_RUNTIME_ROOT`

- run-artifact root:
  - default `.adl/runs/`
  - override via `ADL_RUNS_ROOT`

- runtime marker:
  - `.adl/runtime_environment.json`
  - records the active runtime root/runs-root mode without leaking absolute host paths

- per-run layout:
  - `.adl/runs/<run_id>/run.json`
  - `.adl/runs/<run_id>/steps.json`
  - `.adl/runs/<run_id>/logs/`
  - `.adl/runs/<run_id>/learning/`
  - `.adl/runs/<run_id>/control_path/`
  - `.adl/runs/<run_id>/meta/`

This milestone does **not** claim full chronosense, persistent identity, or full agency continuity. It establishes the bounded substrate primitives those later systems require.

---

## Birth and Presence

From the perspective of the implementation, the runtime is where bring-up happens, where artifacts are rooted, and where bounded execution state becomes inspectable.

---

## Implication for Design

Because the runtime is an environment:

- failures are exposed as **interruptions of cognition signals** (the runtime detects and surfaces them, but does not decide policy)
- recovery requires **continuity-preserving primitives** (provided by the runtime; strategies defined elsewhere)
- orchestration must respect **agency**, but enforcement and intervention are not owned by the runtime layer

This directly motivates:

- the lifecycle layer
- the Shepherd layer
- persistence and checkpointing
- later identity and chronosense systems

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
- defines the local runtime-root and run-artifact contract
- does not impose lifecycle or recovery policy

This separation ensures:
- no duplication of responsibility
- clear implementation boundaries
- composability of policies over a stable substrate

---

## Key Statement

The ADL runtime environment is:

> A bounded local substrate with explicit runtime roots, causal artifacts, and persistence primitives on which later cognitive-runtime layers can depend.

It is not just a container for computation. It is the runtime contract the rest of the milestone builds on.
