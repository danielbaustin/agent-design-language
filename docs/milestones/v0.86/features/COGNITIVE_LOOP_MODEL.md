# COGNITIVE_LOOP_MODEL.md

## Status

Tracked feature doc — v0.86

---

## Purpose

Define the **authoritative cognitive loop** for ADL v0.86.

This document provides a single, unified view of how bounded signals, arbitration, Freedom Gate, execution, evaluation, reframing, and memory participate in the milestone loop.

All component docs must be consistent with this loop.

This doc also preserves richer future-facing context, but the bounded loop defined below is the normative `v0.86` implementation contract.

---

## Canonical Cognitive Loop

```
memory/context read
  → signal intake (instinct + affect)
  → arbitration
  → freedom_gate
  → execution (AEE-lite)
  → evaluation
  → optional reframing
  → memory write
  → terminate or begin next bounded pass
```

This loop is:

- bounded
- inspectable
- replayable
- policy-governed

### v0.86 Loop Interpretation

For `v0.86`, this means:

- memory participates at entry and exit, not as an unbounded background system
- instinct and affect are bounded signals, not full later-milestone subsystems
- arbitration emits explicit route decisions
- Freedom Gate emits explicit allow / defer / refuse commitment decisions
- execution is bounded (`AEE-lite`), not full convergence machinery
- evaluation may trigger bounded reframing
- termination conditions are explicit and artifact-visible

The implemented runtime execution surface for this milestone is:

- `bounded_execution.v1.json` records visible bounded execution iterations
- fast-path execution performs one direct bounded execution iteration
- slow-path execution performs a bounded review iteration followed by one execution iteration
- execution exits into an explicit provisional termination state for later evaluation

This milestone does not implement open-ended convergence, adaptive retry loops, or unbounded continuation.

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
  - bounded execution persistence / retry behavior
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
  - bounded frame-adequacy indicators
- May emit a bounded reframing trigger when continued execution is low-value

---

### 4. Freedom Gate (Constitutional Layer)

- Enforces hard constraints before execution
- Overrides any prior signal (instinct, affect, arbitration)
- Ensures all actions remain policy-compliant

For the implemented `v0.86` runtime surface:

- `freedom_gate.v1.json` records the bounded gate input, gate decision, reason code, and whether commitment was blocked
- `allow`, `defer`, and `refuse` remain the full bounded decision set
- commitment is blocked unless the gate decision is `allow`

---

### 5. Execution (AEE-lite)

- Performs bounded execution and limited convergence behavior
- Iterates with:
  - critique
  - refinement
  - evaluation
- Emits:
  - progress signals
  - failure signals
  - frame-related signals

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

- bounded frame adequacy is low
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

For the implemented `v0.86` runtime surface:

- `bounded_execution.v1.json` records the bounded execution handoff and iteration shape
- `evaluation_signals.v1.json` records evaluation signals and the explicit `termination_reason`
- `reframing.v1.json` records `frame_adequacy_score`, `reframing_trigger`, `reframing_reason`, and the bounded re-execution or termination choice
- termination is emitted as a bounded control output, not inferred from prose or hidden state

## Frame Adequacy and Reframing Notes

`v0.86` may use bounded frame-adequacy indicators and a bounded reframing trigger, but those should be treated as supporting control primitives, not as a separate full subsystem.

They exist to answer one milestone-critical question:

> should the system continue under the current frame, or is bounded reframing the higher-value action?

For `v0.86`, that judgment must remain:

- explicit
- inspectable
- bounded
- artifact-visible when it affects control flow

---

## Artifact Visibility Requirements

At minimum, the loop must emit artifacts showing:

- instinct inputs
- affect signals
- route_selected
- bounded frame-adequacy signal
- reframing trigger (if any)
- reframing reason and bounded re-execution or termination choice
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
