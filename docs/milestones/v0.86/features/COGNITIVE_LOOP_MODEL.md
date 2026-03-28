

# COGNITIVE_LOOP_MODEL.md

## Status

Tracked feature doc — v0.86

---

## Purpose

Define the **authoritative cognitive loop** for ADL v0.86.

This document provides a single, unified view of how Instinct, Affect, Arbitration, Freedom Gate, AEE (execution), Evaluation, Reframing, and Memory interact.

All component docs must be consistent with this loop.

---

## Canonical Cognitive Loop

```
instinct → affect → arbitration → freedom_gate → execution (AEE)
        → evaluation → (reframing?) → memory (ObsMem) → affect
```

This loop is:

- bounded
- inspectable
- replayable
- policy-governed

---

## Component Roles

### 1. Instinct

- Provides **fast priors** and default behavioral tendencies
- Shapes initial candidate actions and priorities
- Does not override policy or arbitration

Examples:

- completion pressure
- curiosity toward anomalies
- coherence maintenance
- integrity / safety bias

---

### 2. Affect

- Provides **weighting signals** for urgency, salience, and persistence
- Modulates:
  - arbitration decisions
  - AEE persistence / retry behavior
  - reframing pressure

Affect is **bounded and non-anthropomorphic**.

---

### 3. Cognitive Arbitration

- Selects routing mode:
  - fast
  - slow
  - hybrid
  - defer
  - refuse
- Consumes:
  - confidence
  - risk
  - cost
  - affect signals
  - **frame_adequacy_score**
- May emit:
  - **reframing_trigger**

---

### 4. Freedom Gate (Constitutional Layer)

- Enforces hard constraints before execution
- Overrides any prior signal (instinct, affect, arbitration)
- Ensures all actions remain policy-compliant

---

### 5. Execution (AEE)

- Performs bounded execution and convergence
- Iterates with:
  - critique
  - refinement
  - evaluation
- Emits:
  - progress signals
  - failure signals
  - **frame-related signals**

---

### 6. Evaluation

- Produces signals about execution outcome:
  - progress / delta
  - novelty
  - contradiction
  - oscillation

These signals feed:

- AEE continuation/termination
- arbitration feedback
- reframing detection

---

### 7. Reframing

Triggered when:

- **frame_adequacy_score is low**
- repeated non-progress occurs
- contradictions persist

Behavior:

- restate or restructure the task
- shift problem decomposition
- move from execution → diagnosis

Constraint:

- must remain bounded and inspectable

Note:

- Reframing is the system-level equivalent of **handling contradiction without collapse**

---

### 8. Memory (ObsMem)

Stores:

- outcomes
- failure patterns
- routing decisions
- **reframing history**

Feeds back into:

- affect weighting
- arbitration priors
- future execution strategies

---

## Frame Adequacy (Core Primitive)

```
frame_adequacy_score
```

Definition:

- A bounded estimate of whether the current problem framing is consistent and productive.

Low adequacy indicators:

- internal contradictions
- oscillating evaluation
- repeated non-progress
- persistent disagreement

Effects:

- may trigger `reframing_trigger`
- influences arbitration decisions
- informs AEE termination logic

---

## Reframing Trigger

```
reframing_trigger
```

Indicates:

- continued execution under current frame is low-value

Possible outcomes:

- task restatement
- decomposition change
- clarification request
- escalation under new frame

---

## AEE Termination Reasons

```
termination_reason:
  - success
  - bounded_failure
  - no_progress
  - reframed
  - policy_blocked
```

Requirements:

- must be explicit
- must be visible in artifacts
- must be deterministic or explainable

---

## Artifact Visibility Requirements

At minimum, the loop must emit artifacts showing:

- instinct inputs
- affect signals
- route_selected
- frame_adequacy_score
- reframing_trigger (if any)
- termination_reason

Artifacts must be:

- inspectable
- replayable
- auditable

---

## Design Principles

1. **Explicit over implicit**
   All major decisions must be visible.

2. **Bounded cognition**
   Execution, routing, and reframing must respect limits.

3. **Policy supremacy**
   Freedom Gate overrides all other signals.

4. **Frame awareness**
   The system must detect when it is solving the wrong problem.

5. **Deterministic or explainable behavior**
   Even stochastic elements must produce inspectable reasoning traces.

---

## v0.86 Scope

v0.86 does not require full generality.

It requires:

- one complete, working path through this loop
- bounded demonstrations of each stage
- artifact evidence for all key decisions

Integration is the goal.
