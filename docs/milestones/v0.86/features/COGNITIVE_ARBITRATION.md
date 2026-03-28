# COGNITIVE_ARBITRATION.md

## Status

Tracked feature doc — v0.86

---

## Purpose

This document defines the **Cognitive Arbitration Layer** for ADL.

The Cognitive Arbitration Layer decides when an agent should use:

- fast-path cognition
- slow-path cognition
- escalation
- deferral
- refusal

It is therefore the operational control surface for bounded, cost-aware, risk-aware cognition.

---

## Why Arbitration Exists

A dual-process architecture is not sufficient by itself.

The core architectural problem is not merely that there are two modes of cognition.
The real problem is:

> deciding when each mode should be used.

In biological systems, this arbitration is implicit, evolved, and mostly opaque.
In ADL, it must be:

- explicit
- inspectable
- bounded
- improvable

---

## Scope

This layer is responsible for:

1. selecting fast vs slow cognition
2. escalating when uncertainty is high
3. incorporating cost and latency constraints
4. incorporating risk and reversibility
5. incorporating constitutional and policy constraints
6. emitting observable routing decisions
7. learning from outcomes over time

This layer is not itself the full reasoning engine.
It is the controller that decides **how much cognition to spend** on a task.

---

## Inputs to Arbitration

The arbitration layer should be able to consume signals from multiple ADL subsystems.

### Task Signals

- task type
- task complexity
- required precision
- user urgency
- reversibility of action
- expected blast radius

### Cognitive Signals

- model confidence estimate
- uncertainty estimate
- novelty / out-of-distribution indicators
- disagreement across agents or evaluators
- frame inadequacy indicators
- historical success rate on similar tasks

### Runtime Signals

- latency budget
- token / compute budget
- local model availability
- remote model availability
- current queue / execution pressure

### Governance Signals

- constitutional constraints
- freedom-gate restrictions
- explicit human approval requirements
- policy-based mandatory escalation rules

### Memory Signals

- relevant ObsMem history
- prior failure patterns
- prior success patterns
- learned routing priors
- prior reframing patterns and outcomes

---

## Outputs of Arbitration

The arbitration layer should emit a structured routing decision.

### Minimum Output Fields

- selected path (`fast`, `slow`, `hybrid`, `defer`, `refuse`)
- confidence in routing decision
- risk class
- governing constraints applied
- cost/latency assumptions used
- escalation reason, if any
- evidence references, if available

This decision should be artifact-visible and reviewable.

---

## Core Routing Modes

### 1. Fast Path

Use when:

- task is familiar
- cost of failure is low
- action is reversible or low-impact
- confidence is sufficient
- policy allows fast execution

Typical implementations:

- local small models
- heuristics
- cached answers
- lightweight evaluation

### 2. Slow Path

Use when:

- uncertainty is high
- cost of failure is high
- task is novel or ambiguous
- reasoning depth is required
- policy requires stronger review

Typical implementations:

- Gödel loops
- reasoning graphs
- multi-agent contention / review
- replayable evaluation and evidence gathering

### 3. Hybrid Path

Use when:

- fast path can generate a candidate
- slow path should validate, refine, or veto

Typical pattern:

- instinct proposes
- affect weights
- fast model drafts
- slow path verifies
- constitutional layer may override

### Reframing Trigger

In some cases, the correct arbitration outcome is not merely fast-path, slow-path, defer, or refuse.
The system may instead determine that the **current problem framing is inadequate**.

This can occur when:

- constraints appear internally inconsistent
- repeated attempts fail without meaningful new information
- evaluation signals oscillate without convergence
- agent disagreement suggests a framing error rather than ordinary uncertainty
- the task as posed does not align with the observed reality of the situation

In such cases, the arbitration layer should be able to emit a bounded **reframing trigger**.
This does not mean unconstrained reinterpretation.
It means the system explicitly recognizes that further execution under the current frame is likely to waste budget or produce low-value behavior.

Possible outcomes after reframing is triggered:

- restate the task at a higher level
- decompose the task differently
- request clarification or missing structure
- switch from execution to diagnosis
- escalate to a slower or more evidence-driven path under a revised frame

This connects arbitration directly to AEE convergence behavior.
Repeated failure is not only evidence about execution quality; it may also be evidence that the system is solving the wrong problem or solving the right problem under the wrong description.

In human cognition, one bounded expression of this capability is **humor**: the recognition that the active frame does not fit reality, combined with the ability to continue coherently rather than collapse.
ADL does not require anthropomorphic humor, but it may require the functional equivalent:

> detect that the current frame is inadequate, then shift frames without loss of coherence

This suggests that arbitration should eventually include a notion of **frame adequacy judgment** in addition to confidence, cost, and risk.

### 4. Defer

Use when:

- budget constraints prevent safe execution now
- required context is missing
- external dependencies are unavailable
- a better later execution point is acceptable

### 5. Refuse

Use when:

- the task is prohibited by policy
- risk exceeds allowable bounds
- required authorization is absent
- safe execution is not possible

---

## Bayesian Discriminator

The primary routing mechanism should be a **Bayesian Discriminator**.

Its role is to estimate whether fast-path execution is likely to be sufficient relative to the cost of error and the cost of escalation.

### Conceptual Inputs

- prior success on similar tasks
- current uncertainty
- task risk class
- action reversibility
- available budget
- policy constraints

### Conceptual Outputs

- probability that fast path succeeds
- expected harm of fast-path failure
- expected cost of slow-path escalation
- recommended route

### Conceptual Decision Rule

A simplified framing is:

- choose fast when expected value of fast execution exceeds expected value of slow execution **and** policy permits it
- otherwise escalate

A more explicit form is:

```text
E_fast = P(success_fast) * Value_success
         - P(failure_fast) * Cost_failure
         - Cost_fast

E_slow = P(success_slow) * Value_success
         - P(failure_slow) * Cost_failure
         - Cost_slow
```

Subject to:

- constitutional constraints
- risk thresholds
- approval requirements
- execution budget

---

## Risk-Aware Cognitive Routing

Routing cannot be optimized on cost alone.

The system must account for the possibility that a cheap fast-path mistake causes disproportionately large damage.

### Required Risk Dimensions

- severity of failure
- reversibility
- blast radius
- trust impact
- legal / constitutional sensitivity
- downstream workflow mutation risk

### Example Intuition

- autocomplete text → often fast-path acceptable
- code suggestion → maybe hybrid
- repository mutation → often slow or policy-gated
- financial or identity-sensitive action → likely slow and/or human-gated

This means arbitration must be **risk-aware**, not merely confidence-aware.

It may also need to be **frame-aware**.
A low-confidence situation and a wrong-frame situation are not identical.
Some tasks fail not because more reasoning is needed, but because the task has been posed incorrectly or the operational assumptions no longer match reality.

---

## Cognitive Budget

The arbitration layer should operate within an explicit **cognitive budget**.

This budget may include:

- maximum allowed latency
- maximum token or compute spend
- maximum number of escalations
- maximum number of review passes
- preferred local/remote model allocation

This makes cognition an allocatable runtime resource rather than an unbounded default.

---

## Learning Loop

Arbitration policy must improve over time.

### Feedback Cycle

1. route task
2. observe outcome
3. compare predicted vs actual success
4. update routing priors / thresholds
5. emit improved routing policy

This is where Gödel can improve not only task execution but the **meta-policy of cognition allocation**.

### Learnable Quantities

- calibration of confidence estimates
- routing thresholds by task class
- error rates by model and context
- budget usage vs outcome quality
- policy exceptions worth promoting to rules

---

## Relationship to Other ADL Components

| Component | Relationship to Arbitration |
|----------|------------------------------|
| FAST_SLOW_THINKING_MODEL.md | Defines the dual-process model that arbitration governs |
| Instinct Model | Produces fast candidate actions or defaults |
| Affect Model | Supplies weighting, urgency, salience, motivational pressure, and bounded signals relevant to reframing pressure |
| Gödel Agent | Learns better routing and policy over time |
| Reasoning Graphs | Provide slow-path structured cognition |
| ObsMem | Supplies priors, evidence, historical outcomes, and reframing history |
| Freedom Gate | Enforces bounded agency and hard constraints |
| IAM / Identity work | Later determines which actions are in-scope for the acting agent |

---

## Observable Artifact Surface

Arbitration decisions should be visible in artifacts rather than hidden inside runtime internals.

Possible artifact fields:

- route selected
- route alternatives considered
- uncertainty estimate
- risk classification
- cognitive budget consumed
- policy gate invoked
- escalation rationale
- post-hoc outcome assessment

This is necessary for:

- review
- replay
- trust claims
- Gödel improvement
- human oversight

---

## Failure Modes

### 1. Over-eager Fast Routing

Effects:

- shallow reasoning
- avoidable errors
- trust degradation
- unsafe automation

### 2. Over-eager Slow Routing

Effects:

- excessive cost
- latency blow-up
- poor usability
- analysis paralysis

### 3. Miscalibrated Confidence

Effects:

- repeated wrong route selection
- brittle behavior on novel tasks
- false sense of safety

### 4. Policy-Blind Optimization

Effects:

- economically efficient but constitutionally invalid decisions
- unsafe action selection
- bounded-agency failure

### 5. Hidden Routing Logic

Effects:

- non-auditable cognition
- weak trust surface
- limited ability to improve the system

---

## Design Principles

1. **Arbitration must be explicit.**
   The choice of fast vs slow should not be hidden.

2. **Arbitration must be bounded.**
   Costs, risks, and constraints must be represented.

3. **Arbitration must be reviewable.**
   Routing decisions should leave inspectable traces.

4. **Arbitration must be improvable.**
   Gödel and surrounding systems should be able to refine it.

5. **Arbitration must be subordinate to policy.**
   Efficiency never outranks constitutional constraints.

---

## Open Questions for v0.86 Planning

- What is the minimal viable routing schema?
- Which risk classes should exist initially?
- How should confidence and uncertainty be represented?
- Should arbitration live in AEE, alongside it, or beneath it?
- Which decisions require mandatory human approval?
- How should hybrid routing be represented in artifacts?
- What is the smallest demo that proves the concept?
- How should frame inadequacy be detected, represented, and surfaced in artifacts?

---

## Summary

The Cognitive Arbitration Layer is the system that decides:

> when to think fast, when to think slowly, and when not to act at all.

It turns dual-process cognition into an engineered, bounded, reviewable control system.
Without it, fast/slow thinking remains a metaphor.
With it, ADL gains a practical mechanism for:

- cost-aware cognition
- risk-aware routing
- bounded agency
- self-improving allocation of thought

This makes cognitive resource allocation a first-class architectural concern in ADL.
