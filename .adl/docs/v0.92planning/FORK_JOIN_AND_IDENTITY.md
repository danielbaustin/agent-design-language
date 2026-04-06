

# Fork, Join, and Identity

## Purpose

Define how identity behaves when an agent’s execution **branches (forks)** and later **reconverges (joins)**.

This document establishes rules for:
- identity continuity across parallel reasoning
- temporal coherence across branches
- reconciliation of divergent histories

---

## Core Problem

In a deterministic but concurrent system, an agent may:

- explore multiple hypotheses in parallel
- evaluate alternative strategies
- spawn sub-agents or branches

This creates a fundamental question:

> When an agent splits into multiple timelines, what is its identity?

---

## Definitions

### Fork

A **fork** occurs when a single agent state produces multiple concurrent branches of execution.

Each branch:
- shares a common history up to the fork point
- diverges in reasoning, actions, or decisions
- maintains its own temporal progression

### Join

A **join** occurs when multiple branches are:
- merged
- evaluated
- or collapsed into a single continuation

---

## Identity Model

### Pre-Fork Identity

Before a fork:

- identity is singular
- history is linear
- temporal continuity is unambiguous

### Post-Fork Identity

After a fork:

- identity becomes **branch-relative**
- each branch is a valid continuation of the same prior self
- branches MUST retain:
  - shared history
  - temporal anchors
  - causal lineage

Key principle:

> Forking does not create new identities. It creates multiple continuations of one identity.

---

## Temporal Structure of Forks

Forking introduces **parallel temporal trajectories**.

Requirements:

- each branch MUST maintain its own monotonic ordering
- each branch MUST preserve agent_age continuity
- fork point MUST be explicitly recorded in trace

Example:

```
fork_event:
  fork_id: <uuid>
  parent_span: <span_id>
  branches:
    - branch_id: A
    - branch_id: B
```

---

## Join Semantics

When branches reconverge, the system MUST define:

### 1. Selection

- choose one branch as canonical
- discard or archive others

### 2. Synthesis

- combine results from multiple branches
- preserve causal contributions

### 3. Evaluation

- compare branches
- select based on criteria (cost, correctness, coherence)

---

## Identity After Join

After a join:

- identity resumes as a single trajectory
- prior branches become **historical alternatives**
- continuity is preserved via trace

Key principle:

> The agent is not the branch—it is the continuity across branches.

---

## Continuity Requirements

Fork/join behavior MUST satisfy continuity validation:

- no loss of prior history
- no temporal discontinuity
- no reset of agent_age
- all branches traceable to origin

Violations include:

- orphan branches
- untracked merges
- identity resets during join

---

## Implications for ObsMem

ObsMem MUST support:

- storage of parallel branches
- linkage to fork points
- retrieval of alternative histories

Example queries:

- “What alternatives were explored?”
- “Why was branch A chosen over B?”
- “What was rejected and when?”

---

## Causal Reasoning

Fork/join enables explicit causal analysis:

- different decisions → different outcomes
- counterfactual reasoning becomes first-class

This strengthens:
- evaluation
- learning
- hypothesis testing

---

## Determinism and Replay

Fork/join behavior MUST be deterministic:

- same inputs → same branching structure
- same evaluation → same join outcome

Replay MUST reproduce:
- branch structure
- decision points
- final selection

---

## Relationship to Chronosense

Fork/join extends chronosense into **multi-path temporal reasoning**.

- time is no longer a single line
- it becomes a structured set of possible trajectories

Chronosense must therefore support:
- branching timelines
- comparative temporal analysis
- reconciliation of divergent durations

---

## Why It Matters

Without fork/join identity:

- parallel reasoning breaks identity
- evaluation becomes opaque
- replay becomes meaningless

With it:

- agents can explore safely
- decisions become explainable
- identity remains coherent

---

## Current Status

- Milestone: v0.87
- Status: Draft
- Area: Identity / Trace / ObsMem

---

## Notes

This document defines how identity persists under concurrency. It is foundational for:

- reasoning graphs
- hypothesis engines
- multi-agent coordination