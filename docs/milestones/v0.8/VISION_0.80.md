# ADL v0.8 - Vision (Godel + Authoring Cohesion Release)

**Status:** Draft  
**Depends on:** v0.75 (Deterministic Substrate + ObsMem v1)

v0.8 is the release where ADL becomes a **cohesive deterministic intelligence system**.

If v0.75 locks the substrate and memory, v0.8 turns that foundation into:

- Controlled self-improvement (Gödel v1)
- Safe, ergonomic authoring (Authoring Surfaces v1)
- End-to-end deterministic refinement loops

v0.8 does **not** expand infrastructure surface area.  
It integrates and proves the layers built in v0.75.

---

# One-Sentence Thesis

**v0.8 enables replay-validated self-improvement and safe workflow authoring on top of a frozen deterministic substrate and memory layer.**

---

# What Changed Since Earlier Planning

We redistributed scope to control milestone size and reduce ceremony overhead.

- v0.75 now includes:
  - Deterministic execution contract freeze
  - Activation log + replay
  - Trace bundle v2
  - Failure taxonomy
  - ObsMem v1 (index + retrieval)

- v0.8 now focuses strictly on:
  - Godel self-improvement layer (EPIC-C)
  - Authoring Surfaces v1 (EPIC-D / #517)

- v0.85 will handle:
  - Distributed / cluster execution

This keeps v0.8 product-focused instead of infrastructure-heavy.

---

# Scope (What Ships in v0.8)

## EPIC-C - Godel v1 (Controlled Self-Improvement)

Gödel operates entirely through deterministic artifacts.

Core loop:

1. Retrieve evidence from ObsMem.
2. Propose a structured mutation (overlay, not base mutation).
3. Evaluate via deterministic replay.
4. Emit signed improvement overlay.

Constraints:

- No in-place runtime mutation.
- No hidden learning.
- Every improvement must be replay-validated.
- Every improvement must be inspectable and versioned.

Minimum deliverables:

- Mutation proposal engine (structured)
- Replay-backed evaluation harness
- Overlay artifact format
- Signed overlay emission
- Comparison report (baseline vs mutated)

---

## EPIC-D - Authoring Surfaces v1

Authoring Surfaces v1 makes ADL usable without writing raw YAML.

Layers:

1. Plain English -> Authoring IR (JSON)
2. Deterministic IR -> Canonical ADL YAML
3. Safe defaults injection
4. Pattern registry integration

Rules:

- LLMs never emit runtime YAML directly.
- All output passes through Authoring IR validation.
- Canonical YAML emission must be deterministic.

Minimum deliverables:

- Authoring IR schema
- Deterministic IR → YAML compiler
- CLI guided workflow generator
- Constrained NL→IR generator (experimental)
- 2-3 example workflows authored via the new surface

---

# Non-Goals (Explicitly Deferred)

These belong to v0.85 or later:

- Cluster / distributed execution
- Multi-host coordination
- Kubernetes backend
- Advanced memory training or fine-tuning
- Major runtime refactors

v0.8 is about integration, not expansion.

---

# Integration Acceptance Matrix (v0.8)

v0.8 is complete when we can demonstrate:

## Demo 1 - Author -> Run -> Replay

- Author workflow from plain English.
- Emit canonical ADL YAML.
- Run deterministically.
- Replay and confirm artifact identity.

## Demo 2 - Memory-Assisted Improvement

- Run workflow.
- Ingest trace bundle.
- Retrieve similar failures.
- Propose mutation.
- Replay baseline vs mutation.
- Emit signed overlay.

## Demo 3 - Controlled Evolution

- Apply overlay.
- Re-run workflow.
- Show improvement delta in metrics (latency/cost/failure rate).
- All evidence cited.

All demos must:

- Produce stable artifact trees.
- Be reproducible from docs.
- Pass CI determinism checks.

---

# Contracts v0.8 Must Respect

v0.8 may not weaken any v0.75 guarantees.

Specifically:

1. Deterministic execution contract remains frozen.
2. Trace bundle schema remains versioned and stable.
3. ObsMem retrieval remains deterministic.
4. Improvement overlays are append-only.
5. No hidden state outside artifacts.

---

# Definition of Done (DoD)

v0.8 is DONE when:

- Gödel v1 can propose and evaluate structured mutations.
- Authoring Surfaces v1 emits canonical, deterministic YAML.
- Replay validates all improvements.
- Overlay artifacts are signed and versioned.
- Demo matrix runs from a fresh checkout.

---

# Roadmap Context

- v0.75 - Substrate + Memory (frozen foundation)
- v0.8  - Godel + Authoring (cohesion layer)
- v0.85 - Cluster execution (scale layer)

The architectural identity remains:

> ADL is a deterministic execution substrate capable of introspection and controlled self-improvement.

v0.8 proves that thesis.

---

**Related planning:** see `../v0.85/CLUSTER_EXECUTION.md` for deferred cluster/distributed execution scope.
