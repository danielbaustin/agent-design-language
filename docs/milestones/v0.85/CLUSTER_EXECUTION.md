# Distributed / Cluster Execution — v0.85

**Status:** Draft (planning)

This document describes how ADL evolves from a single-machine deterministic runtime into a **distributed execution environment** while preserving ADL’s central thesis:

> **Determinism is non-negotiable.**

For v0.85, cluster execution is part of the broader **execution-substrate strengthening** work.
It sits alongside:

- dependable execution
- checkpoint / resume semantics
- replay and provenance
- bounded retry / adaptive execution work
- future trust-policy enforcement across execution boundaries

Related backlog: **#339 Distributed cluster execution**.

---

## 1. Purpose and milestone framing

Cluster execution matters because ADL is not just a local workflow runner.
The platform is intended to support:

- deterministic execution at larger scale
- durable runs with restart / retry behavior
- trusted remote or isolated executors
- future Gödel / ObsMem / AEE workloads that benefit from concurrent execution

However, v0.85 is still a **bounded maturity milestone**.
So this document does **not** commit ADL to a full distributed platform in the current release.
Instead, it defines the architecture direction, the determinism constraints, and the minimum substrate shape needed so later implementation work does not drift.

---

## 2. Goals and non-goals

### Goals

1. **Distributed execution without weakening determinism**
   - Multiple workers may execute steps, but the outcome must still be replayable from artifacts.

2. **Stable scheduling contract**
   - Execution may be parallel where the DAG allows it, but scheduling decisions must remain reproducible.

3. **Explicit trust boundaries**
   - Remote or isolated workers must be treated as trust-boundary crossings, not just background threads.

4. **Operationally realistic rollout**
   - Start with a small, inspectable execution model before attempting multi-host scale-out.

5. **Compatibility with dependable execution**
   - Cluster execution must align with checkpointing, retries, replay bundles, and provenance surfaces.

### Non-goals (v0.85)

- A full serverless platform replacement.
- Multi-tenant isolation comparable to major cloud providers.
- Exactly-once guarantees for arbitrary external side effects.
- A fully production-hardened multi-region control plane.

---

## 3. Determinism constraints in a distributed system

Distributed systems introduce nondeterminism through:

- race conditions (which worker picks up which step)
- time (timeouts, clock skew, lease expiry)
- retries (duplicate or overlapping work)
- non-idempotent side effects
- network failures and partial commits

ADL’s strategy is:

1. **The activation log is the source of truth.**
2. **All tool boundaries are captured or replay-gated.**
3. **Scheduling decisions that affect execution are recorded as artifacts.**
4. **Tie-breaking is deterministic.**
5. **Workers are execution agents, not scheduling authorities.**

In other words:

> We do not try to make the world deterministic.
> We make ADL’s **interpretation of the world** deterministic.

---

## 4. Execution model

### 4.1 Core primitives

- **Plan**: canonical workflow graph with stable ordering and stable step ids.
- **Activation**: immutable record of a step attempt.
- **Lease**: a time-bounded claim by a worker to execute a specific activation.
- **Work item**: `(run_id, step_id, attempt)` plus required inputs and policy.
- **Artifact bundle**: durable captured record of execution inputs, outputs, logs, and tool-boundary events.
- **Artifact store**: durable store for activation bundles and derived trace assets.

### 4.2 Central rule

> A step may run on any worker, but given the same inputs and captured boundary events it must produce the same activation artifact bundle.

That rule is more important than wall-clock ordering.

### 4.3 Coordinator authority

The coordinator is the only component allowed to:

- decide which steps are ready
- issue leases
- create new attempts
- accept or reject commits for an activation

Workers do **not** autonomously change the global schedule.

---

## 5. Minimal cluster architecture for v0.85 planning

This document remains intentionally conservative.

The near-term architecture should assume a **small, inspectable coordinator/worker system** whose protocol can later support multi-host execution, even if current implementation remains bounded.

### 5.1 Components

1. **Coordinator** (control plane)
   - Creates runs
   - Maintains the activation log
   - Computes ready steps from the canonical DAG
   - Issues leases
   - Applies deterministic tie-break rules

2. **Work queue**
   - Durable queue of ready work items
   - Candidate backends:
     - SQLite for bounded local / single-node control-plane work
     - Postgres for later small-cluster evolution

3. **Workers** (data plane)
   - Poll for work
   - Claim leases
   - Execute steps in the sandbox contract
   - Emit activation bundles
   - Acknowledge completion or failure

4. **Artifact store**
   - Stores:
     - activation inputs / outputs
     - tool boundary event captures
     - logs
     - signatures / provenance markers
     - derived metrics and replay artifacts

### 5.2 Message flow (conceptual)

```text
Author / CLI
   │
   ▼
Coordinator ──► Activation Log (append-only)
   │
   ├──► Work Queue (ready steps)
   │
   └──► Artifact Store (run + activation bundles)

Workers (N)
   │
   ├─ poll queue
   ├─ claim lease
   ├─ execute sandbox
   └─ write activation bundle + ack
```

---

## 6. Protocol direction

v0.85 does not require a final production network protocol, but it **does** require that the protocol shape be explicit and versioned.

### 6.1 Protocol requirements

- Versioned top-level messages
- Canonical serialization
- Stable activation identity via `(run_id, step_id, attempt)`
- Explicit lease and lease-deadline semantics
- Signed or attributable worker identity
- Explicit result / failure reporting

### 6.2 Minimum message set

Representative message types:

- `RegisterWorker { worker_id, public_key, capabilities }`
- `PollWork { worker_id, max_items }`
- `LeaseGranted { run_id, step_id, attempt, lease_id, lease_deadline, input_refs, policy }`
- `ReportResult { lease_id, status, activation_bundle_ref, bundle_hash, signature }`
- `ReportFailure { lease_id, classification, error_ref, logs_ref }`
- `Heartbeat { worker_id, lease_ids[] }`

### 6.3 Canonical serialization

Payload serialization must be canonical so that:

- hashing is stable
- signatures are stable
- replay artifacts are reviewable

JSON in canonical form or CBOR in canonical form are both plausible.
The key point is not the format name but the **stability guarantee**.

---

## 7. Scheduling semantics

### 7.1 Deterministic readiness

Given:

- the canonical plan
- activation log state

The set of ready steps must be computable deterministically.

### 7.2 Parallelism is allowed; authority is not distributed

Where multiple steps are ready simultaneously, any worker may execute them **after coordinator lease issuance**.

Determinism is preserved by recording:

- lease issuance order or deterministic lease keys
- activation completion records
- duplicate / rejected commit events where applicable

Replay does **not** need to reproduce wall-clock timing.
It needs to reproduce:

- the set of activations
- the recorded boundary events
- the resulting artifact bundles

### 7.3 Tie-breakers

When the coordinator must choose among multiple ready steps, tie-break by canonical ordering over:

`(run_id, step_id, attempt)`

Additional priority fields are allowed only if they are:

- explicit
- artifact-derived
- deterministic

---

## 8. Retry semantics in a cluster

Retries are especially easy to get wrong in distributed systems.

Rules:

1. **Only the coordinator creates new attempts.**
2. **Workers never autonomously retry; they report failure.**
3. **Lease expiry may create overlapping work, but overlap resolution is activation-log governed.**
4. **Retry policy must remain bounded and inspectable.**

Duplicate execution handling:

- If two workers execute the same `(run_id, step_id, attempt)` due to lease races:
  - the first valid commit wins
  - later commits are rejected
  - the duplicate event is recorded as an artifact

This is necessary for deterministic debugging and for future AEE / policy learning.

---

## 9. Trust and signing model

Cluster execution expands the trust surface.

Minimum requirements for the architecture direction:

- Workers have runtime identity (keypair or equivalent signing identity)
- Activation bundles are signed or otherwise strongly attributable
- The coordinator verifies result provenance before accepting commits
- Policy may restrict which workers may execute which steps

This links directly to ADL’s broader **dependable execution** and **verifiable inference** themes:

- trusted executor as policy dimension
- attributable execution evidence
- reviewable provenance at the execution boundary

---

## 10. Sandboxing and remote I/O

Workers must run steps under the same sandbox contract regardless of host.

Relevant controls include:

- filesystem sandbox / restricted root
- network policy
- tool allow / deny policy
- secret redaction rules
- normalized environment variables where required

To preserve determinism:

- timestamps and random seeds must be controlled where relevant
- external I/O must cross explicit tool-boundary capture points
- hidden host-local state must not influence replayable outcomes

---

## 11. Relationship to dependable execution and verifiable inference

Cluster execution is not an isolated feature.
It is part of the trust story for ADL.

### 11.1 Dependable execution

Cluster mode must still support:

- replayable execution history
- resumability
- bounded retry behavior
- explicit failure semantics

### 11.2 Verifiable inference

If reasoning work or model-driven steps run on distributed workers, the resulting artifacts must still support:

- provenance
- attributable execution origin
- reviewable evidence bundles
- replay / inspection of reasoning-adjacent outputs where applicable

This matters even more once Gödel / AEE / reasoning-graph work begins using richer execution traces.

---

## 12. Relationship to existing substrates

ADL should resist the temptation to rebuild a general-purpose serverless platform from scratch.

ADL’s differentiation is not just “run functions at scale.”
It is:

- deterministic execution
- replay
- provenance
- controlled adaptive loops

Candidate future substrates:

- Kubernetes + Jobs
- Nomad
- a bounded custom coordinator / worker model

The recommended direction is:

> Define ADL’s coordinator / worker contract first, then optionally target another substrate later.

---

## 13. Milestone slicing

### v0.85 (this milestone)

- Architecture direction documented
- Determinism constraints made explicit
- Coordinator / worker / lease model clarified
- Trust-boundary and provenance requirements identified
- Relationship to dependable execution, retries, and replay clarified

### v0.9+

- Bounded cluster MVP implementation
- Multi-process / multi-worker execution path
- Queue / coordinator backend hardening
- Worker registration and signing flows
- Optional multi-host evolution once artifact and replay semantics are stable

This keeps v0.85 focused on **design clarity and interface stability**, not premature scale claims.

---

## 14. Acceptance tests (must-haves for future implementation)

1. **Deterministic replay across execution modes**
   - Run in distributed / clustered mode
   - Replay in a bounded local mode
   - Artifact bundles match or differ only in explicitly allowed metadata

2. **Lease race safety**
   - Force lease expiry and overlapping claims
   - Verify duplicate commits are recorded and rejected deterministically

3. **Trust policy gating**
   - Disallow an untrusted worker from executing a protected step

4. **Checkpoint / resume compatibility**
   - Interrupt coordinator or worker process
   - Resume without corrupting activation history

5. **ObsMem ingestion**
   - Cluster-produced trace bundle is ingestible and queryable

---

## 15. Open questions

- What is the default bounded backend for the coordinator log / queue?
  - SQLite first, or Postgres-first for small-cluster realism?
- What is the default artifact store?
  - local filesystem vs S3-compatible storage
- What is the first implementation boundary?
  - single-host multi-process only, or immediately protocol-ready for multi-host evolution?
- Which signing / identity mechanism best fits the existing security envelope work?

---

## Summary

Distributed / cluster execution is necessary for ADL’s long-term direction, but it must be approached conservatively.

The central rule remains:

> A step may execute anywhere, but ADL must still be able to explain, replay, and verify what happened.

For v0.85, the right outcome is a clear architecture and stable execution contract, not inflated claims of scale.