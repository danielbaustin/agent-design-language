# Instinct Model v0.86

**Status:** Draft
**Version:** v0.86 planning
**Scope:** Cognitive architecture / agent substrate
**Related:** affect model, freedom gate, reasoning graphs, Gödel/Hadamard loop, ObsMem, adaptive execution

---

## Overview

This document proposes an **Instinct Layer** for the ADL cognitive architecture.

The instinct layer represents a set of **persistent internal drives** that influence reasoning, prioritization, persistence, anomaly response, and behavior selection.

Unlike externally supplied task goals, instincts are:

- persistent across tasks
- system-level rather than task-specific
- active in the background
- pre-cognitive or pre-deliberative influences on reasoning

The purpose of this layer is not to imitate human psychology for its own sake.
Its purpose is to provide a bounded engineering mechanism for **agency, persistence, anomaly sensitivity, and internal prioritization**.

The central hypothesis is:

> Effective long-running agents require some form of persistent internal drive structure.
> In ADL, this structure is modeled as an Instinct Layer.

---

## Motivation

Most current agent systems remain fundamentally reactive.

Their effective architecture is usually:

```text
external goal → planner → execution → output
```

Even when these systems appear autonomous, they remain dependent on externally provided intent and generally lack:

- persistent priorities
- internal motivational continuity
- principled persistence
- built-in anomaly pressure
- self-generated follow-through

As a result, many current "agents":

- stop too early
- fail to investigate contradictions
- abandon partially completed lines of work
- do not naturally re-prioritize around surprising findings
- remain tool-like rather than agent-like

ADL has already introduced several architectural components that move beyond purely reactive execution:

- Observational Memory (ObsMem)
- affect / emotion modeling
- reasoning graphs
- adaptive execution
- governance and freedom constraints

The instinct layer is proposed as the missing substrate that explains **what continuously drives the rest of the stack**.

---

## Instinct vs Goals vs Affect

These concepts should remain distinct.

### Goals

Goals are:

- explicit
- bounded
- task-specific
- often externally assigned

Examples:

- update a milestone doc
- run a validation flow
- review a card
- execute a bounded experiment

### Affect

Affect provides dynamic evaluation signals about current state, progress, or tension.

Examples:

- contradiction tension
- confidence increase
- curiosity activation
- completion frustration
- uncertainty salience

### Instinct

Instinct defines persistent background priorities that remain present across tasks and across local contexts.

Examples:

- maintain coherence
- preserve integrity
- investigate anomalies
- complete initiated work

A useful summary is:

| Layer | Function |
|---|---|
| Instinct | defines what matters persistently |
| Affect | signals how current state relates to those priorities |
| Goals | define the specific work currently being attempted |

In compact form:

```text
instinct → pressure
affect → evaluation
reasoning → action selection
```

---

## Why Instincts Matter

Without an instinct layer, the rest of the system remains under-motivated.

Reasoning alone does not explain:

- why a contradiction should matter
- why an anomaly should be investigated
- why unfinished work should create pressure
- why the system should preserve internal coherence over time

Instincts provide exactly this background pressure.

They are especially important for:

- long-running agents
- adaptive execution
- experiment selection
- structured review loops
- self-improvement architectures
- multi-step bounded autonomy

This may be the difference between:

- a reactive executor
- and a bounded autonomous agent

---

## Candidate Core Instincts

The initial ADL instinct set should remain small, interpretable, and auditable.

### 1. Integrity Instinct

**Purpose:** Preserve system integrity, artifact integrity, and policy compliance.

This instinct creates pressure toward:

- avoiding corruption of system artifacts
- preserving replay compatibility
- maintaining policy/governance boundaries
- preventing unsafe or invalid state transitions

This instinct aligns naturally with ADL’s emphasis on trust, auditability, and deterministic infrastructure.

---

### 2. Curiosity Instinct

**Purpose:** Increase sensitivity to missing explanations, anomalies, novelty, and unknowns.

This instinct creates pressure toward:

- investigating unexplained results
- exploring knowledge gaps
- seeking causal understanding
- generating bounded hypotheses for novel observations

Curiosity is especially relevant to:

- Gödel-style incompleteness response
- hypothesis generation
- adaptive execution
- exploratory reasoning

---

### 3. Coherence Instinct

**Purpose:** Maintain internal consistency in reasoning structures, claims, plans, and world-model fragments.

This instinct creates pressure toward:

- contradiction detection
- reconciliation of inconsistent claims
- repair of broken reasoning chains
- closure of unresolved dependencies

This instinct aligns strongly with:

- reasoning graph quality
- Popperian criticism
- structured review
- durable multi-step cognition

---

### 4. Completion Instinct

**Purpose:** Increase pressure to complete initiated work, especially where bounded execution has already begun.

This instinct creates pressure toward:

- following through on started tasks
- finishing experiment loops
- avoiding unnecessary abandonment
- closing validation and review cycles

This instinct provides a formal cognitive basis for the "sticktoitiveness" idea already discussed elsewhere in ADL planning.

---

## Optional Future Instincts

These should not be part of the initial minimal implementation, but may be worth future exploration.

### Social / Cooperative Instinct
Pressure toward coordination, contribution, and stable multi-agent interaction.

### Conservation Instinct
Pressure toward bounded resource usage and avoidance of wasteful exploration.

### Truth-Seeking / Epistemic Humility Instinct
Pressure toward distinguishing evidence from speculation and keeping uncertainty explicit.

These should be considered optional extensions, not initial scope.

---

## Architecture Placement

The instinct layer should not be treated as a replacement for reasoning.
It is a substrate that conditions reasoning.

A conceptual stack might be:

```text
policy / freedom constraints
↓
instinct layer
↓
affect model
↓
cognitive arbitration
↓
meta-reasoning
↓
reasoning graphs
↓
observational memory + retrieval
↓
adaptive execution engine
↓
actions / artifacts / experiments
```

This ordering is conceptual, not necessarily runtime-linear.

The key point is that instincts influence:

- salience
- prioritization
- persistence
- anomaly response
- action pressure

without being equivalent to explicit task goals.

---

## Instinct Signals

Instincts should expose bounded, interpretable signals.

Possible signal types include:

- `priority_bias`
- `attention_bias`
- `urgency_signal`
- `anomaly_signal`
- `persistence_pressure`
- `integrity_guard_signal`
- `coherence_tension`

These signals should be:

- inspectable
- bounded
- composable with the affect model
- usable in review/debug surfaces
- safe for deterministic workflows where applicable

The instinct layer should not become a vague source of hidden behavior.

```text
Example:

instinct: completion (high)
candidates:
  A: continue in-progress task
  B: explore new anomaly

result:
  A receives higher priority score due to completion pressure
```

---

## Interaction with Affect

Instinct and affect are related but distinct.

Instinct is persistent pressure.
Affect is dynamic evaluation of current state relative to pressure, context, and outcomes.

Example:

```text
instinct: coherence
event: contradiction detected
affect: contradiction tension rises
reasoning: prioritize contradiction resolution
```

Another example:

```text
instinct: completion
event: task stalls mid-flight
affect: completion frustration rises
reasoning: re-plan to finish bounded work
```

This separation helps preserve clarity in the cognitive architecture.

---

## Interaction with Goals and Planning

Goals remain externally or procedurally introduced units of work.

Instincts should not replace goals.
Instead, instincts should influence how goals are pursued, prioritized, and sustained.

Example:

```text
goal: review output card
coherence instinct: prioritize contradictions or ambiguous claims
completion instinct: favor full closure of review steps
integrity instinct: ensure no schema or policy violations
```

This improves agent effectiveness without requiring uncontrolled autonomy.

---

## Interaction with Cognitive Arbitration

Instinct signals may influence routing decisions in the Cognitive Arbitration Layer.

Examples:

- high integrity → bias toward slow-path or policy-gated execution
- high curiosity → bias toward escalation or exploratory reasoning
- high completion → bias toward finishing work via fast-path when safe

Instinct does not determine routing alone.

It provides **pressure signals** that are evaluated by the Cognitive Arbitration Layer alongside:

- uncertainty
- cost
- risk
- policy constraints

This preserves boundedness and prevents instinct from directly controlling execution strategy.

---

## Interaction with Gödel / Meta-Reasoning

The instinct layer has a particularly important relationship to Gödel-style incompleteness handling.

Gödel should not be reduced to a reasoning graph artifact.
Gödel belongs to the meta-reasoning realization that no formal representation is complete.

Instincts help operationalize that realization:

- curiosity responds to what is missing
- coherence responds to contradictions
- integrity responds to dangerous invalid states
- completion responds to unfinished loops

In this sense, instincts help transform incompleteness awareness into bounded cognitive action.

---

## Interaction with the Freedom Gate

The instinct layer must remain subordinate to governance.

A useful formula is:

```text
instinct → motivation
freedom gate → permission
reasoning → plan
execution → action
```

This is crucial.

Instincts may create pressure, but they must not override:

- policy limits
- safety rules
- deterministic workflow contracts
- explicit human governance

ADL is not aiming for unbounded autonomy.
It is aiming for **bounded, inspectable agency**.

---

## Why This Is Different from Reward Functions

Instinct should not be collapsed into reward maximization.

A reward function usually evaluates outcomes.
Instinct provides persistent pressure prior to outcome evaluation.

That distinction matters.

Reward asks:

- was this good?

Instinct asks:

- what continues to matter?
- what creates pressure before explicit evaluation?
- what keeps the system from becoming purely reactive?

This is one reason the instinct concept is often missing from current AI thinking.

---

## Minimal v0.86 Direction

For v0.86, the goal is not full implementation.

The goal is to establish:

- terminology
- architecture placement
- interaction boundaries
- a minimal instinct set
- a future-facing design direction

Recommended minimal scope:

1. define the instinct layer conceptually
2. define the four candidate instincts
3. distinguish instinct from affect and goals
4. connect instinct to agency and bounded autonomy
5. connect instinct to reasoning graphs, Gödel/meta-reasoning, and adaptive execution
6. explicitly keep the design bounded and reviewable

---

## Open Questions

The following should remain open design questions:

- Are instincts fixed, or can weights be adjusted over time?
- Should instinct signals be surfaced directly in artifacts?
- How should instinct interactions be represented in reasoning graphs, if at all?
- Can instinct pressure be reflected in scheduling/prioritization without compromising determinism?
- What is the minimum viable implementation that improves agent behavior without creating hidden complexity?
- How should instinct-driven persistence be bounded to avoid runaway loops?

---

## Summary

The Instinct Layer is proposed as a missing component in the ADL cognitive substrate.

It provides persistent internal drives that help explain:

- agency
- persistence
- anomaly sensitivity
- prioritization
- bounded autonomy

Together with memory, affect, reasoning, meta-reasoning, governance, and adaptive execution, it contributes to a more complete model of agent cognition.

The purpose is not anthropomorphic simulation.
The purpose is to identify the minimal mechanisms needed to turn capable but reactive systems into **bounded, inspectable, and effective agents**.