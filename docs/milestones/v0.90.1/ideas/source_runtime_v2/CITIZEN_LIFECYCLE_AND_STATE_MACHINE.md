


# CITIZEN LIFECYCLE AND STATE MACHINE (Runtime v2)

## Status
Historical source draft - context only in the v0.90.1 tracked package

## Purpose

Define the **lifecycle, state transitions, identity model, and citizenship model** for agents in ADL Runtime v2.

This document establishes:

- what a **citizen** is
- how it differs from **identity**
- how agents transition through states
- how agents relate to the **manifold (polity)**

---

## Core Distinction: Identity vs Citizenship

This is a foundational concept.

### Identity

Identity is:

- persistent
- intrinsic
- evolving
- non-transferable

It includes:

- memory (ObsMem)
- internal models
- preferences
- learned structure
- "World 2" (subjective experience, in Popper’s sense)

> Identity persists across time, migration, and even across polities.

Identity MUST NOT be owned by any single manifold.  
It must remain portable across manifolds.

---

### Citizenship

Citizenship is:

- relational
- external
- governed
- replaceable

It includes:

- rights
- duties
- permissions
- constraints
- obligations to the manifold

> Citizenship defines the agent’s relationship to a specific **polity (manifold)**.

---

## Key Principle

> **Identity persists. Citizenship is assigned.**

An agent may:

- migrate between manifolds
- change citizenship

But its identity remains continuous.

---

## Analogy (Diaspora Model)

Inspired by Greg Egan's `Diaspora`:

- Identity ≈ the internal mind-state
- Citizenship ≈ membership in a polis

Polises differ in:

- rules
- constraints
- capabilities

Agents may:

- leave one polity
- join another

without losing identity.

---

## Citizen Definition

A **citizen** is:

> An identity-bearing agent bound to a manifold through a citizenship contract.

---

## Citizen Structure

```yaml
citizen:
  citizen_id: ga-004
  identity_ref: id://ga-004
  manifold_id: csm-01

  status: awake | sleeping | migrating | suspended | terminated

  citizenship:
    rights: [...]
    duties: [...]
    constraints: [...]
    policy_profile: ...

  identity:
    memory_namespace: mem://ga-004
    continuity_anchor: <timestamp>
```

---

## State Machine Overview

### States

- `created`
- `admitted`
- `awake`
- `sleeping`
- `suspended`
- `migrating`
- `detached`
- `exiled`
- `terminated`

---

## State Definitions

### 1. created

- identity instantiated
- no citizenship yet

---

### 2. admitted

- citizenship granted
- not yet active

---

### 3. awake

- active in manifold
- can:
  - act
  - decide (via Freedom Gate)
  - interact

---

### 4. sleeping

- inactive but preserved
- no execution
- memory + obligations persist

---

### 5. suspended

- restricted by governance
- may not act
- used for:
  - violations
  - quarantine

---

### 6. migrating

- in transition between manifolds
- must not act

---

### 7. detached

- identity exists
- no active citizenship
- MUST NOT allow action
- cannot act until admitted to a manifold

### 7a. exiled

- identity persists
- explicitly rejected by a polity
- cannot act or re-enter without remediation

---

### 8. terminated

- identity destroyed OR archived
- irreversible

---

## State Transitions

### created → admitted

- admission process
- policy validation
- citizenship contract must pass Freedom Gate validation

---

### admitted → awake

- activation

---

### awake → sleeping

- runtime sleep event

---

### sleeping → awake

- wake event

---

### awake → suspended

- invariant violation or governance action

---

### suspended → awake

- remediation + approval

---

### awake → migrating

- snapshot initiated
- requires:
  - no active conflicting instance (C-2)
  - snapshot initiation event (M-3)

---

### migrating → admitted

- rehydration complete in new manifold

---

### admitted → detached

- citizenship revoked

---

### detached → admitted

- new citizenship assigned

---

### any → terminated

- explicit termination

---

## Transition Constraints (Invariants)

- no direct awake → detached (must suspend or sleep first)
- no duplicate awake instances
- migrating state must be exclusive
- terminated is terminal (no exit)
- detached state cannot act

---

## Citizenship Model

Citizenship is defined as a **contract**:

```yaml
citizenship_contract:
  manifold_id: csm-01
  rights: [...]
  duties: [...]
  constraints: [...]
  enforcement_owner: Freedom Gate
```

---

## Citizenship Change

### Leaving a Polity

- citizen enters `detached`
- obligations resolved or transferred

---

### Joining a New Polity

- new citizenship contract assigned
- identity reused

---

## Identity Continuity Requirements

- identity_ref must remain stable
- memory_namespace must persist
- continuity_anchor must advance monotonically

---

## Interaction with Other Systems

### With Manifold

- citizenship binds citizen to manifold

### With Snapshot System

- citizen state must serialize fully

### With Invariants

- lifecycle transitions must be validated

### With Freedom Gate

- all actions from `awake` state pass through gate

---

## Failure Modes

- duplicate active citizen
- lost identity reference
- citizenship mismatch with manifold
- illegal state transition

---

## Summary

This model defines:

- identity as persistent self
- citizenship as relational contract
- lifecycle as governed state transitions

> **Agents are not just processes. They are citizens in a governed world.**

---

## Next Steps

- map lifecycle to runtime implementation
- integrate with invariant enforcement
- test local sleep/wake identity continuity first
- defer cross-polis and cross-machine migration semantics to the later identity/migration band
