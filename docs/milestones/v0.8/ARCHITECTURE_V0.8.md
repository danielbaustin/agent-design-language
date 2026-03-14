

# ADL v0.8 Architecture

**Status:** Draft (Product Cohesion Release)

v0.8 is the release where ADL stops being a collection of powerful subsystems and becomes a **coherent deterministic AI execution platform**.

This document defines how the following pillars fit together:

- Deterministic Workflow Runtime
- Replay / Auto‑Retry / Checkpointing
- ObsMem integration boundary (v0.75 foundation)
- Gödel Self‑Improvement Model
- Authoring Surfaces (Plain English → ADL)

---

# 1. Architectural Thesis

The entire v0.8 system rests on one non‑negotiable principle:

> **Determinism is the foundation.**

From determinism we get:

- Replay
- High‑fidelity traces
- Structured memory
- Reliable self‑evaluation
- Controlled self‑improvement

The dependency chain is intentional:

```
Deterministic Runtime
        ↓
Replay + Activation Log
        ↓
Structured Trace Bundles
        ↓
ObsMem (Bounded Evidence-Adjusted Retrieval)
        ↓
Gödel Evaluation + Mutation
        ↓
Authoring Refinement Loop
```

If determinism weakens, everything above it collapses.

---

# 2. System Layers (Outside‑In)

## 2.1 Authoring Surfaces v1

User-facing entry points:

- Plain English → Workflow
- CLI + IDE surfaces
- Template selection
- Policy injection (budget, retry, trust)

Outputs:

- Deterministic ADL document
- Explanation artifact
- Versioned plan

Authoring never bypasses determinism enforcement.

---

## 2.2 Compiler + Static Analysis

Responsibilities:

- NL → ADL graph generation
- Determinism validation
- Graph normalization
- Canonical step ordering
- Policy completion (no hidden defaults)

Outputs:

- Canonical plan.json
- Policy-complete workflow

---

## 2.3 Deterministic Workflow Runtime

Core responsibilities:

- DAG execution (fan‑out / fan‑in / conditionals)
- Stable hashing of:
  - inputs
  - outputs
  - captured tool events
- Idempotent step contracts
- Explicit tool boundary capture

Determinism contract:

> Given identical workflow version + identical inputs + identical captured tool events,
> the runtime must reproduce identical step outputs.

---

## 2.4 Durable Execution Substrate

Components:

- Activation event log (append‑only)
- Leasing scheduler
- Checkpoint + resume
- Failure classification
- Auto‑retry with policy
- Replay engine

Replay must:

- Rehydrate from activation log
- Respect captured boundary events
- Reproduce identical artifact set

---

## 2.5 Trace Bundles (Interface Layer)

Replay produces **trace bundles**.

These are the contract between runtime and higher systems.

Bundle contents (conceptual):

```
run/
  run.json
  plan.json
  activations/
  artifacts/
  derived/
```

Trace bundles are:

- Versioned
- Hash‑stable
- Indexable
- Replay‑sufficient

---

## 2.6 ObsMem Integration Boundary (v0.75 Foundation)

ObsMem is treated as an existing dependency from v0.75 and remains outside the v0.8 runtime expansion surface.

ObsMem capabilities available to v0.8:

- Structured index
- Deterministic retrieval over indexed evidence
- Derived feature summaries

ObsMem retrieval must:

- Always return citations
- Always explain score components
- Be deterministic given index state + query

ObsMem integration provides:

- Evidence retrieval over prior runs
- Failure similarity search
- Provider/policy analytics
- Stability metrics

See `OBSMEM_BAYES.md` for background model details.

---

## 2.7 Gödel Self‑Improvement Layer

Gödel operates strictly through artifacts.

Loop:

1. Retrieve evidence from ObsMem
2. Propose structured mutation
3. Evaluate via deterministic replay
4. Emit signed overlay (never mutate base)

Gödel cannot:

- Modify runtime in place
- Bypass replay
- Introduce nondeterministic learning

Self‑improvement is therefore:

> Explicit, inspectable, replay‑validated evolution.

---

# 3. Milestone Slicing

v0.8 is too large to land in one jump.

We split into:

## v0.75 — Deterministic Substrate

Must ship:

- Stable execution contract
- Activation log
- Replay engine
- Failure taxonomy
- Auto‑retry policy
- Trace bundle export v2
- ObsMem indexing + retrieval v1

v0.75 delivers infrastructure only.

---

## v0.8 — Product Cohesion

Must ship:

- Gödel mutation + evaluation harness
- Authoring Surfaces v1
- 2–3 flagship end‑to‑end demos

v0.8 delivers the integrated system story.

---

# 4. Flagship Demo Matrix (Acceptance Criteria)

v0.8 is complete when we can demonstrate:

### Demo 1 — Resilient Research Agent

- Generated from plain English
- Deterministic plan
- Simulated failure
- Auto‑retry
- Replay explains failure
- Retrieval surfaces consume v0.75 ObsMem evidence deterministically

### Demo 2 — Operational Intelligence

- Query: "Best provider under budget"
- Retrieval results are deterministic, cited, and replay-auditable
- Deterministic ranking

### Demo 3 — Gödel Improvement Loop

- Detect repeated failure pattern
- Propose mutation
- Replay baseline vs mutated
- Emit signed overlay

All demos must:

- Be reproducible
- Produce stable artifacts
- Pass CI regression

---

# 5. Cross‑Cutting Contracts

## 5.1 Determinism

- Stable hashing everywhere
- Canonical serialization
- Deterministic tie‑breaking in ranking

## 5.2 Provenance

- Every artifact is traceable
- Every retrieval is cited
- Every improvement is versioned

## 5.3 Versioning

Version and record:

- Retrieval weights
- Embedding model id
- Replay configuration
- Policy defaults

---

# 6. Risk Areas

- Hidden nondeterminism in tool boundaries
- Embedding instability across model upgrades
- Over‑coupling Gödel to retrieval heuristics
- Policy complexity overwhelming authoring UX

These must be actively constrained.

---

# 7. Architectural Identity

ADL v0.8 is not:

- A prompt wrapper
- A generic agent framework
- A RAG library

It is:

> A deterministic execution substrate capable of introspection and controlled self‑improvement.

That is the differentiator.

---

# 8. Next Steps

Implementation order recommendation:

1. Lock execution contract
2. Lock trace bundle schema
3. Consume v0.75 ObsMem retrieval/indexing surfaces
4. Add v0.8 retrieval integration reports
5. Build replay‑backed Gödel evaluation harness
6. Expose authoring loop

Everything else builds on this spine.

---

**End of v0.8 Architecture Draft**
