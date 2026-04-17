
# SENSE OF URGENCY AND TASK PRIORITIZATION

## Status

Partially delivered

Current disposition: `v0.88 bounded temporal slice delivered / v0.90 governance-package source material`

Public support issue:
- `#1614` Split urgency and task prioritization into temporal and governance bands

Delivered bounded `v0.88` implementation owner:
- `WP-06 / #1651` Commitments and deadlines

This document now lives in `docs/milestones/v0.90/` as source material for
the later governance/reprioritization package.

Delivered/planned split:
- temporal/deadline and obligation pressure belongs with the `v0.88` chronosense package, especially `COMMITMENTS_AND_DEADLINES.md`, and is now delivered there as the bounded `v0.88` slice
- scheduling, interruption, reprioritization, and priority-aware decision behavior belong with a later governance/agency package now targeted at `v0.90` and preserved in the local backlog

It remains useful source material, but it is not a bounded canonical feature doc as written.
Its `v0.88` contribution is now represented by the narrower commitment/deadline surface,
while the broader reprioritization/governance material remains future work.

Do not treat this as an active `v0.89.1` execution input. Its remaining value is
as a v0.90 planning source for explicit priority schema, scheduler,
interruption, resumption, and trace-artifact work.

## Current Packaging Decision

This doc now has a clearer future role:

- `v0.88` delivered the bounded temporal/deadline and obligation-pressure slice
- `v0.89` is intentionally not absorbing the reprioritization remainder during the active governance wave
- `v0.89.1` is currently centered on adversarial/runtime and anti-harm work, so it is not the right home either
- the remaining urgency/reprioritization package now points at `v0.90` as the first clean governance/agency follow-on band

That later package should be treated as a concrete runtime and review surface, not just a loose conceptual remainder.

## Later-Band Package Definition

The later governance/agency package should include at least:

- explicit priority model and schema surfaces
- deterministic scheduler/reprioritization policy
- interruption and resumption semantics
- trace-linked priority and reprioritization artifacts
- governance-aware justification for deferral, interruption, and task selection
- evaluation surfaces for decision quality under competing demands

This is the package that remains after the delivered `v0.88` commitment/deadline slice.

## Non-claim

This document does not claim that runtime reprioritization is already implemented.

It claims only that:

- the bounded temporal slice is real and delivered in `v0.88`
- the broader reprioritization/governance remainder is now packaged more concretely for later execution

## Overview

ADL currently provides strong structures for deterministic execution, traceability, and task decomposition. However, it lacks an explicit representation of **urgency** and **priority**, which are essential for effective real-world decision making.

In human cognition, urgency and importance shape behavior continuously. In ADL, this dimension is currently implicit or lost, leading to:

- Tasks being treated as equally important
- Lack of interruptibility or reprioritization
- Inefficient scheduling of work
- Reduced alignment with real-world constraints

This document proposes a structured way to represent and utilize urgency and priority within ADL.

---

## Core Distinction

Urgency and importance are related but distinct.

- **Importance**: how much a task contributes to long-term goals
- **Urgency**: how time-sensitive a task is

These combine to determine **priority**.

Priority is not static. It evolves based on:
- Time
- External signals
- Internal state
- Outcomes of other tasks

---

## Design Goals

The system should:

- Represent urgency explicitly in tasks and workflows
- Allow dynamic reprioritization during execution
- Support interruption and rescheduling
- Preserve determinism and traceability
- Integrate with existing skill and execution models

---

## Proposed Model

### Task Attributes

Each task or node may include:

- `importance`: static or slowly changing weight
- `urgency`: time-sensitive weight (may increase over time)
- `deadline`: optional explicit temporal constraint
- `priority`: derived value (function of importance + urgency)

These attributes should be:
- Explicit in schemas
- Captured in trace
- Observable and explainable

---

### Priority Function

Priority should be computed, not assigned arbitrarily.

Example conceptual model:

priority = f(importance, urgency, time_to_deadline, context)

Where:
- urgency may increase as deadlines approach
- importance may remain stable
- context may include system state or external inputs

The exact function can evolve but must be:
- Deterministic
- Traceable

---

### Temporal Dynamics

Urgency is inherently time-based.

The system should support:

- Increasing urgency as time passes
- Deadline-based escalation
- Time-aware evaluation loops

This connects directly to future **chronosense** work.

---

## Execution Semantics

Priority should influence execution without breaking determinism.

Key behaviors:

- Higher-priority tasks are selected first
- Lower-priority tasks may be deferred
- Running tasks may be interrupted if a higher-priority task appears
- All decisions must be recorded in trace

---

## Decision Surfaces

Urgency interacts with agency.

Agents should be able to:

- Re-evaluate priorities during execution
- Justify deferring or interrupting tasks
- Surface conflicts between tasks

This integrates with:
- Freedom Gate (decision authority)
- Evaluation systems (was the right task chosen?)

---

## Integration Points

### 1. Trace

- Record priority values at decision points
- Capture why a task was selected over others

### 2. Skills

- Allow skills to expose urgency signals
- Ensure skill execution respects priority constraints

### 3. Scheduler / Runtime

- Introduce priority-aware scheduling
- Support interruption and resumption

### 4. Aptitudes

- Evaluate how well an agent prioritizes tasks
- Measure decision quality under competing demands

---

## Risks

- Overcomplicating execution model
- Introducing non-deterministic behavior
- Allowing hidden priority heuristics

Mitigation:

- Keep priority functions explicit and simple
- Ensure all priority decisions are traceable
- Avoid implicit or model-only prioritization

---

## Success Criteria

- Tasks are not treated uniformly when they should not be
- The system can explain why one task was chosen over another
- Priority changes over time are visible and consistent
- Agents demonstrate improved decision quality in multi-task scenarios

For the current local planning state, the immediate success criteria are:

- the delivered `v0.88` slice remains clearly bounded
- the later-band remainder has a concrete package definition
- backlog and placement surfaces agree on the future home

---

## Notes

This concept is foundational for:

- Real-world scheduling
- Multi-agent coordination
- Autonomous operation

It is also a prerequisite for meaningful agency, as prioritization is a core component of decision-making behavior.

This work should align with:
- Chronosense (time awareness)
- Freedom Designed (agency and governance)
- Aptitude systems (evaluation of decision quality)

Likely future home:
- `v0.90` governance/agency follow-on band
