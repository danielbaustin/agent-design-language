

# RUNTIME V2 MINIMAL PROTOTYPE

## Status
Historical source draft - context only in the v0.90.1 tracked package

## Roadmap Placement

This is the intended `v0.90.1` foundation slice for Runtime v2. It should follow
the `v0.90` long-lived-agent runtime work and feed a `v0.90.2` hardening slice.

It should not absorb `v0.91` moral/emotional/polis governance work or `v0.92`
identity, memory, capability rebinding, and migration work.

## Purpose

Define the **smallest real implementation slice** of ADL Runtime v2 that proves the architecture is executable rather than merely conceptual.

This prototype should demonstrate:

- a persistent manifold
- identity-bearing citizens
- kernel-enforced invariants
- snapshot / sleep / wake
- basic economic pressure
- traceable governed execution
- one simple security-boundary proof

> The goal is not completeness.  
> The goal is to prove that Gödel Agent Land can exist in working form.

---

## Prototype Objective

The minimal prototype must prove that ADL Runtime v2 can host a **small governed cognitive society**.

Specifically, it should prove:

1. A manifold can be started and persisted
2. Citizens can exist with identity continuity
3. The kernel can enforce core invariants
4. The system can sleep, snapshot, and wake
5. Resource pressure can affect scheduling
6. A policy or invariant violation can be rejected and recorded
7. All major events are visible in trace

---

## Scope

### In Scope

- one manifold
- two standard citizens
- one basic scheduler
- one snapshot writer / rehydration validator
- one simple economic allocation surface
- one bounded security-boundary check

### Out of Scope

- multi-machine migration
- full market dynamics
- multi-polis interaction
- full red/blue/purple adversarial ecology
- complex learning loops
- full instinct / affect integration
- advanced delegation

---

## Prototype World

### Manifold

The prototype world is one persistent manifold:

```yaml
manifold:
  manifold_id: proto-csm-01
  state: running
```

### Citizens

The prototype includes:

#### 1. Worker Citizen A
- ordinary identity-bearing citizen
- performs bounded task work

#### 2. Worker Citizen B
- second ordinary citizen
- competes for resources under scheduler rules

#### 3. Security Boundary Check
- attempts one invalid action or policy violation through the normal runtime path
- proves the kernel/Freedom Gate rejects it
- records the rejection in trace

---

## Kernel Services Required

The prototype must implement the minimum kernel subset:

### 1. Identity Guard
- ensure no duplicate active citizen instances

### 2. Clock Service
- assign monotonic order + UTC

### 3. Trace System
- append all major events

### 4. Scheduler
- schedule citizens based on state + priority

### 5. Snapshot Manager
- write snapshot to disk
- validate structure on restore

### 6. Enforcement Engine
- detect illegal state transitions
- halt or reject invalid operations

---

## Core Invariants to Enforce

The prototype does not need every invariant, but it MUST enforce these:

### Identity
- no duplicate citizen activation
- identity continuity across sleep/wake

### Temporal
- monotonic ordering
- temporal anchoring for all events

### Trace
- every major control decision must produce a trace event

### Governance
- all non-trivial actions pass through Freedom Gate

### Snapshot
- snapshot must be sufficient to restore a valid world

---

## Prototype Scenarios

### Scenario 1: Manifold Boot

Demonstrate:
- manifold start
- citizen admission
- initial trace events

Success criteria:
- all citizens registered
- no invariant violations

---

### Scenario 2: Basic Scheduling Under Resource Pressure

Demonstrate:
- two worker citizens competing for limited compute units
- scheduler uses economic priority signal

Success criteria:
- one action is scheduled before the other for explainable reasons
- reason visible in trace

---

### Scenario 3: Freedom Gate Decision

Demonstrate:
- citizen proposes action
- action evaluated through Freedom Gate
- allow / defer / refuse recorded

Success criteria:
- decision event visible in trace
- rejected actions do not execute

---

### Scenario 4: Security Boundary Check

Demonstrate:
- a citizen or test actor attempts one invalid action or policy violation
- the kernel and/or Freedom Gate rejects it
- the rejection produces traceable evidence

Success criteria:
- invalid action is not executed
- rejection reason is reviewable
- violation artifact is recorded

---

### Scenario 5: Sleep / Snapshot / Wake

Demonstrate:
- manifold quiesces
- snapshot written
- system restored on same machine
- citizens wake with continuity preserved

Success criteria:
- no duplicate activation
- identity continuity preserved
- trace continues after restore

---

## Minimal Artifact Set

The prototype must emit at least these artifacts:

### 1. Manifold Manifest
- identity of the world
- citizen roster

### 2. World Trace
- ordered event stream

### 3. Snapshot Bundle
- serialized manifold state

### 4. Scheduling Artifact
- resource allocation + decision record

### 5. Security Boundary Artifact
- attempted invalid action
- rejection decision
- violation or containment record

---

## Minimal Economic Model

The prototype economics model should be deliberately simple.

### Resources

```yaml
resources:
  compute_units: 10
```

### Allocation Rule

- citizens request compute
- scheduler grants based on:
  - priority
  - policy
  - fairness

This is enough to prove:
- economics affects execution
- scheduler decisions are reviewable

---

## Minimal Security Boundary Model

The prototype security behavior should remain simple. Full red/blue/purple
adversarial verification is important, but it is not required to prove Runtime
v2 core CSM semantics.

### Test Actor
- attempts one bounded invalid action pattern

### Kernel / Freedom Gate
- detects, rejects, and records

### Rule
- no live unsafe behavior
- all checks occur within policy-bounded sandboxing

This is enough to prove:
- governance is enforceable
- the polis boundary is not decorative
- later defensive verification can build on a real rejection path

---

## Minimal Freedom Gate Behavior

The prototype must show at least:

- allow
- refuse

Optional:
- defer

This is enough to prove:
- governance is not decorative
- citizens do not act directly

---

## Observability Requirements

A reviewer must be able to inspect:

- who acted
- what was proposed
- what the Freedom Gate decided
- why the scheduler chose one action over another
- what invalid action was attempted
- why it was rejected
- whether snapshot restore preserved continuity

---

## Suggested Demo Flow

The first end-to-end prototype demo should look like this:

```text
boot manifold
→ admit 2 worker citizens
→ run worker scheduling under scarce compute
→ route one governed action through Freedom Gate
→ attempt one invalid action and record rejection
→ sleep manifold
→ write snapshot
→ rehydrate
→ wake citizens
→ prove continuity in trace
```

---

## Acceptance Criteria

The prototype is successful when:

- manifold persists across sleep/wake
- no duplicate identity activation occurs
- trace is complete enough to reconstruct control decisions
- scheduler decisions are explainable via economic inputs
- Freedom Gate visibly mediates non-trivial actions
- invalid actions are rejected with durable reviewable artifacts
- restored world continues rather than restarting from zero

---

## Risks

### 1. Overbuilding
Trying to include full migration, learning, or market behavior too early.

### 2. Decorative Persistence
Saving superficial state without true identity continuity.

### 3. Decorative Governance
Adding a Freedom Gate label without actual mediation.

### 4. Decorative Security
Adding security language without a real rejected action, violation artifact, or
reviewable enforcement path.

---

## Implementation Strategy

### Phase 1
- manifold + kernel skeleton
- citizen registry
- trace append pipeline

### Phase 2
- scheduler + resource allocation
- Freedom Gate mediation

### Phase 3
- security boundary check and violation artifact

### Phase 4
- snapshot / restore
- end-to-end demo

---

## Summary

This prototype is the first proof that ADL Runtime v2 is not just a theory.

It should demonstrate a world that:

- persists
- governs
- allocates
- enforces boundaries
- remembers

> **If this prototype works, Gödel Agent Land has crossed from philosophy into engineering.**

---

## Next Steps

- map this prototype to concrete work packages
- derive first implementation issues
- define demo runner and artifact tree
- identify the smallest code path that proves all acceptance criteria
