# Cognitive Stack — v0.85

## Status

Tracked authority for the v0.85 cognitive sub-stack.

## Purpose

This document defines the authoritative cognitive stack for the tracked v0.85 milestone docs.

It exists to remove two sources of ambiguity:

- fractional layer numbering such as "3.5"
- confusion between the internal cognitive sub-stack and the higher-level milestone architecture view

This document does not replace the broader architecture framing in `VISION_v0.85.md`.
It defines the internal bounded-cognition sub-stack that sits inside that broader architecture.

## Scope Note

The v0.85 doc set now uses two valid but different views:

1. a high-level architecture view
   - execution
   - workflow
   - cognitive
   - evaluation
   - adaptive
2. an internal cognitive sub-stack view
   - the bounded internal layers that shape cognition, routing, memory, and action

This document is the authority for the second view.

## Canonical Cognitive Stack

```text
1. Instinct
2. Bounded Affect
3. Cognitive Arbitration
4. Freedom Gate
5. Reasoning and Reframing
6. Memory (ObsMem)
7. Adaptive Execution and Artifacts
```

The numbering above is stable for tracked v0.85 docs.

## Layer Roles

### 1. Instinct

Instinct provides persistent fast-prior pressure.

It:
- creates durable pressure toward coherence, completion, curiosity, or integrity
- biases what matters before slower cognition starts
- does not override policy or governance

### 2. Bounded Affect

Bounded affect provides dynamic weighting signals for:
- salience
- urgency
- contradiction pressure
- persistence
- confidence shifts

It is a bounded reasoning-control surface, not a simulation of human feeling.

### 3. Cognitive Arbitration

Cognitive arbitration is a full numbered layer, not a fractional layer.

It decides:
- fast vs slow cognition
- hybrid routing
- defer vs refuse
- how much cognitive effort to spend

It consumes evidence such as:
- bounded affect signals
- uncertainty
- cost
- reversibility
- policy pressure
- frame adequacy

### 4. Freedom Gate

The freedom gate is the governance and constitutional boundary.

It:
- enforces hard constraints before action
- can override outputs from instinct, affect, or arbitration
- keeps agency bounded and reviewable

### 5. Reasoning and Reframing

This layer carries explicit reasoning and frame repair.

It includes:
- structured reasoning
- critique
- meta-reasoning
- reframing when the current frame is inadequate

Reframing is treated as a bounded reasoning function, not as a separate fractional layer.

### 6. Memory (ObsMem)

Memory stores:
- outcomes
- failures
- routing history
- reframing history
- useful observations for later cognition

Memory informs future affect, arbitration, and reasoning.

### 7. Adaptive Execution and Artifacts

Adaptive execution carries out bounded action and emits inspectable outputs.

It includes:
- AEE execution behavior
- progress/failure signaling
- emitted artifacts
- evaluation inputs for later routing and memory updates

## Explicit Numbering Decisions

1. Cognitive arbitration is Layer 3.
   - It is not "Layer 3.5".
2. Reframing belongs inside Layer 5.
   - It is a reasoning/meta-reasoning function, not a standalone layer.
3. The freedom gate is Layer 4.
   - It is not merely an external note; it is part of the bounded-cognition stack.
4. Evaluation is a loop function, not a separate numbered stack layer here.
   - In the canonical loop model, evaluation feeds bounded affect, arbitration, and memory.

## Relationship To The Canonical Cognitive Loop

The canonical v0.85 loop is defined in `COGNITIVE_LOOP_MODEL_v0.85.md`.

That loop describes the dynamic cycle:

```text
instinct -> bounded affect -> cognitive arbitration -> freedom gate
        -> execution (AEE) -> evaluation -> reframing? -> memory (ObsMem)
        -> bounded affect
```

This stack document complements that loop:
- the loop describes flow over time
- the stack describes the stable internal layer model

The two views should agree, not compete.

## Relationship To The Higher-Level Architecture View

The five-layer architecture in `VISION_v0.85.md` is still valid.

That view describes the milestone at system scale:
- execution
- workflow
- cognitive
- evaluation
- adaptive

This cognitive-stack document describes the internal structure of the bounded cognition part of that system.

## Design Rules

1. No fractional layers
   - tracked v0.85 docs should not use numbering such as "3.5"
2. One authority
   - this doc is the tracked authority for cognitive stack numbering
3. Bounded cognition
   - all layers remain policy-governed and inspectable
4. Compatibility with loop authority
   - the stack must remain compatible with `COGNITIVE_LOOP_MODEL_v0.85.md`

## Scope Boundaries

This issue does not:
- implement the stack in runtime code
- settle every future v0.86+ cognition question
- replace the higher-level milestone architecture view

It does:
- define stable numbered layers for tracked v0.85 docs
- give later instinct, bounded-affect, arbitration, and AEE work a consistent reference point
