

# ADL Operational Skills Substrate

## Status
Draft

## Purpose

Define the **runtime execution substrate** for skills and skill compositions in ADL.

This document answers:
- how skills are executed
- how compositions are realized at runtime
- how trace is emitted
- how determinism is enforced
- where non-determinism is allowed

This is the bridge between:
- the **Skill Model** (what a skill is)
- the **Composition Model** (how skills combine)
- the **Trace System** (what actually happened)

---

## Core Principle

> ADL is a deterministic graph execution engine over stochastic nodes.

- **Graph** = composition structure
- **Nodes** = skill invocations
- **Edges** = explicit data flow
- **Stochasticity** = confined to model/tool execution inside a skill

The substrate enforces:
- structure
- boundaries
- trace

---

## Execution Model Overview

Execution proceeds as follows:

1. load composition graph
2. resolve skill definitions
3. validate inputs
4. execute graph (respecting primitives)
5. emit trace
6. return outputs

Each step must be:
- explicit
- observable
- replayable

---

## Execution Phases

The runtime executes compositions in explicit phases.

### 1. Plan Phase
- load composition graph
- resolve skill definitions
- validate structure (DAG, inputs, capabilities)

### 2. Bind Phase
- bind inputs to nodes
- resolve artifact references
- construct execution contexts

### 3. Schedule Phase
- determine execution order
- identify parallelizable nodes
- allocate execution slots

### 4. Execute Phase
- invoke skills
- handle tool/provider interactions
- apply retry/fallback policies

### 5. Commit Phase
- emit trace events
- persist artifacts
- finalize outputs

### Phase Principle

> Execution must proceed through explicit phases to ensure determinism, observability, and replayability.

---

## Execution Unit

The fundamental unit of execution is a **Skill Invocation**.

Each invocation includes:
- skill identity (name, version)
- input payload
- execution context
- allowed tools/capabilities

### Invocation Boundary Principle

> A skill invocation is the smallest reviewable and traceable execution unit.

---

## Execution Context

Each skill runs within a bounded execution context.

Context includes:
- inputs
- references to prior outputs
- allowed tools
- runtime metadata
- trace correlation identifiers

Context must be:
- explicit
- immutable (unless explicitly passed forward)

No hidden global state.

---

## Graph Execution Semantics
The runtime executes the composition DAG.

### Sequence
- execute node A
- pass output to B

### Parallel
- schedule nodes concurrently
- wait for completion
- merge outputs

### Branch
- evaluate condition
- select path

### Retry / Fallback
- apply policy
- record decision

### Merge Semantics

Parallel execution requires explicit merge behavior.

Merge strategies may include:
- aggregation
- selection
- adjudication

The merge strategy must be defined in the composition.

---

## Determinism Model

ADL enforces deterministic orchestration.

Deterministic:
- graph structure
- execution order (given same inputs)
- branching logic
- retry/fallback policy

Non-deterministic:
- model outputs
- tool responses

### Determinism Principle
> All non-determinism must be isolated within skill execution, never in orchestration.

### Deterministic Replay

Replay may operate in two modes:
- **strict replay** (reuse recorded outputs)
- **re-execution** (re-run skills under same structure)

The mode must be explicitly selected.

---

## Data Passing Model

Data flows explicitly between nodes.

Allowed forms:
- structured payloads
- artifact references (IDs)
- validated intermediate forms

Disallowed:
- hidden mutation
- implicit shared memory

### Data Integrity Principle
> Every data transition must be visible in trace.

### Artifact Model Integration

Data may be passed by reference using artifact IDs.

This enables:
- large data handling
- immutability guarantees
- efficient reuse

Artifacts must be:
- immutable
- versioned
- trace-linked

---

## Trace Emission Model

Trace is emitted at multiple levels.

### 1. Composition Start
- composition ID
- graph definition

### 2. Skill Invocation Start
- skill identity
- inputs
- context

### 3. Skill Invocation End
- outputs
- status (success/failure)
- termination reason

### 4. Decision Points
- branch decisions
- retry attempts
- fallback triggers
- adjudication results

### 5. Composition End
- final outputs
- overall status

### Trace Boundary Principle
> Every skill invocation and control decision must produce a trace boundary.

### Trace Fidelity

Trace must capture:
- exact execution order
- all intermediate states (where material)
- all decision rationale (where applicable)

> Trace is the ground truth of execution, not a summary.

---

## Error Model

Errors are first-class runtime events.

Types:
- validation error
- execution error
- timeout
- capability mismatch

Each error must:
- be captured in trace
- include structured metadata
- trigger defined policy (stop, retry, fallback)

---

## Tool Interaction Model

Tools are invoked within a skill, not by the composition engine.

Rules:
- tools must be declared in skill
- tool usage must be traceable
- tool outputs must be captured

### Tool Isolation Principle

> The runtime does not call tools directly—skills do.

This preserves the ADL boundary:
- agents → skills
- skills → tools/models

---

## Provider Interaction Model

Providers (LLMs, APIs) are accessed inside skills.

Rules:
- provider usage must be declared
- capabilities must be validated
- outputs must be captured in trace

### Provider Isolation Principle

> The runtime orchestrates, but does not reason—the provider does.

---

## Scheduling Model (v1)

Basic scheduling rules:
- sequence = ordered execution
- parallel = concurrent execution
- no speculative execution
- no dynamic graph mutation

Future versions may include:
- prioritized scheduling
- resource-aware execution
- distributed execution

### Scheduling Determinism

Even in parallel execution, scheduling decisions must be deterministic given identical inputs and environment.

---

## Replay Model

The substrate must support replay.

Replay requires:
- same composition graph
- same inputs
- recorded outputs (or controlled re-execution)

### Replay Principle
> A trace must be sufficient to reconstruct execution.

### Replay Constraints

Replay must not:
- introduce new side effects
- alter recorded outputs in strict mode
- violate original capability constraints

---

## Security Boundaries

The substrate enforces execution safety.

Key controls:
- tool access restricted per skill
- no implicit privilege escalation
- bounded execution
- explicit resource access

### Security Principle
> Capabilities must be explicitly granted, never assumed.

### Capability Enforcement

Each skill invocation must be checked against its declared capabilities before execution.

Unauthorized capability use must result in immediate failure.

---

## Composition ↔ Substrate Mapping

| Composition Concept | Runtime Behavior |
|--------------------|----------------|
| Node               | Skill invocation |
| Edge               | Data transfer |
| Sequence           | Ordered execution |
| Parallel           | Concurrent scheduling |
| Branch             | Conditional execution |
| Retry              | Re-invocation with policy |
| Adjudication       | Multi-input evaluation |

---

## Learning Integration

The substrate provides the foundation for learning.

Because:
- execution is structured
- trace is complete
- decisions are explicit

Future GHB systems can:
- analyze traces
- detect failure patterns
- propose improved compositions
- refine skills

But in v1:
- substrate is execution-only

### Learning Boundary Principle

> Learning systems may observe and propose changes, but must not alter execution semantics at runtime in v1.

---

## Design Implications

### 1. Keep orchestration simple and deterministic

### 2. Push intelligence into skills, not the runtime

### 3. Make every boundary observable

### 4. Avoid hidden state at all costs

### 5. Treat trace as a first-class output

---

## Non-Goals (v1)

This document does not define:
- distributed runtime architecture
- dynamic graph mutation
- autonomous planning
- long-lived background agents

These may come later.

---

## Summary
The ADL Operational Skills Substrate is the execution engine for skill compositions.

It is:
- graph-based
- phase-driven
- deterministic in structure
- bounded in execution
- explicit in data flow
- fully traceable

> The substrate does not think—it ensures that thinking is structured, observable, and reliable.

---

## Related Documents

- `SKILL_MODEL.md`
- `SKILL_COMPOSITION_MODEL.md`
- `ADL_LEARNING_MODEL.md`
- `TRACE_SCHEMA_V1.md`
- `TRACE_RUNTIME_EMISSION.md`
