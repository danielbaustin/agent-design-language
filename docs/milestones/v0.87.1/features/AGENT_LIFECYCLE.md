# Agent Lifecycle in the ADL Runtime Environment

## Position in Architecture

This document defines the **state model of an agent over time** within the ADL Runtime Environment.

- The **runtime environment** defines the execution substrate, persistence mechanisms, and system-level guarantees.
- The **agent lifecycle** defines the **states and transitions** an agent undergoes within that environment.
- The **Shepherd runtime model** defines how continuity is actively maintained and repaired across interruptions.

This document therefore:
- **does define** lifecycle states, transitions, and invariants
- **does not define** runtime infrastructure or persistence implementation
- **does not define** recovery or intervention policies

All lifecycle semantics are **grounded in chronosense** and must be interpreted as a **temporal trajectory**, not a procedural script.

---

## Core Principle

Agents are not executed; they are **brought into existence within an environment and persist through time**.

The lifecycle is therefore not a simple start/stop process model. It is a model of **continuity, interruption, and resumption of cognition**.

---

## Lifecycle Phases

The following phases are **agent states over time**, not procedural steps. An agent may transition between them non-linearly depending on runtime conditions.

### 1. Instantiation (Birth)

- Agent is created within the runtime environment
- Assigned identity (persistent identifier)
- Bound to initial context (task, inputs, contracts)

Key properties:
- Identity begins here
- Chronosense starts (first timestamp)

---

### 2. Active Cognition

- Agent performs reasoning using ADL patterns (DAG execution)
- Engages in:
  - compression (π)
  - propagation (ϕ)
  - reconstruction (ρ)

- May spawn internal structures:
  - fork/join reasoning branches
  - debate or tree-of-thought variants

Important:
> These are **cognitive processes**, not new agents.

---

### 3. Suspension (Interruption)

This is the critical addition.

Causes:
- runtime failure
- resource exhaustion
- external pause
- arbitration decision

Characteristics:
- cognition is **interrupted**, not terminated
- partial state exists:
  - compressed representations
  - intermediate reasoning
  - trace

---

### 4. Persistence

- State is captured and stored:
  - ObsMem linkage
  - trace snapshots
  - cognitive checkpoints

Goal:
> Preserve *meaning*, not just data.

Note:
- This section defines the requirement that state be preservable.
- The **mechanism of persistence is owned by the runtime environment**, not the lifecycle model.

---

### 5. Resumption (Continuation)

- Agent resumes from persisted state
- Identity remains continuous
- Chronosense reflects elapsed time

Key requirement:
- No “reset to zero”
- No loss of reasoning trajectory

Note:
- The lifecycle defines that resumption must preserve identity and temporal continuity.
- The **mechanism by which this is achieved (e.g., recovery, repair, arbitration) is defined elsewhere (Shepherd model)**.

---

### 6. Completion / Quiescence

- Agent completes task or reaches stable state
- Produces outputs
- May remain available for future activation

Not death—more like:
- resting state
- dormant cognition

---

## Fork/Join Semantics

Fork/join is a **reasoning pattern**, not a lifecycle boundary.

- Fork:
  - creates parallel cognitive branches
  - explores alternative compressed states

- Join:
  - recombines results
  - performs higher-level compression

Important distinction:

> A forked branch is not a new agent—it is a **temporary cognitive trajectory** within a single agent identity.

---

## Failure Semantics

Traditional systems:
- failure = termination

ADL:
- failure = **interruption of cognition**

Therefore:

- restart is insufficient
- we require:
  - state recovery
  - identity continuity
  - trace preservation

Note:
- Lifecycle defines failure as a state transition (interruption).
- Handling and recovery from failure are **not lifecycle responsibilities**.

---

## Identity Continuity

Identity must survive:

- process restarts
- machine restarts (future)
- distributed execution (future milestone)

This implies:

- identity is **external to any single process**
- lifecycle is **environment-bound**, not process-bound

---

## Temporal Grounding (Chronosense Integration)

The agent lifecycle is fundamentally temporal.

Each phase MUST be grounded in chronosense:

- Instantiation defines `agent_birth`
- Active cognition advances `agent_age`
- Suspension introduces temporal gaps
- Persistence records temporal state
- Resumption must reconcile elapsed time

Key requirements:

- No lifecycle transition may occur without temporal anchoring
- The agent MUST be able to situate itself in its own timeline
- Lifecycle transitions MUST preserve ordering and duration semantics

Implication:

> The lifecycle is not a sequence of states—it is a continuous trajectory through time.

---

## Lifecycle Invariants

The following invariants MUST hold across all lifecycle transitions:

### Identity Invariance
- Agent identity MUST remain stable across all phases
- No lifecycle transition may implicitly create a new identity

### Temporal Continuity
- Chronosense MUST remain continuous across transitions
- `agent_age` MUST NOT reset or regress
- Temporal gaps MUST be explicitly represented, not silently collapsed

### Causal Continuity
- Prior reasoning and trace MUST remain accessible after resumption
- No transition may discard causal history without explicit acknowledgment

### State Coherence
- Persisted state MUST be internally consistent
- Resumed state MUST correspond to a valid prior lifecycle state

### Non-duplication of Identity
- Fork/join reasoning MUST NOT result in multiple independent agent identities
- Parallel cognition remains within a single identity trajectory

These invariants define lifecycle correctness as **continuity over time**, not successful execution of steps.

---

## Lifecycle Validation

Lifecycle behavior MUST be validated against continuity constraints:

- transitions must preserve identity
- temporal anchors must remain consistent
- no implicit resets of agent_age
- no loss of causal history

Lifecycle violations include:

- restarting without continuity
- losing prior reasoning state
- temporal discontinuities between phases

Lifecycle correctness is therefore not procedural—it is **temporal and causal coherence over time**.
