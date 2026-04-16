

# ADL Learning Model

## Status
Draft

## Purpose

Define the **learning architecture of ADL**.

This document establishes how learning occurs across the system, separating:
- model-level learning (provider-owned)
- system-level learning (ADL-owned)
- cognitive learning (agent-owned)

It clarifies responsibilities, boundaries, and interactions between these layers.

---

## Core Principle

> Learning in ADL is not a single mechanism. It is a **layered system** spanning model, system, and cognition.

ADL deliberately shifts learning away from opaque model training toward:
- explicit system composition (skills)
- observable cognitive refinement (GHB)

---

## The Three Layers of Learning

### 1. Model-Level Learning (Aptitude Layer)

**Owner:** Provider  
**Mechanisms:**
- pretraining
- fine-tuning
- LoRA adapters

**Characteristics:**
- slow to evolve
- opaque
- non-deterministic internally
- not portable across providers

**Definition:**
Model-level learning defines the **aptitudes** of a model:
- reasoning ability
- language capability
- generalization
- pattern recognition

**Key Constraint:**
ADL does **not control** this layer.

---

### 2. System-Level Learning (Skill Layer)

**Owner:** ADL  
**Mechanisms:**
- skills
- tools
- structured workflows
- prompt templates
- deterministic orchestration

**Definition:**
A **skill** is an externalized, reusable capability that extends or constrains model behavior.

> A skill is how ADL teaches a model to do something without modifying the model itself.

**Characteristics:**
- explicit
- composable
- deterministic (within ADL constraints)
- portable across providers

---

### Skill ↔ Aptitude Relationship

> Skills compensate for or amplify model aptitudes.

Examples:

- Weak reasoning → structured reasoning skill  
- Weak memory → retrieval skill  
- Weak planning → workflow skill  

**Implication:**

> System design can compensate for model limitations.

This is a core ADL thesis.

---

### 3. Cognitive Learning (GHB Layer)

**Owner:** Agent  
**Mechanisms:**
- Gödel agent
- Hadamard-style insight
- Bayesian updating
- trace-driven reflection

**Definition:**

Cognitive learning is **learning through reasoning over experience**, not training.

**Characteristics:**
- iterative
- trace-dependent
- self-improving
- grounded in execution history

---

## Layer Interaction Model

| Layer | Mechanism | Scope | Control |
|------|----------|------|--------|
| Model | Training / LoRA | Aptitude | Provider |
| System | Skills | Behavior | ADL |
| Cognitive | GHB | Strategy | Agent |

---

## Boundaries

### ADL DOES:
- define skills
- compose workflows
- enable cognitive refinement
- measure aptitude

### ADL DOES NOT:
- modify model weights
- rely on hidden training
- depend on provider-specific tuning

---

## Why This Matters

Without this separation:

- learning becomes opaque
- improvements are not portable
- debugging is impossible
- system behavior cannot be governed

With this model:

- learning becomes inspectable
- behavior becomes composable
- improvement becomes systematic
- agents become evolvable

---

## Design Implications

### 1. Prefer Skills Over Fine-Tuning

Fine-tuning:
- is opaque
- breaks portability
- cannot be inspected

Skills:
- are explicit
- reusable
- composable

---

### 2. Trace is Required for Cognitive Learning

GHB depends on:
- execution trace
- observable outcomes
- structured feedback

Without trace, cognitive learning collapses.

---

### 3. Memory Must Be Grounded

ObsMem must:
- derive from trace
- preserve provenance
- support reasoning

Memory is not free-form text.

---

### 4. Aptitudes Must Be Measured

Aptitudes are not assumed.

They must be:
- tested
- benchmarked
- compared

---

## Future Extensions (Not v1)

- automated skill synthesis via GHB
- skill marketplaces
- adaptive skill selection
- cross-agent learning

---

## Summary

ADL defines a **three-layer learning system**:

1. **Model learning** → provides raw capability (aptitude)  
2. **System learning** → structures behavior (skills)  
3. **Cognitive learning** → improves reasoning (GHB)  

> ADL shifts intelligence from the model into the system and the agent.

This makes learning:
- explicit
- inspectable
- composable
- governable

---

## Related Documents

- SKILL_MODEL.md  
- SKILL_COMPOSITION_MODEL.md  
- APTITUDE_MODEL.md  
- OPERATIONAL_SKILLS_SUBSTRATE.md  
- COGNITIVE_LOOP_MODEL.md  
