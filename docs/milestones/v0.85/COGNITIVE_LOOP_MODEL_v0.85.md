# Cognitive Loop Model — v0.85

## Status

Tracked authority for the v0.85 milestone doc set.

## Purpose

This document is the single authoritative cognitive-loop model for the tracked v0.85 milestone docs.

Other v0.85 docs may summarize or apply parts of the loop, but they should not define a competing authoritative loop model.

## Canonical Cognitive Loop

```text
instinct -> bounded affect -> cognitive arbitration -> freedom gate
        -> execution (AEE) -> evaluation -> reframing? -> memory (ObsMem)
        -> bounded affect
```

This loop is intended to be:
- bounded
- inspectable
- replay-compatible
- policy-governed

## Component Roles

### 1. Instinct

Instinct provides fast priors and candidate-action pressure.

It:
- creates persistent pressure toward coherence, completion, curiosity, or integrity
- influences what initially matters before slower reasoning runs
- does not override governance or policy

### 2. Bounded Affect

Bounded affect provides dynamic weighting signals for:
- urgency
- salience
- persistence
- contradiction pressure
- confidence shifts

It is not a human-emotion simulation layer. It is a bounded, inspectable reasoning-control layer.

### 3. Cognitive Arbitration

Cognitive arbitration selects routing mode and cognitive effort, including:
- fast
- slow
- hybrid
- defer
- refuse

It consumes evidence such as:
- confidence
- risk
- cost
- bounded affect signals
- frame adequacy

### 4. Freedom Gate

The freedom gate is the constitutional and policy boundary.

It:
- enforces hard constraints before execution
- can override instinct, affect, or arbitration outputs
- keeps agency bounded and reviewable

### 5. Execution (AEE)

Execution is performed by the Adaptive Execution Engine.

It:
- carries out bounded execution
- emits progress and failure signals
- provides material for evaluation and possible reframing

### 6. Evaluation

Evaluation produces signals about:
- progress
- novelty
- contradiction
- failure
- delta relative to expected outcomes

These signals inform both continued execution and later reframing.

### 7. Reframing

Reframing occurs when the current frame appears low-value or self-contradictory.

It may:
- restate the task
- change decomposition
- shift from execution to diagnosis

Reframing must remain bounded and inspectable.

### 8. Memory (ObsMem)

Memory stores:
- outcomes
- failure patterns
- routing decisions
- reframing history

Memory feeds back into bounded affect and future arbitration/execution choices.

## Design Rules

1. Policy supremacy
   - the freedom gate overrides other signals when necessary
2. Explicit over implicit
   - major loop decisions should be visible in artifacts where practical
3. Bounded cognition
   - execution, reframing, and adaptation remain constrained
4. Deterministic or explainable behavior
   - hidden nondeterministic control surfaces are out of scope
5. Artifact visibility
   - the loop should eventually be legible through emitted artifacts, not only prose

## Relationship To Other v0.85 Docs

- [AFFECT_MODEL_v0.85.md](AFFECT_MODEL_v0.85.md) defines the bounded affect subsystem in more detail.
- [AFFECTIVE_REASONING_MODEL.md](AFFECTIVE_REASONING_MODEL.md) defines the signal model attached to reasoning/evaluation surfaces.
- [DESIGN_v0.85.md](DESIGN_v0.85.md) describes how this loop fits the milestone architecture.
- [VISION_v0.85.md](VISION_v0.85.md) describes the architectural role of the cognitive substrate at milestone level.

## Scope Note

This is the authoritative loop model for tracked v0.85 docs only.

Later planning material for v0.86 and beyond may refine the model, but tracked v0.85 docs should reference this document rather than define another canonical loop.
