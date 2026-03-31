# Gödel–Hadamard–Bayes (GHB) Algorithm

## Overview

The Gödel–Hadamard–Bayes (GHB) algorithm is the **central cognitive loop** of ADL.

It defines how an agent:
- understands its situation
- generates new possibilities
- evaluates those possibilities
- improves over time

GHB is not merely a reasoning technique—it is a **discipline for controlled cognition**.

It exists to solve a fundamental problem in modern AI systems:

> How do we allow intelligence to explore, without allowing it to act beyond its authority?

The answer in ADL is:

> Separate *thinking* from *authorized action*, and bind both to a deterministic substrate.

GHB is the mechanism that makes that separation operational.

---

## The Core Insight

Most AI systems today collapse three distinct activities into one:

- interpretation
- creativity
- execution

This leads to systems that are:
- impressive
- useful
- but fundamentally unreliable and unbounded

GHB enforces a separation:

- **Gödel** → What is the current state of the system?
- **Hadamard** → What could be true instead?
- **Bayes** → What should we believe, given evidence and constraints?

This separation is the foundation of **controlled intelligence**.

---

## The Cognitive Loop

GHB operates as a structured loop:

1. **Gödel Phase (Analysis / Introspection)**
2. **Hadamard Phase (Hypothesis Generation)**
3. **Bayes Phase (Evaluation / Update)**

Each phase has a distinct role and must not be collapsed into the others.

### 1. Gödel Phase — Structured Awareness

The system constructs a formal representation of its current state:

- task definition
- prior outputs (trace / ObsMem)
- constraints and contracts
- known inconsistencies
- failure signals

This phase answers:

> “What is actually true right now?”

Outputs:
- structured problem representation
- identified gaps, contradictions, or uncertainties

This phase must be:
- deterministic in structure
- grounded in observable state
- externally inspectable

---

### 2. Hadamard Phase — Controlled Creativity

The system generates candidate hypotheses:

- alternative solution paths
- structural reorganizations
- corrective strategies
- reframings of the problem

This phase answers:

> “What could be true instead?”

This is the only phase where **non-deterministic generation** is allowed.

However, it is bounded by:
- task scope
- contract constraints
- prior state

Outputs:
- a set of candidate hypotheses (optionally ranked or structured)

This phase must be:
- creative but constrained
- expansive but scoped
- generative but accountable

---

### 3. Bayes Phase — Disciplined Judgment

Each hypothesis is evaluated against:

- likelihood of success
- consistency with constraints
- prior evidence and performance
- alignment with task objectives

This phase answers:

> “What should we believe and act on?”

Outputs:
- updated belief weights
- selected hypothesis (or top-k set)

This phase must be:
- comparative, not absolute
- evidence-driven
- constraint-aware

---

## The Deterministic Envelope

GHB does not operate in isolation.

It operates inside a **deterministic envelope** enforced by ADL:

- structured inputs and outputs (contracts)
- explicit schemas
- traceability of all steps
- bounded retries and iteration limits
- explicit handoff boundaries

This ensures that GHB is:

- replayable
- auditable
- inspectable
- composable

Without this envelope, GHB degenerates into uncontrolled generation.

---

## Constraint as First-Class Structure

The most important design principle of GHB is this:

> Constraint lives in the substrate, not in the temperament of the model.

The model is allowed to generate hypotheses freely.

But **no hypothesis is allowed to become action** unless it passes through constraint evaluation.

Constraints include:

- contracts (input/output schemas)
- milestone scope
- task-bundle boundaries
- invocation context (skill vs exploratory)
- system policies (e.g., Freedom Gate)

This creates a strict separation:

- **generation is permissive**
- **execution is controlled**

This is the core safety and reliability mechanism of ADL.

---

## Thinking vs Acting

GHB explicitly separates:

- thinking (internal hypothesis generation)
- acting (external state change)

Most current systems collapse these.

GHB enforces the boundary:

- Hadamard generates possibilities
- Bayes evaluates them
- but **execution requires external admission**

This prevents the failure mode:

> speculation → execution collapse

---

## Failure Modes (Observed)

Early experimentation reveals critical failure modes when GHB is not properly constrained.

### 1. Speculation → Execution Collapse

- exploratory idea is generated
- treated as instruction
- executed without authorization

### 2. Constraint Underspecification

- evaluation prioritizes plausibility over scope
- system produces high-quality but out-of-scope work

### 3. Missing Admission Gate

- no preflight or admission step
- execution occurs without issue context or milestone alignment

These are not edge cases.

They are **default behaviors** of unconstrained systems.

---

## Required Mitigations

To make GHB operationally safe:

- GHB must run **inside an admitted context** (issue, skill, or task)
- GHB outputs must pass **preflight or equivalent validation** before execution
- hypotheses must be explicitly classified as:
  - exploratory
  - planning
  - executable

The system should be able to say:

> “This appears to be exploratory. Do you want me to proceed?”

This is not optional—it is required for controlled cognition.

---

## Relationship to Replay (AEE)

GHB and replay (AEE) operate at different levels:

- Replay → persistence across attempts
- GHB → structured improvement within an attempt

Together:

- replay ensures the system does not give up
- GHB ensures the system improves intelligently

---

## Relationship to Skills

GHB is not a feature—it is a **pattern that can be embedded inside features**.

Within skills (e.g., `preflight-check`), GHB provides:

- structured diagnosis
- hypothesis-driven repair
- disciplined selection of actions

Future work should define a reusable **GHB execution pattern** within ADL.

---

## Architectural Position

GHB sits within a larger control architecture:

[Intent Classification]
        ↓
[Admission / Preflight]
        ↓
[GHB Loop]
        ↓
[Policy / Freedom Gate]
        ↓
[Execution]

This separation ensures:

- reasoning is powerful
- execution is controlled

---

## Open Questions (v0.86)

- How should GHB phases appear in trace output?
- Should hypothesis sets be persisted in ObsMem?
- How does affect modulate hypothesis selection?
- What are the correct bounds for iteration depth?
- How do we formally separate “speculation” from “authorized execution”?
- Where should the admission gate sit relative to the GHB loop?
- Should GHB emit an explicit `intent_classification` field?

---

## Status

- Conceptual model: defined
- Constraint model: partially defined (needs formalization)
- Failure modes: observed and documented
- Initial integration: experimental
- Full operationalization: planned across v0.86–v0.9

---

## Closing

GHB is not just a reasoning loop.

It is a proposal for how intelligence itself should be structured:

- aware of its state
- capable of creativity
- disciplined in judgment
- constrained in action

If implemented correctly, it enables something rare:

> Intelligence that is both powerful and trustworthy.
