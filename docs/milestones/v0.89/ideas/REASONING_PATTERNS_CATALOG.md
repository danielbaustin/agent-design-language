


# Reasoning Patterns Catalog

## Purpose

Define canonical reasoning patterns that leverage ADL’s deterministic runtime, temporal grounding (chronosense), and identity continuity.

This document serves as:
- a catalog of reusable reasoning strategies
- a bridge between cognitive architecture and implementation
- a foundation for demos and authoring surfaces

---

## Design Principles

All reasoning patterns in ADL MUST:

- operate over trace + ObsMem (not hidden state)
- preserve identity and continuity
- be temporally grounded (chronosense-aware)
- be replayable and deterministic
- emit explicit decision events

Key principle:

> Reasoning is not hidden—it is a structured, inspectable process over time.

---

## Pattern 1: Linear Deliberation

### Description

A single-path reasoning process:
- think → decide → act → evaluate

### Properties

- no branching
- minimal cost
- fully sequential

### Use Cases

- simple queries
- low-ambiguity tasks

---

## Pattern 2: Fork–Evaluate–Join

### Description

The agent explores multiple alternatives in parallel, then selects or synthesizes a result.

### Structure

1. fork into multiple branches
2. evaluate each branch
3. join via selection or synthesis

### Properties

- parallel exploration
- explicit alternatives
- strong causal trace

### Use Cases

- ambiguous problems
- strategy selection
- creative tasks

---

## Pattern 3: Debate (Multi-Agent or Multi-Branch)

### Description

Multiple agents or branches argue competing positions.

### Structure

- generate competing hypotheses
- critique and counter-argue
- converge via evaluation

### Properties

- adversarial reasoning
- exposes hidden assumptions
- improves robustness

### Use Cases

- high-stakes decisions
- correctness-critical tasks

---

## Pattern 4: Iterative Refinement (Replay Loop)

### Description

The agent repeatedly improves a result using feedback.

### Structure

- generate initial output
- evaluate
- revise
- repeat until convergence

### Properties

- temporal progression of improvement
- explicit revision history
- leverages replay

### Use Cases

- code generation
- document drafting
- optimization problems

---

## Pattern 5: Hypothesis Tree

### Description

The agent builds a structured tree of hypotheses and evaluates them.

### Structure

- generate root hypotheses
- expand into sub-hypotheses
- evaluate nodes
- prune invalid branches

### Properties

- hierarchical reasoning
- explicit structure
- supports pruning

### Use Cases

- diagnosis
- planning
- scientific reasoning

---

## Pattern 6: Memory-Guided Reasoning

### Description

The agent uses ObsMem to inform reasoning.

### Structure

- retrieve relevant past events
- incorporate into current reasoning
- update memory

### Properties

- continuity-aware
- context-rich
- supports learning

### Use Cases

- long-running tasks
- personalization
- historical analysis

---

## Pattern 7: Temporal Reasoning

### Description

Reasoning explicitly over time.

### Capabilities

- ordering of events
- duration analysis
- recency and staleness
- deadline tracking

### Properties

- chronosense-dependent
- supports causal inference

### Use Cases

- scheduling
- monitoring
- commitment tracking

---

## Pattern Composition

Patterns may be combined:

- fork → debate → join
- linear → refinement loop
- hypothesis tree + memory retrieval

Composition MUST preserve:

- determinism
- temporal coherence
- identity continuity

---

## Trace Requirements

All patterns MUST emit:

- explicit decision points
- fork/join events where applicable
- temporal anchors
- artifact references

This ensures:

- replayability
- inspectability
- causal analysis

---

## Evaluation Criteria

Patterns should be evaluated based on:

- correctness
- cost (time, tokens, compute)
- coherence (temporal + causal)
- stability under replay

---

## Relationship to GHB

These patterns form the execution substrate for the GHB loop:

- Gödel → structure / hypothesis generation
- Hadamard → search / exploration (forking)
- Bayes → evaluation / selection (join)

The catalog defines *how* reasoning unfolds.
GHB defines *how it improves over time*.

---

## Why It Matters

Without explicit reasoning patterns:

- behavior is opaque
- results are inconsistent
- improvement is ad hoc

With them:

- reasoning becomes structured
- performance becomes tunable
- systems become explainable

---

## Current Status

- Milestone: v0.87
- Status: Draft
- Area: Reasoning / Cognitive Architecture

---

## Notes

This catalog will expand over time. Future additions may include:

- cost-bounded exploration
- uncertainty-aware reasoning
- distributed multi-agent patterns
- adaptive pattern selection