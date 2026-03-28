

# FAST_SLOW_THINKING_MODEL.md

## Status

Tracked feature doc — v0.86

---

## Overview

This document defines a **dual-process cognitive architecture** for ADL agents inspired by Daniel Kahneman’s model of fast and slow thinking, but reinterpreted as an **explicit, engineered system**.

Unlike biological cognition, ADL agents can:

- explicitly define their fast and slow processes
- measure their performance
- improve routing decisions over time
- optimize for cost, latency, and correctness

This model is therefore not metaphorical. It is an **operational control system** for agent cognition.

---

## Core Model

ADL cognition is divided into two execution paths:

### Fast Path (System 1)

Characteristics:

- low latency
- heuristic / approximate
- lower cost
- may be incorrect

Examples:

- routing decisions
- classification
- quick responses
- lightweight local model inference

### Slow Path (System 2)

Characteristics:

- higher latency
- explicit reasoning
- higher cost
- higher reliability

Examples:

- Gödel loops
- reasoning graphs
- multi-agent debate
- replay validation

---

## Key Insight: The Switch Matters More Than the Systems

The central problem is not implementing fast and slow thinking.

The central problem is:

> deciding when to use each.


This is handled by the **Cognitive Arbitration Layer**.

---

## Integrated Cognition (Φ-lite)

ADL systems vary in their degree of **cognitive integration**. We introduce an engineering interpretation inspired by Integrated Information Theory (IIT), but adapted for practical use.

### Φ-lite (Engineering Interpretation)

Rather than computing formal Φ (which is intractable), we define:

Φ_ADL ≈ degree of:
- cross-component dependency
- memory coupling (ObsMem participation)
- feedback loops (Gödel / AEE participation)
- irreducibility of the execution graph

### Interpretation

- Low Φ_ADL:
  - stateless execution
  - independent tools
  - simple DAG workflows
- Medium Φ_ADL:
  - workflows with memory and limited feedback
- High Φ_ADL:
  - tightly coupled reasoning, memory, affect, and policy loops
  - Gödel-driven adaptive systems
  - persistent identity-like behavior

### Relationship to Fast/Slow Thinking

- Fast Path → typically low Φ_ADL
  - local, cheap, decomposable
  - minimal cross-component interaction

- Slow Path → typically higher Φ_ADL
  - integrated reasoning across components
  - memory + evaluation + policy interaction
  - irreducible cognitive behavior

### Design Insight

The Cognitive Arbitration Layer is not only choosing between:

- fast vs slow
- cheap vs expensive

It is implicitly choosing between:

> lower integration vs higher integration

This reframes routing as a **cognitive depth allocation problem**, not just a cost/latency tradeoff.

### Strategic Implication

Higher Φ_ADL systems:

- exhibit stronger coherence
- support adaptive learning (Gödel loop)
- enable identity and policy continuity

Lower Φ_ADL systems:

- are cheaper and faster
- scale more easily
- are appropriate for low-risk tasks

The optimal system dynamically balances Φ_ADL based on:

- risk
- uncertainty
- cost constraints
- reversibility of outcomes

---

## Bayesian Discriminator (Cognitive Router)

We introduce a new core component:

### Bayesian Discriminator

The Bayesian Discriminator is responsible for routing tasks between the fast and slow paths.

It evaluates:

- uncertainty
- task complexity
- historical error rates
- cost sensitivity
- risk profile

It outputs:

- probability that fast-path execution will be sufficient
- expected cost of error
- expected cost of escalation

### Decision Rule (Conceptual)

Fast path is selected when:

- P(success_fast) × benefit_fast > P(failure_fast) × cost_of_failure

Otherwise, escalate to slow path.

---

## Learning and Gödel Integration

The Bayesian Discriminator is not static.

It is improved via the Gödel loop:

1. Observe outcomes (success / failure of fast path)
2. Update belief model
3. Adjust routing thresholds
4. Emit improved policy

This creates a feedback loop:

```
Fast decision → Outcome → Evaluation → Gödel update → Improved routing
```

This is one of the first places where Gödel becomes **directly operational in cognition**, not just workflow mutation.

---

## Economic and Biological Motivation

Biological cognition uses fast and slow thinking because:

- computation is expensive
- time is limited
- survival depends on efficient decision-making

ADL systems face analogous constraints:

- API calls are expensive
- latency impacts usability
- compute resources are finite

### Cost Optimization

Fast path can be implemented using:

- small local models
- cached heuristics
- deterministic shortcuts

Slow path may involve:

- large remote models
- multi-agent workflows
- replay and validation

Therefore, correct routing produces:

- lower cost
- lower latency
- better system scalability

---

## Risk Model

Routing is not purely economic.

Incorrect fast-path decisions can be catastrophic.

### Analogy

In biology:

- a wrong fast decision → you get eaten

In ADL:

- incorrect output
- system failure
- incorrect mutation
- degraded trust

Therefore, the system must explicitly model:

- cost of failure
- reversibility of actions
- safety constraints

---

## Arbitration Layer Responsibilities

The Cognitive Arbitration Layer must:

1. Route tasks (fast vs slow)
2. Escalate when uncertainty is high
3. Override fast-path decisions when policy requires
4. Track performance metrics
5. Provide signals to Gödel

---

## Relationship to Other ADL Components

| Component | Role |
|----------|------|
| Instinct Model | Generates fast-path candidate actions |
| Affect Model | Provides weighting and urgency signals |
| Bayesian Discriminator | Chooses fast vs slow path |
| Gödel Agent | Learns and improves routing and policies |
| Reasoning Graphs | Execute slow-path reasoning |
| ObsMem | Provides evidence and historical outcomes |
| Freedom Gate | Enforces constraints and overrides |

---

## Key Design Insight

In humans, System 1 is opaque.

In ADL:

> System 1 is programmable.

This enables:

- explicit heuristics
- versioned instincts
- measurable performance
- systematic improvement

This is a fundamental architectural advantage over biological cognition.

---

## Failure Modes

### 1. Overuse of Fast Path

- shallow reasoning
- incorrect outputs
- hallucination-like behavior

### 2. Overuse of Slow Path

- high latency
- high cost
- reduced throughput

### 3. Miscalibrated Discriminator

- incorrect routing decisions
- oscillation between paths
- degraded performance

---

## Strategic Importance

This model transforms ADL into:

- a cost-aware cognitive system
- a self-improving decision architecture
- a system capable of bounded agency

It also introduces a capability not present in most agent frameworks:

> explicit cognitive resource allocation.

---

## Open Questions (v0.86 Planning)

- What is the minimal viable Bayesian model?
- How are uncertainty signals represented in artifacts?
- How does the discriminator interact with AEE?
- What metrics define success for routing decisions?
- How are safety constraints encoded into routing?

---

## Summary

The fast/slow thinking model provides:

- a cognitive foundation
- a cost model
- a learning loop

Most importantly, it provides a mechanism for:

> deciding when to think.

This is a core requirement for any system that aims to exhibit effective agency.
