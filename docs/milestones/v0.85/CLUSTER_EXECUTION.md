# Distributed / Cluster Execution (v0.8 Planning)

**Status:** Draft (planning)

This document describes how ADL evolves from a single-machine deterministic runtime into a **distributed execution environment** while preserving ADL’s central thesis:

> **Determinism is non-negotiable.**

Distributed execution is a v0.8 concern because:

- Replay + auto-retry imply durable execution state.
- Remote tools/providers imply a trust boundary.
- Real workloads require concurrency beyond one host.
- ObsMem and Gödel become dramatically more valuable when runs are executed at scale.

Related backlog: **#339 [Distributed cluster execution]**.

---

## 1. Goals and non-goals

### Goals

1. **Distributed execution without weakening determinism**
   - Multiple workers may execute steps, but the outcome must be replayable.
2. **Stable scheduling contract**
   - Execution order may be parallel where allowed, but decisions must be reproducible from artifacts.
3. **Secure remote workers**
   - Signed workers, signed bundles, and explicit trust policies.
4. **Operational simplicity**
   - Start with a “good enough” cluster mode (LAN / single region) before multi-region.

### Non-goals (v0.8)

- A full serverless platform replacement.
- Multi-tenant isolation comparable to major cloud providers.
- Exactly-once execution of external side effects (we instead capture tool boundaries and replay deterministically).

---

## 2. Determinism constraints in a distributed system

Distributed systems introduce nondeterminism via:

- race conditions (which worker picks up which step)
- time (timeouts, clock skew)
- retries (duplicate work)
- non-idempotent side effects

ADL’s strategy:

1. **The activation log is the source of truth.**
2. **All tool boundaries are captured or replay-gated.**
3. **Scheduling decisions are recorded as artifacts.**
4. **Tie-breaking is deterministic.**

In other words:

> We don’t try to make the world deterministic.
> We make ADL’s *interpretation of the world* deterministic.

---

## 3. Execution model

### 3.1 Core primitives

- **Plan**: canonical workflow graph (stable ordering, stable step ids).
- **Activation**: immutable record of a step attempt.
- **Lease**: a time-bounded claim by a worker to execute a specific activation.
- **Work item**: `(run_id, step_id, attempt)` plus required inputs and policy.
- **Artifact store**: durable store for captured boundary events + outputs.

### 3.2 Central rule

> A step may run on any worker, but it must produce the same activation artifact bundle given the same inputs and captured tool events.

---

## 4. Minimal cluster architecture (v0.8)

This is intentionally conservative.

For v0.8, “distributed” explicitly means multi-process on a single host.

The coordinator/worker protocol must be transport-agnostic so that it can later
run over a network, but the only supported transport in v0.8 is:

- **Unix domain sockets (stream)**

This provides a real process boundary, real serialization, and realistic
failure modes (disconnects, partial writes, restarts) without introducing
multi-host complexity.

### 4.1 Components

1. **Coordinator** (control plane)
   - Creates runs
   - Maintains the activation log
   - Computes ready steps from DAG
   - Issues leases

2. **Work queue**
   - A durable queue of ready work items
   - Can be implemented with:
     - SQLite (single-node),
     - Postgres (small cluster),
     - or a queue service later.

3. **Workers** (data plane)
   - Poll for work
   - Claim a lease
   - Execute step in a sandbox
   - Emit activation bundle
   - Acknowledge completion

4. **Artifact store**
   - Stores:
     - activation inputs/outputs
     - tool boundary event captures
     - logs
     - derived metrics
   - v0.8 can start with local filesystem paths or S3-compatible storage.

### 4.2 Message flow (conceptual)

```
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

### 4.3 Protocol v1 (local, Unix domain sockets)

v0.8 defines a minimal, versioned protocol over Unix domain sockets.

Transport characteristics:

- Stream sockets (Unix domain)
- Length-prefixed frames (u32 or u64)
- Canonical serialization (JSON or CBOR, canonical form required)
- Explicit protocol version field in every top-level message

Core message types (minimum viable set):

- `RegisterWorker { worker_id, public_key, capabilities }`
- `PollWork { worker_id, max_items }`
- `LeaseGranted { run_id, step_id, attempt, lease_id, lease_deadline, input_refs, policy }`
- `ReportResult { lease_id, status, activation_bundle_ref, bundle_hash, signature }`
- `ReportFailure { lease_id, classification, error_ref, logs_ref }`
- `Heartbeat { worker_id, lease_ids[] }`

Determinism requirements:

- `(run_id, step_id, attempt)` is the immutable activation key.
- The coordinator is the only authority that creates new attempts.
- All scheduling decisions that affect execution (e.g., lease issuance)
  must be derivable from artifacts in the activation log.
- Payload serialization must be canonical to ensure stable hashing.

---

## 5. Scheduling semantics

### 5.1 Deterministic readiness

Given:

- canonical plan
- activation log state

The set of ready steps must be computable deterministically.

### 5.2 Parallelism is allowed, ordering is recorded

Where multiple steps are ready simultaneously, any worker may take them.

Determinism is preserved by recording:

- lease issuance order (or a deterministic tie-break key)
- activation completion records

Replay does not need to reproduce wall-clock ordering; it needs to reproduce:

- the set of activations
- the captured tool boundary events

### 5.3 Tie-breakers

When the coordinator has to choose among multiple ready steps for lease issuance, tie-break by:

`(run_id, step_id, attempt)` in canonical ordering.

Workers do not choose the schedule; they only claim ready items.

---

## 6. Retry semantics in a cluster

Retries are easy to get wrong in distributed systems.

Rules:

1. **Only the coordinator creates new attempts.**
2. **Workers never autonomously retry; they report failure.**
3. **Lease expiry may create duplicate work; duplicates are resolved by activation log rules.**

Duplicate execution handling:

- If two workers execute the same `(run_id, step_id, attempt)` due to lease races:
  - the first commit wins
  - later commits are rejected and recorded as duplicates

This must be explicit in artifacts so debugging remains deterministic.

---

## 7. Trust and signing model

Distributed execution expands the attack surface.

v0.8 requirements:

- Workers have a **runtime identity** (keypair / signing id)
- Activation bundles are signed by the worker
- Coordinator verifies signatures before accepting results
- Policy can restrict which workers may execute which steps

This links directly to the v0.7/v0.8 security envelope work:

- “trusted executor” as a first-class policy dimension

---

## 8. Sandboxing and remote IO

Workers must run steps under the same sandbox contract regardless of host:

- filesystem sandbox (restricted root)
- network policy
- tool allow/deny
- secret redaction

To preserve determinism:

- environment variables are normalized
- timestamps and random seeds are controlled
- external IO occurs only through tool boundary capture

---

## 9. Relationship to "serverless" engines

We should resist the temptation to rebuild OpenWhisk/KNative wholesale.

ADL’s differentiation is not “run functions at scale.” It is:

- deterministic execution
- replay
- provenance
- controlled learning loops

That said, we can integrate with existing substrates later.

### 9.1 Candidate substrates

- **Kubernetes + Jobs** (simple, explicit, operationally common)
- **KNative** (serverless semantics; may fight determinism by emphasizing elasticity + eventing)
- **Nomad** (simpler than k8s, good for batch)
- **OpenWhisk** (feature-rich; heavy; rewrite is a multi-year effort)

v0.8 approach:

> Build a minimal coordinator + worker model that can *optionally* target k8s as an execution backend.

---

## 10. Milestone slicing

This feature is big. Recommended split:

### v0.75 (foundation)

- Activation log + replay schema frozen
- Worker identity + signature verification (minimum)
- Artifact store abstraction

### v0.8 (cluster MVP)

- Coordinator + worker protocol (transport-agnostic by design)
- Lease + queue semantics
- Local “mini-cluster” mode (single host, multi-process only):
  - one coordinator process
  - N worker processes (same machine)
- Explicit non-goal for v0.8:
  - no LAN / multi-host workers yet

### v0.85 / v0.9 (scale)

- LAN / multi-host worker support
- k8s backend
- multi-host discovery
- multi-tenant hardening

---

## 11. Acceptance tests (must-haves)

1. **Deterministic replay across hosts**
   - Run in cluster mode
   - Replay on a single host
   - Artifacts match

2. **Lease race safety**
   - Intentionally force lease expiries
   - Verify duplicates are recorded and ignored deterministically

3. **Trust policy gating**
   - Disallow an untrusted worker from running a step

4. **ObsMem ingestion**
   - Cluster-produced trace bundle is ingestible and queryable

---

## 12. Open questions

- What is the v0.8 default backend for the coordinator log/queue?
  - SQLite (single host) vs Postgres (small cluster)
- What is the artifact store default?
  - filesystem vs S3-compatible
- When (if ever) do we move from Unix domain sockets (Protocol v1) to
  a networked transport (e.g., gRPC over TCP) for multi-host support?

---

**End of Distributed / Cluster Execution (v0.8 Planning)**