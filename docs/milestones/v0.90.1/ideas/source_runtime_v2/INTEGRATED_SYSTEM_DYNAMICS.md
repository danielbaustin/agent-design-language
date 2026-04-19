# INTEGRATED SYSTEM DYNAMICS (Runtime v2)

## Status
Historical source draft - context only in the v0.90.1 tracked package

## Purpose

Define the **integrated dynamics** of ADL Runtime v2 by composing three core
loops plus an optional security-pressure layer:

1. **Cognitive Loop** (reasoning and choice)
2. **Economic Loop** (pressure and allocation)
3. **Governance Loop** (constraints and legitimacy)
4. **Adversarial Verification Layer** (security and resilience; not CSM core)

This document explains how these loops interact to produce **bounded, persistent, and governable behavior**.

---

## Core Thesis

> **ADL Runtime v2 is a closed, governed dynamical system in which cognition,
> economics, and governance continuously interact through trace.**

The adversarial/security layer is crucial for defending a real polis, but it is
not constitutive of the Cognitive Spacetime Model itself. CSM is about time,
state, causality, continuity, trace, and governed action. Red/blue/purple
security roles are a later defensive subsystem that can operate inside the
polis under explicit posture.

All loops are synchronized and mediated through the **kernel event pipeline** to ensure deterministic control and full traceability.

No loop operates in isolation. All loops are mediated by:

- chronosense (time)
- trace (causality)
- identity (continuity)
- citizenship (polity constraints)

---

## Core Loops And Security Layer

### 1) Cognitive Loop

Canonical form (from COGNITIVE_LOOP_MODEL):

```text
instinct → affect → arbitration → freedom_gate → execution (AEE)
        → evaluation → (reframing?) → memory (ObsMem) → affect
```

Role:
- generates candidate actions
- selects among alternatives
- commits through the Freedom Gate

Key properties:
- bounded
- replayable
- policy-mediated

---

### 2) Economic Loop

```text
resource_state → pricing → arbitration_inputs → decision
             → resource_consumption → trace → reputation → pricing
```

Role:
- allocates scarce resources (compute, memory, attention, bandwidth)
- creates **pressure** that influences decisions

Key integrations:
- feeds **cost_estimate** into arbitration
- constrains execution via scheduler
- persists in snapshot for continuity

---

### 3) Governance Loop

```text
policies/constitution → freedom_gate_evaluation → commitment
                    → trace → review/reputation → policy updates (bounded)
```

Role:
- enforces legality and legitimacy of actions
- provides refusal/deferral/escalation

Key integrations:
- overrides all other signals when necessary
- records decision events in trace
- binds citizens via **citizenship contracts**

---

### 4) Adversarial Verification Layer (Security Pressure)

Role:
- proactively discover vulnerabilities before external adversaries
- continuously test the system under realistic threat assumptions
- validate system correctness and resilience
- prevent latent weaknesses from persisting

> **Assumption:** any real vulnerability will be discovered quickly by external actors.  
> Therefore, the system must discover and validate vulnerabilities first.

This is the walled-city-state problem for the polis. If frontier systems such as
Mythos-class vulnerability finders collapse the detection ceiling, then a
serious polis must assume continuous external probing. ADL should support
governed internal adversarial verification so our own red team finds and
replays weaknesses first.

This layer remains explicitly bounded:

- it is security posture, not the definition of CSM
- it runs under the Freedom Gate and policy constraints
- it emits traceable evidence and replay artifacts
- it may be deferred from the first Runtime v2 kernel prototype

---

## Coupling Between Loops

### A. Economics → Cognition

- cost and scarcity influence arbitration
- high cost may trigger:
  - defer
  - alternative selection
  - reduced depth (fast path)

---

### B. Cognition → Governance

- selected action is passed to the Freedom Gate
- alternatives are evaluated under policy and commitments

---

### C. Governance → Execution

- only approved actions proceed to execution
- rejected actions produce trace + possible escalation

---

### D. Execution → Trace

- all actions produce events
- trace is the **single source of truth**

---

### E. Trace → Memory / Reputation

- ObsMem stores outcomes and context
- reputation is derived from trace history

---

### F. Memory → Cognition

- prior outcomes affect affect/arbitration
- supports learning and continuity

---

### G. Economics ↔ Governance

- policies constrain markets (fairness, anti-hoarding)
- economic impact is evaluated by the Freedom Gate

---

### H. Security Pressure ↔ Core Loops

- adversarial verification can stress economics, governance, and cognition
- defensive roles can enforce invariants and containment
- the first Runtime v2 prototype only needs a simple policy/invariant violation
  proof, not a full red/blue ecology

---

### I. Security vs Economic Separation

Security pressure and economic pressure are distinct and must not be conflated.

- adversarial verification optimizes for correctness and resilience
- economic loop optimizes for allocation and prioritization

Interactions occur only at controlled boundaries:

- scheduler (resource allocation for security verification work)
- governance (policy may prioritize security work)
- prioritization layer (selection of which findings to address first)

The security layer is not a market mechanism.  
It is a **falsification and integrity system**.

---

## The Unified Feedback System

The system can be summarized as:

```text
economics → arbitration → freedom_gate → execution → trace
     ↑                                              ↓
     └──────── memory / reputation / policy ────────┘
```

Security verification may deliberately perturb this loop, but only under
explicit posture and governance.

---

## Invariant Boundaries

All loops are constrained by invariants:

- **Identity invariants** → no duplication, continuity preserved
- **Temporal invariants** → monotonic ordering, anchoring
- **Trace invariants** → full observability
- **Migration invariants** → safe portability
- **Governance invariants** → Freedom Gate supremacy

> No loop may produce behavior that violates invariants.

---

## Role of the Kernel

The kernel enforces the dynamics by:

- running the **event pipeline**
- validating invariants before execution
- ensuring all effects are traced
- coordinating snapshot and migration

Kernel = **physics engine of the system**

---

## Role of the Polis

The polis shapes the dynamics by:

- defining policies and constitution
- setting economic rules
- defining citizenship contracts
- defining how defensive verification is allowed to operate

Polis = **social and governance layer of the system**

---

## Emergent Properties

When all loops operate together, the system exhibits:

- bounded agency
- persistence across time
- adaptive behavior (without loss of control)
- economic prioritization
- governance-compliant action
- optional adversarial resilience when the security layer is enabled
- optional proactive vulnerability discovery when the security layer is enabled

---

## Failure Modes (Cross-Loop)

- economic dominance overriding governance (must not happen)
- governance deadlock (no action possible)
- security tooling capture of scheduler or pricing
- memory corruption breaking feedback loop
- trace gaps breaking reconstruction

---

## Observability Requirements

To be valid, the system MUST allow inspection of:

- decisions (Freedom Gate events)
- costs and resource usage
- policy constraints applied
- security verification events and responses, when enabled
- state transitions of citizens

---

## Summary

ADL Runtime v2 is a **coupled dynamical system**:

- cognition selects actions
- economics applies pressure
- governance constrains behavior
- security verification can test resilience under explicit posture
- trace records everything

> **The system is not just a pipeline. It is a living, governed feedback system.
> When the security layer is enabled, it can discover its own failures before
> external attackers do.**

---

## Next Steps

- map loops to concrete runtime APIs
- implement instrumentation for each loop
- build core integrated demo (cognition/economics/governance active)
- defer full red/blue/purple security ecology until the security model stabilizes
- validate invariants under stress
