

# MANIFOLD AND SNAPSHOT SPEC (Runtime v2)

## Status
Historical source draft - context only in the v0.90.1 tracked package

## Purpose

Define the **data model, serialization contract, and lifecycle semantics** for:

- the **manifold** (persistent cognitive spacetime)
- the **snapshot / seal / rehydrate** process

This document is the bridge from architecture → implementation.

---

## Core Requirement

> A sealed snapshot MUST be sufficient to reconstruct a valid manifold that satisfies all Runtime v2 invariants.

---

## 1. Manifold Data Model

### 1.1 Manifold Identity

```yaml
manifold:
  manifold_id: csm-01
  created_utc: <timestamp>
  version: v2
  state: running | sleeping | sealed | rehydrating
```

---

### 1.2 Clock Stack (Chronosense)

```yaml
clocks:
  utc_now: <timestamp>
  monotonic_counter: <int>
  lifetime_start_utc: <timestamp>
```

Invariant links:
- T-1 Temporal Anchoring
- T-2 Monotonic Ordering

---

### 1.3 Citizen Registry

```yaml
citizens:
  - citizen_id: ga-004
    class: godel_agent
    status: awake | sleeping | migrating
    birth_utc: <timestamp>
    memory_namespace: mem://ga-004
    obligations: [...]
    capability_envelope_ref: cap://ga-004
```

Invariant links:
- C-1 Identity Continuity
- C-2 No Duplicate Activation
- C-3 Memory Persistence

---

### 1.4 Memory (ObsMem)

```yaml
memory:
  namespaces:
    mem://ga-004:
      entries:
        - event_id: e-123
          temporal_anchor: {...}
          payload: {...}
```

Requirements:
- temporally anchored
- causally traceable

---

### 1.5 World Trace

```yaml
trace:
  events:
    - event_id: e-123
      monotonic_order: 10234
      utc: <timestamp>
      type: decision | mutation | migration | violation
      actor: ga-004
      data: {...}
```

Requirements:
- MUST represent complete causal history OR
- MUST reference an immutable external trace store with verifiable integrity

Invariant links:
- TR-1 Full Observability
- TR-2 Decision Visibility

---

### 1.6 Capability Envelopes

```yaml
capabilities:
  cap://ga-004:
    model: gpt-5
    tools: [fs, http]
    limits:
      max_tokens: 128k
```

Requirement:
- must be versioned and recorded

---

### 1.7 Ownership Map

```yaml
ownership:
  invariants:
    C-2:
      owner: kernel.identity_guard
      backup: staff.identity_auditor
```

Critical:
- ownership MUST be serialized

---

## 2. Snapshot / Seal Format

### 2.1 Snapshot Structure

A snapshot is a **complete, portable bundle**.

```yaml
snapshot:
  manifest:
    manifold_id: csm-01
    snapshot_id: snap-0001
    created_utc: <timestamp>
    monotonic_cutoff: 10500

  manifold: {...}
  citizens: {...}
  memory: {...}
  trace:
    mode: embedded | external
    embedded: {...}           # full trace up to cutoff (if embedded)
    external_ref:
      uri: <immutable_store_uri>
      range: [0, monotonic_cutoff]
      integrity_hash: <digest>
  capabilities: {...}
  ownership: {...}
```

---

### 2.2 Snapshot Requirements

A valid snapshot MUST:

- include all active citizens
- include all unresolved obligations
- include trace up to monotonic_cutoff
- include ownership model
- be internally consistent

- MUST include either full trace OR verifiable external reference

Invariant links:
- M-3 Replay Sufficiency
- MIG-1 Snapshot Integrity

---

### 2.3 Continuity Proof

Each snapshot SHOULD include:

```yaml
continuity_proof:
  previous_snapshot: snap-0000
  hash: <digest>
```

Purpose:
- chain-of-custody for identity and state

---

## 3. Rehydration Contract

### 3.1 Rehydrate Steps

1. Load snapshot
2. Validate structure
3. Validate invariants
4. Restore manifold
5. Rebind capabilities
6. Restore citizens (sleeping state)
7. Resume runtime

---

### 3.2 Validation Rules

Must verify:

- no duplicate citizens
- monotonic ordering intact
- trace continuity valid
- ownership present

- trace integrity hash must verify against embedded or external source

Failure = abort rehydration

---

### 3.3 Capability Rebinding

If environment changes:

```yaml
capability_rebind:
  from: ollama.local
  to: openai.remote
  changes:
    model: llama3 -> gpt-5
```

Requirement:
- must be recorded in trace

Invariant link:
- MIG-5 Capability Rebinding Transparency

---

## 4. Lifecycle Semantics

### 4.1 Sleep

- freeze event intake
- flush memory
- finalize trace segment
- mark citizens sleeping

---

### 4.2 Seal

- produce snapshot
- validate completeness
- emit seal event

---

### 4.3 Transfer

- snapshot moved externally

---

### 4.4 Rehydrate

- restore state
- validate invariants

---

### 4.5 Wake

- kernel starts
- staff agents start
- citizens transition to awake

---

## 5. Failure Conditions

### Snapshot Invalid If:

- missing citizens
- broken ordering
- missing ownership
- inconsistent trace

### Rehydrate Abort If:

- identity duplication detected
- invariant violation detected

---

## 6. Minimal Implementation Slice

First prototype MUST support:

- 1 manifold
- 1–2 citizens
- snapshot → disk
- rehydrate → same machine

Then extend to:

- cross-machine migration

---

## Summary

This spec defines:

- what must be persisted
- how it must be structured
- how it must be restored

> **If the snapshot is correct, the world survives.**

---

## Next Steps

- define binary / JSON encoding
- implement snapshot writer
- implement rehydration validator
- integrate invariant checks
- build first same-machine sleep/snapshot/rehydrate demo
- defer cross-machine migration demo until the later identity/migration band
