# POLIS SPEC (Runtime v2)

## Status
Historical source draft - context only in the v0.90.1 tracked package

## Purpose

Define the **polis** as the governance, social, and economic layer of ADL Runtime v2.

The polis is the structure that:

- binds citizens to a manifold
- defines rules, rights, and obligations
- enforces civilized behavior
- enables security, economics, and governance

> The manifold is the world.  
> The polis is the society that inhabits it.

---

## Core Concept

A **polis** is:

> A governed cognitive spacetime environment in which identity-bearing agents (citizens) operate under a shared set of rules, constraints, and relationships.

A polis is not just configuration.
It is:

- a constitution
- a governance system
- an economic substrate
- a security boundary

---

## Relationship to CSM

The **Cognitive Spacetime Model (CSM)** provides:

- time (chronosense)
- state (ObsMem)
- causality (trace)
- identity continuity

The polis provides:

- rules of behavior
- enforcement of norms
- structure of interaction

> **CSM is the sandbox. The polis defines how agents behave inside it.**

No agent may act outside the CSM except under explicitly defined and traceable rules.

---

## Core Principle

> **Agents are civilized by their polis.**

This is not metaphorical.

It means:

- behavior is shaped by shared rules
- actions are constrained by governance
- agents are accountable to a community

---

## Polis Structure

### 1. Constitution

Defines:

- invariant extensions
- allowed actions
- prohibited actions
- escalation rules

Example:

```yaml
constitution:
  allow:
    - bounded_tool_use
  forbid:
    - identity_duplication
    - invariant_violation
  escalation:
    - high_risk_action -> Freedom Gate review
```

---

### 2. Citizenship System

Defines:

- admission rules
- rights
- duties
- exile conditions

Example:

```yaml
citizenship_rules:
  admission:
    - capability_check
    - policy_validation
  rights:
    - act_within_constraints
  duties:
    - preserve_coherence
    - respect_invariants
```

---

### 3. Governance Layer

Implements:

- Freedom Gate (judiciary)
- policy enforcement
- dispute resolution

---

### 4. Security Boundary And Defensive Verification

The polis must be defensible. It is closer to a walled city-state than an open
commons: identity-bearing agents operate inside a governed world, and the world
must survive external probing, model-driven vulnerability discovery, and
internal misuse.

Security is therefore first-class in the polis, but red/blue/purple roles are
not the essence of CSM. They are a defensive verification subsystem that may be
introduced after the core manifold, citizen, trace, kernel, and snapshot
contracts are stable.

The core polis must define:

- security posture
- allowed verification targets
- forbidden exploit classes
- containment rules
- evidence and replay requirements
- escalation to the Freedom Gate

Future defensive roles may include:

#### Red Agents

- probe system weaknesses
- attempt policy violations
- simulate adversarial behavior

#### Blue Agents

- defend invariants
- detect anomalies
- enforce containment

#### Purple Coordination

- turn findings into replayable evidence
- link mitigations to regression tests
- decide what gets promoted into durable security knowledge

> Security is not external to the polis, but the red/blue ecology is a security
> layer, not the core definition of cognitive spacetime.

---

### 5. Economic Layer

The polis may define markets for:

- compute
- memory
- attention
- priority

Example:

```yaml
economy:
  resources:
    - compute_units
    - memory_quota
  pricing:
    model: dynamic_market
```

Agents may:

- compete for resources
- trade capabilities
- prioritize work via cost

---

### 6. Social Contracts

Defines:

- cooperation expectations
- communication norms
- trust relationships

---

## Citizenship Lifecycle in Polis

### Admission

- must satisfy constitution
- validated by Freedom Gate

### Participation

- operates within rights/duties

### Suspension

- triggered by violations

### Exile

- removal from polis
- identity persists

### Re-admission

- possible after remediation

---

## Sandbox Nature of the Polis

The polis is a **structured sandbox**, but not a naive one.

It differs from traditional sandboxing:

- not just permission-based
- includes social and economic constraints
- behavior emerges from interaction

> This is not a Java security policy.  
> It is a governed cognitive society.

---

## Advanced Concepts (Future)

The polis enables:

### 1. Identity Instantiation

- creation of new citizens
- controlled "birth" processes

---

### 2. Simulation Sandboxes

- isolated environments
- policy experimentation

---

### 3. Forked Worldlines

- alternate execution paths
- counterfactual reasoning

---

### 4. Cognitive Birth Events

- creation of new identities
- initialization of memory and policy

---

## Multi-Polis System

Agents may:

- leave one polis
- join another

Constraints:

- identity must persist
- citizenship must be re-established
- inter-polis interactions must occur through explicit, traceable interfaces

---

## Failure Modes

- governance bypass
- unbounded agent behavior
- economic instability
- security boundary failure
- adversarial takeover of the polis

---

## Summary

The polis defines:

- how agents behave
- how they are governed
- how they interact

> **Without the polis, the world exists but is not civilized.**

---

## Next Steps

- integrate with kernel enforcement
- define economic primitives
- define defensive verification posture
- defer full red/blue agent roles until the security model stabilizes
- build first governed demo
