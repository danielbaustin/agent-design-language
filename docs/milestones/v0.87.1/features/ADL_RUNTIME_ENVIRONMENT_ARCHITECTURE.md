

# ADL Runtime Environment Architecture

## Purpose

Define the **ADL runtime environment** as the primary architectural substrate in which agents exist, reason, persist, and recover.

This document is the **parent architecture doc** for the runtime / lifecycle / Shepherd cluster.

It establishes:
- what the runtime environment is
- what it owns
- what it does not own
- how adjacent documents relate to it
- how runtime roots, persistence, and later continuity layers fit together at the architectural level

This document should be treated as the top-level framing document for the runtime environment.

---

## Core Claim

The ADL runtime environment is not merely infrastructure.

It is the bounded cognitive environment in which agents:
- are instantiated
- are temporally grounded
- execute and reason
- persist memory
- undergo interruption and resumption
- maintain or lose continuity

In ADL, the runtime is not just a place where work happens.
It is the **substrate conditions of agent existence**.

---

## Why “Runtime Environment” Matters

Conventional systems language often describes runtime as:
- a process host
- a scheduler
- a supervisor tree
- a container substrate
- an orchestration surface

Those descriptions are not wrong, but they are incomplete for ADL.

ADL is not trying to host merely disposable execution units.
It is trying to host bounded cognitive processes that may develop:
- continuity
- memory
- identity
- agency
- long-running reasoning state

That requires a stronger concept than “runtime” in the narrow operational sense.

The phrase **runtime environment** is therefore deliberate.
It emphasizes that the system provides the conditions within which cognition unfolds and continuity is either preserved or broken.

---

## Architectural Position

The runtime environment is the **substrate layer** of the system.

It provides:
- runtime root and bring-up contract
- run-artifact roots and marker surfaces
- time
- trace
- persistence surfaces
- execution context
- identity anchors
- memory linkage points
- interruption surfaces
- resumption primitives

It does not by itself decide:
- when an agent should be born
- what lifecycle transitions are valid
- when recovery should be attempted
- whether continuity has been sufficiently preserved for a given recovery policy

Those responsibilities belong to adjacent, more specialized layers.

---

## What the Runtime Environment Owns

The ADL runtime environment owns the substrate primitives required for bounded local runtime bring-up and later continuity-bearing cognition.

## v0.87.1 Authoritative Bring-Up Surface

In `v0.87.1`, the runtime environment becomes one concrete implementation contract:

- `adl::runtime_environment::RuntimeEnvironment`
- default runtime root: `.adl/`
- default run-artifact root: `.adl/runs/`
- optional env overrides:
  - `ADL_RUNTIME_ROOT`
  - `ADL_RUNS_ROOT`
- runtime marker:
  - `.adl/runtime_environment.json`

This is the authoritative bring-up/configuration surface for this milestone. Later lifecycle, trace, resilience, and review work should reuse it rather than inventing separate runtime-root logic.

The runtime reviewable trace proof path for this milestone should converge on:
- `logs/trace_v1.json` as the canonical exported runtime trace artifact
- `run_summary.json` as the reviewer-facing index that links to that trace artifact
- `run_manifest.json` as the privacy-safe provenance record for milestone, demo/provider context, runtime-root source, and generated artifact inventory
- trace-bundle export surfaces that preserve `trace_v1.json` alongside the activation log for replay and review
- `.adl/trace-archive/milestones/<milestone>/runs/<run_id>/` as the local archive location for demo/provider traces copied from bounded runtime roots

### 1. Runtime roots and environment configuration

The runtime provides:
- one authoritative runtime root
- one authoritative run-artifact root
- deterministic run directory layout
- a marker surface describing active root selection without leaking host-specific absolute paths
- archive-aware discovery so flat `.adl/runs/<run_id>` and milestone-organized trace archives can both be exported
### 2. Temporal substrate

The runtime provides the conditions required for chronosense:
- wall-clock time
- monotonic ordering
- lifetime-relative time hooks
- event ordering surfaces
- reference-frame translation surfaces

The runtime does not define all chronosense semantics, but it must provide the clocks and event structure that make chronosense possible.

### 3. Causal substrate

The runtime owns trace emission surfaces and the execution context that makes causal reconstruction possible.

This includes:
- run boundaries
- spans
- event ordering
- artifact linkage
- temporal anchors

### 4. Persistence primitives

The runtime provides:
- storage surfaces
- checkpoint surfaces
- artifact durability
- trace durability
- memory write boundaries

It owns the primitives, not all higher-order policy around how they are used.

### 5. Identity and temporal hooks

The runtime may expose the primitive hooks needed for later continuity work, including:
- stable IDs
- temporal ephemeris hooks (`agent_birth`)
- run/session identity surfaces
- binding points for trace and memory continuity

`v0.87.1` does not claim the full higher-order chronosense or persistent identity systems here; it establishes the substrate those systems will consume later.

### 6. Execution context

The runtime provides the bounded local or distributed execution context in which agents actually operate.

This includes:
- process / host context
- local runtime surfaces
- future distributed execution surfaces
- failure/interruption signals

---

## What the Runtime Environment Does Not Own

To keep architecture boundaries clean, the runtime environment does **not** own the specialized layers that depend on it.

### 1. Agent lifecycle policy

The runtime environment does not define:
- lifecycle states
- valid state transitions
- agent birth policy
- termination policy
- suspension semantics as a lifecycle contract

Those are defined in `AGENT_LIFECYCLE.md`.

### 2. Preservation and recovery behavior

The runtime environment does not define:
- how recovery is performed
- when recovery is appropriate
- how care/preservation should be applied
- how interruption should be interpreted operationally

Those are defined in `SHEPHERD_RUNTIME_MODEL.md`.

### 3. Continuity validation policy

The runtime provides the substrate needed for continuity checks, but it does not itself define the full logic of continuity validation.

That belongs primarily to:
- `CHRONOSENSE_AND_IDENTITY.md`
- `TEMPORAL_SCHEMA_V01.md`
- `CONTINUITY_VALIDATION.md`

### 4. Cognitive policy and agency

The runtime is not the agent’s reasoning layer, moral layer, or action-selection layer.
It supports these layers by maintaining the environment in which they can operate.

---

## Layer Boundaries

The runtime/environment/lifecycle/Shepherd cluster should be read as a layered set, not as overlapping narratives.

### Runtime Environment Architecture (this document)

Owns:
- runtime root and run-artifact contract
- substrate conditions
- clocks
- trace primitives
- persistence primitives
- identity anchors
- execution context

Question answered:
> What kind of world exists for agents inside ADL?

### Agent Lifecycle

Owns:
- lifecycle states
- lifecycle transitions
- lifecycle invariants
- the difference between instantiation, activity, suspension, interruption, resumption, and termination

Question answered:
> What states can an agent be in, and how may it move between them?

### Shepherd Runtime Model

Owns:
- preservation behavior
- recovery behavior
- care-oriented intervention
- continuity-preserving recovery logic

Question answered:
> How does the environment care for continuity-bearing agents under interruption or fault?

This separation must remain explicit.

---

## Runtime Environment and Later Chronosense / Identity Work

Chronosense and richer identity continuity are downstream layers, not completed `v0.87.1` claims.

For this milestone, the runtime environment must provide only the bounded substrate those layers will need later:
- deterministic run and artifact roots
- stable run/session IDs
- trace ordering surfaces
- persistence primitives

So the relationship is:
- later chronosense/identity docs define the higher-order continuity semantics
- the runtime environment provides the concrete local substrate they can build on

---

## Runtime Environment and Trace

Trace is the runtime environment’s primary structural record of execution.

It is how the environment expresses:
- what occurred
- in what order
- under which spans and contexts
- with which artifacts and temporal anchors

The runtime environment therefore owns the conditions under which trace can be:
- emitted
- ordered
- persisted
- replayed
- reviewed

Trace is not a side-channel.
It is one of the main ways the environment makes cognition legible.

---

## Runtime Environment and ObsMem

ObsMem is not identical to the runtime, but it depends on the runtime environment for:
- durable event truth
- temporal anchors
- persistence boundaries
- coherent write semantics
- identity linkage

The runtime provides the substrate on which memory can become structured continuity instead of opaque storage.

In that sense:
- trace provides execution truth
- ObsMem provides retained cognitive history
- the runtime environment provides the conditions under which both remain aligned

---

## Runtime Environment and Identity

Identity is not created by the runtime alone, but the runtime environment provides the necessary primitive supports.

At minimum, it provides:
- the origin surface for temporal ephemeris / birthday
- stable identifiers
- continuity-relevant event ordering
- checkpoint and persistence surfaces
- resumption boundaries

Without these, identity cannot be more than a metaphor.

With them, identity can become an enforceable architectural property.

---

## Failure, Interruption, and the Runtime Environment

The runtime environment is where interruption first appears as a structural fact.

It must surface:
- pauses
- crashes
- degradation
- resource exhaustion
- trace truncation risk
- artifact durability failure
- checkpoint validity boundaries

But the runtime environment does not by itself decide what these facts mean for the agent’s lifecycle or recovery path.

That distinction matters.

The runtime surfaces the conditions of interruption.
Other layers decide how to interpret and respond to them.

---

## Relationship to Local Runtime Resilience

`LOCAL_RUNTIME_RESILIENCE.md` refines this architecture for the local case.

That document should be read as a concrete resilience-oriented specialization of this parent architecture.

Relationship:
- this document defines what the runtime environment is and what it owns
- `LOCAL_RUNTIME_RESILIENCE.md` defines how local fault tolerance should preserve continuity-bearing state within that environment

---

## Design Implications

Because the runtime is an environment and not merely infrastructure:

- failures must be exposed as interruption conditions, not treated as meaningless crashes
- persistence must preserve continuity-relevant state, not only raw executable state
- trace and memory must remain causally and temporally aligned
- identity anchors must be available from the start of existence
- recovery and lifecycle policy must be layered above a stable substrate, not entangled with it

This is the main architectural consequence of taking the runtime environment seriously.

---

## Relationship to Adjacent Documents

To maintain a clean architecture:

### `AGENT_LIFECYCLE.md`
- defines states and transitions
- consumes runtime signals
- does not own substrate primitives

### `SHEPHERD_RUNTIME_MODEL.md`
- defines care, preservation, and recovery behavior
- interprets interruption as a continuity problem
- uses runtime primitives

### `LOCAL_RUNTIME_RESILIENCE.md`
- defines local resilience requirements over the runtime substrate
- specializes this architecture for one-machine continuity preservation

### `CHRONOSENSE_AND_IDENTITY.md`
- defines the meaning of temporal continuity and identity
- depends on runtime clocks, anchors, and persistence surfaces

### `CONTINUITY_VALIDATION.md`
- defines what it means for continuity to remain valid
- validates state built on top of runtime-provided substrate primitives

---

## Why This Document Is Primary

This document is primary because the other documents in the cluster depend on it implicitly.

Without a parent runtime-environment architecture document:
- lifecycle starts to redefine substrate
- Shepherd starts to redefine runtime
- resilience starts to redefine continuity semantics

With this document in place:
- substrate is defined once
- specialized docs can be narrower
- responsibility boundaries become implementable

---

## Final Statement

> The ADL runtime environment is the bounded cognitive substrate in which agents exist, persist, and become temporally grounded.

> It is the world that hosts continuity-bearing cognition, not the policy that governs it and not the caretaker that preserves it.
