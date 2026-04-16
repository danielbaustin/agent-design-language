

# ADL Skill Composition Model

## Status
Implemented

## Purpose

Define how **skills compose** in ADL to produce reliable, bounded, and reviewable behavior.

This document establishes:
- the core composition primitives
- execution semantics
- trace expectations
- safety boundaries

This is a foundational piece of the ADL runtime.

---

## Implemented Surfaces

WP-08 makes the model concrete through a contract module and identity proof hook:

- Contract module: `adl/src/skill_composition_model.rs`
- Identity proof hook: `adl identity skill-composition --out .adl/state/skill_composition_model_v1.json`
- Output schema: `skill_composition_model.v1`
- Upstream substrate schema: `operational_skills_substrate.v1`

The implementation defines canonical composition primitives, DAG validation rules, trace/replay requirements, a bounded `arxiv-paper-writer` composition shape, and downstream boundaries for WP-09 and WP-13.

Scope boundary:
- this model defines composition contracts and proof artifacts
- it does not implement governed delegation/refusal integration for WP-09
- it does not package the final manuscript workflow for WP-13
- it does not permit unbounded loops, hidden prompt recursion, or dynamic graph mutation after planning

---

## Core Principle

> Complex behavior must be constructed from explicit, bounded skill compositions-not from implicit prompt nesting or hidden loops.

Composition is:
- explicit
- structured
- traceable
- reviewable

---

## Why Composition Matters

Without composition:
- skills are isolated
- behavior becomes ad hoc
- trace becomes ambiguous
- review becomes impossible

With composition:
- behavior becomes structured
- execution becomes deterministic (or bounded where non-deterministic)
- trace reflects real reasoning structure
- systems become auditable

---

## Composition Primitives (v1)

ADL defines a small set of canonical composition primitives.

### Primitive Execution Rule

All composition primitives operate over **skill invocations**, not skill definitions.

Each primitive MUST:
- create explicit invocation boundaries
- emit a trace span per invocation
- define clear input/output relationships between nodes

> Composition operates on executions, not abstractions.

### 1. Sequence

Execute skills in order.

```
A -> B -> C
```

Properties:
- output of A may feed into B
- execution is linear
- failure handling must be explicit

---

### 2. Parallel

Execute multiple skills concurrently.

```
A || B || C
```

Properties:
- no implicit shared mutable state
- outputs are collected and merged
- merge strategy must be defined

---

### 3. Validation Gate

Run a validation skill after another skill.

```
A -> validate(A)
```

Properties:
- validation does not mutate original output (unless explicitly allowed)
- produces pass/fail or structured critique

---

### 4. Conditional Branch

Select execution path based on condition.

```
if (condition) -> A
else -> B
```

Properties:
- condition must be explicit
- branch decision must be traceable

---

### 5. Fallback

Attempt primary skill, then fallback if it fails.

```
A or_else B
```

Properties:
- failure definition must be explicit
- fallback must be bounded

---

### 6. Bounded Retry

Retry a skill with constraints.

```
retry(A, max=2)
```

Properties:
- retry count must be bounded
- retry reason must be recorded

---

### 7. Adjudication

Compare multiple outputs and select or synthesize.

```
adjudicate(A, B, C)
```

Properties:
- requires comparison criteria
- decision must be explainable

---

## Composition Graph

Skill compositions form a **directed acyclic graph (DAG)**.

Properties:
- nodes = skill invocations
- each node must have a unique invocation ID
- nodes map directly to trace spans
- edges = data or control flow
- cycles are not allowed in v1

### DAG Principle

> ADL compositions must remain acyclic in v1 to preserve determinism and trace clarity.

Future versions may introduce controlled loops.

---

## Data Flow

Each edge in the composition graph carries data.

Data may include:
- raw outputs
- structured artifacts
- references (IDs into trace/artifacts)

### Data Flow Principle

> Data passed between skills must be explicit and inspectable.

No hidden state.

---

## Execution Semantics

### Determinism

ADL aims for deterministic orchestration, even if model outputs are stochastic.

This means:
- structure is deterministic
- control flow is deterministic
- non-determinism is isolated to skill execution
- composition outcomes must be reproducible given the same inputs and invocation sequence

---

### Failure Handling

Failure must be explicit.

Examples:
- skill returns failure state
- validation fails
- timeout

Composition must define:
- whether to stop
- whether to retry
- whether to fallback

---

### Termination

A composition must have a clear termination condition.

Examples:
- final skill completes
- validation passes
- retry limit reached

---

## Trace Model

Each composition produces a structured trace.

At minimum:
- composition ID
- graph structure
- node executions
- invocation IDs per node
- parent/child relationships between nodes
- inputs and outputs
- decision points (branches, retries, adjudication)
- termination reason

### Trace Principle

> The trace must mirror the composition graph exactly.

This enables:
- replay
- audit
- debugging
- learning

---

## Review Model

A reviewer should be able to:
- see the composition structure
- inspect each skill execution
- understand decisions (branch, retry, adjudication)
- evaluate correctness and quality

### Review Principle

> Composition must be human-legible, not just machine-executable.

---

## Safety Boundaries

Composition introduces risk if not bounded.

Key requirements:
- no unbounded loops
- explicit retry limits
- explicit tool permissions per skill
- no hidden cross-skill state mutation

### Safety Principle

> Composition must not introduce implicit autonomy.

---

## Composition vs Prompt Chaining

Traditional prompt chaining:
- implicit
- unstructured
- not traceable
- not reviewable

ADL composition:
- explicit
- structured
- traceable
- reviewable

> ADL replaces prompt chaining with a formal execution graph.

---

## Composition and Learning

Compositions are the substrate for learning.

Because:
- they are explicit
- they are traceable
- they are reviewable

GHB and future systems can:
- analyze compositions
- propose refinements
- suggest new compositions

But in v1:
- compositions are authored, not synthesized

---

## Design Implications

### 1. Prefer simple primitives over complex hidden logic

### 2. Prefer explicit control flow over emergent behavior

### 3. Prefer bounded execution over open-ended loops

### 4. Prefer inspectable data flow over implicit context

---

## Execution Considerations (v1+)

This document defines the conceptual model of composition.

Concrete implementation responsibilities are defined in feature and runtime documents.

However, this model MUST translate into executable behavior.

The system MUST ensure:
- compositions are representable as explicit DAGs
- each node corresponds to a skill invocation
- all control flow decisions are traceable
- all data flow is explicit and inspectable

The system SHOULD ensure:
- composition graphs are serializable and replayable
- composition structures are human-readable
- validation, retry, and fallback behaviors are standardized

The system WILL ensure (via feature work):
- integration with the Skill Execution Protocol
- alignment with the Operational Skills Substrate
- trace emission that mirrors composition structure exactly

> This document defines the model. Feature documents define the implementation.

---

## Bounded Writer Composition

The bounded `arxiv-paper-writer` composition is represented as an explicit DAG:

```text
load_source_packet -> draft_outline -> draft_sections
draft_sections -> citation_gap_review -> emit_review_packet
draft_sections -> claim_boundary_review -> emit_review_packet
emit_review_packet -> human_publication_gate
```

The composition keeps manuscript work reviewable by requiring:
- a source packet before drafting
- citation-gap review before packet finalization
- claim-boundary review before packet finalization
- a human publication gate before any external submission

This is a WP-08 skill/composition surface. WP-13 owns the later three-paper manuscript packet and reviewer-facing publication workflow evidence.

---

## Proof Hooks

Contract validation:

```bash
cargo test --manifest-path adl/Cargo.toml skill_composition_model
```

Reviewer artifact:

```bash
adl identity skill-composition --out .adl/state/skill_composition_model_v1.json
```

The generated artifact is deterministic for the same source tree and should mirror the composition primitives and DAG constraints in this document.

---

## Summary

ADL skill composition defines how skills combine into reliable behavior.

It is:
- graph-based
- explicit
- bounded
- traceable

> Composition is the executable structure of reasoning in ADL.

---

## Related Documents

- `docs/milestones/v0.89.1/features/OPERATIONAL_SKILLS_SUBSTRATE.md`
- `docs/milestones/v0.89.1/features/ADVERSARIAL_EXECUTION_RUNNER.md`
- `docs/milestones/v0.89.1/DEMO_MATRIX_v0.89.1.md`
- `adl/src/skill_composition_model.rs`
- `adl/src/operational_skills_substrate.rs`
