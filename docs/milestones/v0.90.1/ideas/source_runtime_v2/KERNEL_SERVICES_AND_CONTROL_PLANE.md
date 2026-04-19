

# KERNEL SERVICES AND CONTROL PLANE (Runtime v2)

## Status
Historical source draft - context only in the v0.90.1 tracked package

## Purpose

Define the **kernel services** and **control plane** for ADL Runtime v2 (Gödel Agent Land).

This document specifies:

- the always-on **kernel components**
- how invariants are enforced at runtime
- how citizens are scheduled and managed
- how snapshots, migration, and recovery are orchestrated

> The kernel is the system that makes the laws real.

---

## Core Principles

1. **Invariant-first execution**
   - All kernel actions must preserve invariants

2. **No silent state change**
   - Every mutation must be traceable

3. **Deterministic control, flexible execution**
   - Control plane is deterministic; execution may vary

4. **Separation of concerns**
   - Kernel (hard guarantees)
   - Staff agents (semantic reasoning)

---

## Kernel Overview


The kernel is composed of the following services:

1. Identity Guard
2. Chronosense / Clock Service
3. Trace System
4. Scheduler
5. Snapshot / Seal Manager
6. Migration Manager
7. Invariant Enforcement Engine
8. Capability Manager

---

## Kernel Event Pipeline

Define the canonical flow for all events through the kernel. This ensures deterministic control while allowing flexible execution.

```yaml
event_pipeline:
  1: receive_event
  2: assign_temporal_anchor (clock_service)
  3: validate_identity (identity_guard)
  4: validate_invariants (enforcement_engine)
  5: append_trace (trace_system)
  6: route_to_scheduler (scheduler)
```

### Rules

- Every event MUST pass through the pipeline
- No service may bypass trace append
- Invariant validation MUST occur before execution
- Temporal anchoring MUST occur before validation

### Failure Handling

If any stage fails:

- emit violation artifact
- halt or quarantine execution
- trigger enforcement response

---

# 1. Identity Guard

## Responsibility

Enforce identity invariants:

- no duplicate citizens (C-2)
- identity continuity (C-1)

## Functions

- register citizen
- validate uniqueness
- detect duplicate activation
- enforce single active instance

## Example

```yaml
identity_guard:
  check: duplicate_activation
  action: halt_and_resolve
```

---

# 2. Chronosense / Clock Service

## Responsibility

Maintain temporal invariants:

- monotonic ordering (T-2)
- temporal anchoring (T-1)

## Functions

- assign monotonic_order
- provide UTC timestamps
- maintain lifetime clocks

---

# 3. Trace System

## Responsibility

Maintain complete causal history:

- TR-1 full observability
- TR-2 decision visibility

## Functions

- append events
- validate event structure
- verify causal consistency
- emit violation artifacts

---

# 4. Scheduler

## Responsibility

Control execution of citizens and episodes.

## Functions

- schedule citizen actions
- enforce state constraints
- prevent illegal transitions

## Rules

- only `awake` citizens may execute
- `migrating` citizens must not execute
- all scheduled actions must originate from validated pipeline events

---

# 5. Snapshot / Seal Manager

## Responsibility

Create valid snapshots of the manifold.

## Functions

- initiate snapshot
- collect all required data
- validate completeness
- generate continuity proof

## Guarantees

- satisfies M-3 (replay sufficiency)
- produces valid snapshot bundle

---

# 6. Migration Manager

## Responsibility

Move manifolds across environments safely.

## Functions

- coordinate sleep
- trigger seal
- validate rehydration
- enforce MIG invariants

---

# 7. Invariant Enforcement Engine

## Responsibility

Central enforcement system for all invariants.

## Functions

- receive violation signals
- classify severity
- trigger response policies
- coordinate recovery

## Integration

Works with:

- Identity Guard
- Trace System
- Staff agents

---

# 8. Capability Manager

## Responsibility

Manage execution capabilities:

- model selection
- tool access
- limits

## Functions

- bind capabilities to citizens
- handle capability rebinding
- emit capability change events

---

# Control Plane

The control plane orchestrates all kernel services.

## Responsibilities

- coordinate lifecycle transitions
- route events to kernel services
- enforce global policies

---

## Control Plane Operations

### Citizen Lifecycle

- create → admit → activate
- sleep / wake
- suspend / resume
- migrate
- detach / reattach

---

### Snapshot Flow

1. quiesce execution
2. flush memory
3. validate invariants
4. create snapshot

---

### Migration Flow

1. sleep manifold
2. seal snapshot
3. transfer
4. rehydrate
5. validate invariants
6. wake system

---

### Violation Handling

1. detect violation
2. classify severity
3. invoke response
4. record artifact
5. attempt recovery
- pipeline stage of failure must be recorded

---

## Kernel vs Staff Agents

### Kernel

- fast
- deterministic
- enforces hard invariants

### Staff Agents

- slower
- interpretive
- enforce higher-order reasoning

Example:

- kernel detects duplicate identity
- staff agent determines cause and resolution strategy

---

## Failure Modes

Kernel must detect:

- duplicate citizens
- broken ordering
- invalid snapshots
- illegal state transitions

Response:

- halt execution
- freeze manifold
- trigger recovery

---

## Minimal Implementation Slice

First implementation MUST include:

- identity guard
- clock service
- trace system
- basic scheduler
- snapshot manager

---

## Summary

The kernel:

- enforces invariants
- maintains world integrity
- enables persistence and prepares the later migration path

> Without the kernel, the world cannot remain coherent.

---

## Next Steps

- implement identity guard
- implement trace append pipeline
- build scheduler prototype
- integrate snapshot manager
- connect enforcement engine
- prove one invalid action is rejected and recorded
