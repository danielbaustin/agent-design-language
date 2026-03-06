

# ADL v0.8 — EPIC Mapping

**Status:** Draft (Living Mapping Document)

This document maps the v0.8 Architecture (`ARCHITECTURE_V0.8.md`) to concrete GitHub EPICs and issue tracks.

It answers:

> For each architectural layer, which EPIC(s) and issue clusters implement it?

---

# 1. Architecture → EPIC Matrix

| Architecture Layer | Responsibility | EPIC / Issue Cluster |
|-------------------|---------------|----------------------|
| Deterministic Workflow Runtime | Stable execution contract, hashing, idempotence | EPIC-A (Runtime Core) |
| Durable Execution Substrate | Activation log, replay, auto-retry | EPIC-A (Replay + Substrate) |
| Trace Bundles | Stable export contract for indexing | EPIC-A + ObsMem B1 |
| ObsMem | Indexing + retrieval + Bayesian ranking | EPIC-B (ObsMem v1) |
| Gödel Layer | Proposal → evaluation → overlay | EPIC-C (Godel v1) |
| Authoring Surfaces | NL → ADL + refinement loop | #517 (Authoring Surfaces v1) |

---

# 2. EPIC-A — Deterministic Runtime + Replay (v0.75 foundation)

Implements:

- Architecture §2.3 Deterministic Workflow Runtime
- Architecture §2.4 Durable Execution Substrate
- Architecture §2.5 Trace Bundles

Sub-epics (conceptual):

- A1 — Deterministic Execution Contract
- A2 — Activation Log + Replay Runner
- A3 — Checkpoint / Leasing Scheduler
- A4 — Failure Taxonomy + Policy Injection

Milestone target:

> v0.75 (Infrastructure Complete)

ObsMem and Gödel depend on this EPIC.

---

# 3. EPIC-B — ObsMem v1 (Operational Memory)

Implements:

- Architecture §2.6 ObsMem
- Bayesian model defined in `OBSMEM_BAYES.md`

Sub-issues (created via `create_obsmem_issues_v0.8.sh`):

- B1 — Trace Export Bundle v2
- B2 — Index + Retrieval API v1
- B3 — Smart RAG over runs
- B4 — Analytics + reporting

Dependencies:

- Requires stable Trace Bundle contract (EPIC-A complete)

Milestone target:

> v0.8 (Product Cohesion)

---

# 4. EPIC-C — Gödel Self-Improvement v1

Implements:

- Architecture §2.7 Gödel Layer

Sub-issues (created via `create_godel_agent_issues_v0.8.sh`):

- G1 — Deterministic Improvement Proposal Engine
- G2 — Replay-backed Evaluation Harness
- G3 — Signed Overlay Emission
- G4 — Artifact Model + Storage Layout

Dependencies:

- Requires Replay engine (EPIC-A)
- Strongly benefits from ObsMem retrieval (EPIC-B)

Milestone target:

> v0.8

---

# 5. EPIC-D — Authoring Surfaces v1

Primary issue:

- #517 — [v0.8][EPIC] Authoring Surfaces v1

Implements:

- Architecture §2.1 Authoring Surfaces
- Architecture §2.2 Compiler + Static Analysis

Child issues (created via `create_authoring_issues_v0..8.sh`):

- AUTH-1 — NL→ADL Compiler v1
- AUTH-2 — Interactive Refinement Loop
- AUTH-3 — Policy Defaults + Budgeting
- AUTH-4 — Examples + Demo Workflows

Dependencies:

- Deterministic Runtime (EPIC-A)
- ObsMem retrieval (EPIC-B)
- Replay for refinement debugging

Milestone target:

> v0.8

---

# 6. Dependency Graph (EPIC Level)

```
EPIC-A (Deterministic Substrate)
        ↓
EPIC-B (ObsMem)
        ↓
EPIC-C (Godel)
        ↓
EPIC-D (Authoring Surfaces)
```

Important nuance:

- Authoring can ship basic NL→ADL without Gödel.
- Gödel is the last layer to stabilize.

---

# 7. Milestone Strategy

## v0.75 — Infrastructure Lock

Complete:

- EPIC-A (fully)
- Trace Bundle schema freeze

Do NOT start:

- Bayesian ranking
- Gödel proposal evaluation

---

## v0.8 — Product Integration

Complete:

- EPIC-B
- EPIC-C
- EPIC-D
- Demo matrix acceptance

---

# 8. Definition of Done (v0.8)

All EPICs satisfy:

- Deterministic replay reproducibility
- Traceable artifact lineage
- Stable evidence ranking
- Signed improvement overlays
- End-to-end demo reproducibility

---

# 9. Maintenance Rule

This file must be updated whenever:

- An EPIC is split or renamed
- An architectural layer changes
- A milestone boundary moves

It is the contract between architecture and issue tracker.

---

**End of EPIC Mapping v0.8**