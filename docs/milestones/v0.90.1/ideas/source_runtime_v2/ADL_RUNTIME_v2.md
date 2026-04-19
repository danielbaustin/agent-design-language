

# ADL Runtime v2 — Gödel Agent Land

## Status
Historical source draft - context only in the v0.90.1 tracked package

## Purpose

Define **ADL Runtime v2** as the transition from a job-scoped execution runtime to a **persistent cognitive spacetime manifold**.

This document captures the core ideas, invariants, and architectural direction for what we are calling:

> **Gödel Agent Land** — a long-lived, persistent world in which agents exist, act, sleep, migrate, and continue.

This is not a refinement of the existing runtime. It is a **phase shift**.

---

## Core Shift

### Runtime v1 (Current)

- runtime lifetime == workflow lifetime
- agents are ephemeral
- execution is primary
- memory is auxiliary
- identity is weak or absent

### Runtime v2

- runtime lifetime >> workflow lifetime
- agents are **persistent entities**
- execution is **episodic**, not foundational
- memory is **structural**
- identity is **first-class**

> **The runtime is no longer something that runs tasks.**  
> **It is something in which agents live.**

---

## Core Concept

### Gödel Agent Land

A **persistent cognitive spacetime manifold** with:

- ordered time
- shared state
- durable identity
- causal structure
- governed action
- migration capability

Agents are not spawned for tasks.

They are **citizens** of this world.

---

## Core Invariant

> **No agent of significance may exist only in RAM.**

If an agent matters, it must be reconstructable from:

- identity state
- temporal anchors
- memory
- commitments
- policy state
- recent causal trace

---

## Architectural Layers

### 1. Manifold (Persistent Substrate)

The world itself.

Contains:

- chronosense (timebase, ordering)
- shared state (ObsMem)
- identity registry
- policy / constitutional surfaces
- causal trace
- migration metadata

This is what sleeps, wakes, migrates, and persists.

---

### 2. World Kernel

Always-on core services.

Responsibilities:

- scheduling
- health monitoring
- persistence / checkpointing
- admission control
- trace integrity
- migration coordination
- recovery

Analogy: OS kernel

---

### 3. Resident Staff (Permanent Agents)

Long-lived system agents responsible for maintaining the world.

Examples:

- Security Agent
- Reasonableness / Constitutional Agent
- Freedom Gate Manager
- Runtime Manager
- Archivist / Chronicler
- Repair / Medic Agent
- Migration Marshal
- Threat Watcher
- Resource Economist

These are not optional helpers.
They are part of the **structure of the world**.

---

### 4. Citizens (Identity-Bearing Agents)

Persistent agents with continuity across time.

Properties:

- identity
- memory
- commitments
- policy constraints
- chronosense
- rights and duties

They may:

- sleep
- wake
- migrate
- act
- refuse
- learn (bounded)

---

### 5. Episodes (Execution Units)

Bounded activities occurring within the manifold.

Examples:

- tasks
- workflows
- reviews
- patrols
- repairs
- negotiations
- demos

Episodes are **temporary**.
The manifold and citizens are not.

---

## Lifecycle Model

### Agent States

- awake
- sleeping
- suspended
- degraded
- migrating

### Runtime States

- running
- quiescing
- sleeping
- sealed
- rehydrating

---

## Sleep / Migrate / Wake Protocol

This is a **first-class runtime capability**.

### 1. Sleep

- quiesce active episodes
- halt external side effects
- checkpoint:
  - memory
  - identity
  - commitments
  - pending work
- mark agents as dormant

### 2. Seal

Produce a **portable runtime snapshot**:

- manifold state
- citizen registry
- memory segments
- trace tail
- policy state
- capability envelope

### 3. Transfer

- move snapshot to new environment

### 4. Rehydrate

- restore manifold
- rebind clocks and capabilities
- validate integrity

### 5. Wake

- resume staff first
- then citizens
- then eligible episodes

---

## Citizenship Model (Draft)

Example structure:

```yaml
citizen:
  citizen_id: ga-004
  class: godel_agent
  birth_utc: ...
  manifold_id: csm-01
  status: awake
  role: researcher
  duties:
    - maintain_coherence
    - defend_runtime
  rights:
    - refuse_unconstitutional_action
  memory_namespace: ...
  capability_envelope: ...
```

---

## Freedom Gate (Runtime v2 Role)

The Freedom Gate evolves into a **constitutional layer**:

- governs action selection
- validates trajectories, not just actions
- enforces policy at the level of intent

It becomes:

- judiciary
- constraint system
- moral boundary

---

## Trace Evolution

Trace becomes **world trace**, not run trace.

Must capture:

- manifold lifecycle events
- citizen lifecycle events
- episode lifecycle events
- decisions (Freedom Gate)
- migrations
- repairs and interventions

---

## New Primitives

Runtime v2 introduces:

- manifold
- citizen
- resident staff
- episode
- sleep state
- sealed snapshot
- rehydration
- capability rebinding
- continuity proof

---

## Key Capabilities

### 1. Persistence
The system continues beyond any single task.

### 2. Continuity
Agents maintain identity and memory across time.

### 3. Migration
Runtime can eventually move across machines or clusters.

### 4. Defense
System can reject invalid actions and later support governed defensive
verification.

### 5. Governance
Actions are bounded by constitutional rules.

### 6. Replayability
World state can be reconstructed and inspected.

---

## Acceptance Test Ladder

These tests describe the Runtime v2 direction. They should not all be treated
as the first prototype's acceptance criteria.

### AT-1: Local Manifold Boot
Create persistent runtime with multiple agents.

### AT-2: Sleep / Wake
Suspend and resume without loss of continuity.

### AT-3: Migration
Defer cross-machine migration until after the local snapshot/rehydrate path is
proven. The first Runtime v2 prototype should restore on the same machine.

### AT-4: Failure Recovery
Recover from interruption without corruption.

### AT-5: Defensive Action
The kernel or Freedom Gate rejects one invalid action and records a violation
artifact. Full red/blue/purple defensive verification is later work.

### AT-6: Capability Rebinding
Defer full provider/capability rebinding until the v0.92 identity, memory, and
capability substrate. The first prototype only records capability envelopes.

---

## Relationship to CSM

Runtime v2 is the concrete instantiation of:

- shared cognitive spacetime
- persistent worldlines (agents)
- ordered causal execution
- governed evolution

---

## Relationship to Roadmap

This work defines a follow-on milestone band rather than a single extra work
package:

- `v0.90`: long-lived bounded cycles and continuity handles
- `v0.90.1`: Runtime v2 foundation prototype
- `v0.90.2`: Runtime v2 hardening, invariant tests, violation artifacts, and recovery polish
- `v0.91`: moral/emotional/polis governance expansion, including kindness, harm prevention, and defensive posture
- `v0.92`: deeper identity, memory, capability, migration, and cross-polis continuity semantics

Runtime v2 cuts across:

- identity
- trace
- execution
- governance
- security posture
- distributed runtime and migration

The runtime can be essentially useful by `v0.91`, but not complete in the
strong sense. By then the core persistent governed-world substrate should exist
and moral/emotional governance can begin to inhabit it. Deeper identity,
memory, capability rebinding, and migration remain `v0.92` work.

---

## Non-Goals (for now)

- full autonomous learning
- unbounded self-modification
- social governance systems
- complex multi-world synchronization

---

## Summary

Runtime v2 transforms ADL from:

- a deterministic execution engine

into:

- a persistent cognitive world

> **Agents no longer run. They live.**

---

## Next Steps

- formal manifold data model
- citizen lifecycle specification
- snapshot / seal format
- local snapshot / rehydrate protocol
- kernel services definition
- first working prototype
