

# Reasoning Graph Schema — v0.85

## Purpose

This document defines the **conceptual schema direction for reasoning graphs** used in ADL planning.

The reasoning graph is intended to support:

- structured reasoning traces
- hypothesis exploration
- belief revision
- deterministic replay
- adaptive reasoning policies

In the broader architecture this schema will support:

- the **Adaptive Execution Engine (AEE)**
- the **Gödel hypothesis engine**
- **affective reasoning signals**
- **observational memory (ObsMem)**

The v0.85 milestone introduces only the **schema direction**, not a full runtime implementation.

---

# Design Goals

The reasoning graph must satisfy several constraints.

### Deterministic

Reasoning graphs must support deterministic replay.

Given the same inputs, model outputs, and state, the reasoning trace must be reproducible.

### Inspectable

The graph must be human‑readable and reviewable.

This enables:

- debugging
- external review
- verification of reasoning

### Composable

Graphs must support composition of reasoning steps, including nested reasoning processes.

### Persistent

Reasoning graphs must be serializable and storable in **ObsMem**.

### Compatible with affect signals

Nodes must support attachment of **affective reasoning signals**.

---

# Core Graph Elements

The reasoning graph consists of several primary element types.

| Element | Description |
|------|------|
| Node | A reasoning step or hypothesis |
| Edge | A relationship between reasoning steps |
| Hypothesis | A candidate belief or strategy |
| Evaluation | Evidence supporting or contradicting a hypothesis |
| Revision | A change in belief state |

---

# Node Schema

Nodes represent reasoning units.

Example conceptual schema:

```yaml
node:
  id: node_001
  type: hypothesis
  description: "candidate strategy"

  belief:
    probability: 0.64

  affect:
    confidence: 0.55
    tension: 0.22
    curiosity: 0.48

  metadata:
    created_at: "timestamp"
    source: "planner"
```

Nodes may represent:

- reasoning steps
- hypotheses
- evaluation results

---

# Edge Schema

Edges represent reasoning transitions.

Example:

```yaml
edge:
  from: node_001
  to: node_002
  relation: "derived_from"
```

Possible edge types include:

- `derived_from`
- `supports`
- `contradicts`
- `refines`

Edges capture the **structure of reasoning**.

---

# Hypothesis Records

Hypotheses represent candidate explanations or strategies.

Example schema:

```yaml
hypothesis:
  id: h_001
  description: "alternative reasoning strategy"

  belief:
    probability: 0.51

  status:
    state: active
```

Possible states:

- active
- rejected
- confirmed

---

# Evaluation Records

Evaluations provide evidence regarding hypotheses.

Example:

```yaml
evaluation:
  id: eval_01
  target: h_001

  evidence:
    source: "model_output"
    confidence: 0.7
```

Evaluations allow the graph to represent **reasoning evidence**.

---

# Revision Events

Reasoning systems may update beliefs over time.

Example revision record:

```yaml
revision:
  target: h_001
  previous_belief: 0.51
  new_belief: 0.67

  reason: "new evidence"
```

These records preserve **belief lineage**.

---

# Relationship to Observational Memory (ObsMem)

Reasoning graphs may be persisted in **ObsMem**.

This allows:

- long‑term reasoning history
- auditability
- retrospective analysis

Graphs may be stored as serialized structures.

---

# Relationship to Affective Reasoning

Each node may include an **affect vector**.

Example:

```yaml
affect:
  confidence: 0.4
  tension: 0.7
  curiosity: 0.3
```

These signals influence reasoning policies.

---

# Relationship to the Gödel Hypothesis Engine

The Gödel agent explores alternative reasoning strategies.

Reasoning graphs provide:

- hypothesis tracking
- belief revision
- reasoning lineage

This structure allows the system to **reason about reasoning**.

---

# Determinism Considerations

To preserve deterministic replay:

- node IDs must be stable
- graph serialization must be canonical
- evaluation inputs must be recorded

The graph must never depend on hidden state.

---

# v0.85 Scope

The v0.85 milestone introduces only:

- schema direction
- conceptual node/edge structures
- integration points with AEE and affect models

No runtime graph engine is required yet.

---

# Future Work (v0.9+)

Planned work includes:

- graph storage format
- graph traversal APIs
- hypothesis ranking algorithms
- reasoning replay tooling

These features will support a full **Gödel‑style reasoning engine**.

---

# Summary

The reasoning graph schema defines the structure used to represent reasoning processes in ADL.

It captures:

- reasoning steps
- hypotheses
- evidence
- belief revisions

This schema provides the foundation for future adaptive reasoning capabilities.