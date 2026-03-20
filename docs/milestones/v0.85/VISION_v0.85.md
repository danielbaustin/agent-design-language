# ADL Vision — v0.85

## Overview

Version 0.85 is the milestone where ADL evolves from an experimental deterministic workflow substrate into a more complete platform for **trustworthy agent engineering**.

Earlier milestones established the core foundations:

- deterministic workflow execution
- explicit cards and structured artifacts
- reproducible runs and traceability
- early Gödel-style scientific improvement loops
- ExperimentRecord / Evidence / Evaluation surfaces

v0.85 is intended to **complete and integrate the current architectural doc set** so that ADL is no longer described as a collection of promising ideas, but as a coherent system with identifiable layers, bounded cognitive primitives, and a clearer path to v0.9.

This milestone is therefore not just about polish. It is about consolidation and architectural completion across the current planning set, especially in:

- reasoning graphs
- bounded affect modeling
- structured prompt and authoring architecture
- human-in-the-loop design
- execution substrate evolution
- bounded adaptive execution
- milestone rigor and review discipline

The purpose of v0.85 is to make ADL more **usable, inspectable, scalable, and conceptually complete** before the project crosses into v0.9.

---

## Why v0.85 Matters

v0.85 is the point where ADL begins to look less like a workflow runner and more like a disciplined architecture for bounded reasoning systems.

Many agent frameworks stop at:

- prompt orchestration
- tool invocation
- best-effort planning
- loosely structured memory

ADL is aiming at something more rigorous:

- deterministic execution
- explicit workflow structure
- inspectable evidence and review artifacts
- reasoning representations that can be persisted and analyzed
- bounded adaptive improvement loops
- human oversight as a first-class design feature

In that sense, v0.85 is the milestone where ADL’s **cognitive substrate becomes more explicit**.

Within the tracked v0.85 milestone docs, the authoritative loop definition now lives in [COGNITIVE_LOOP_MODEL_v0.85.md](/tmp/adl-wp-929/docs/milestones/v0.85/COGNITIVE_LOOP_MODEL_v0.85.md).

It does not claim sentience, autonomy, or unrestricted self-modification. Instead, it assembles the practical primitives needed for:

- bounded scientific reasoning
- explicit internal evaluation
- experiment selection and refinement
- more reliable multi-step agent behavior

---

## Core Goals

v0.85 advances ADL in five major areas:

1. Execution substrate
2. Authoring surfaces
3. Trust and verification
4. Cognitive substrate
5. Operational maturity

These five areas are not independent. Together they define the current architectural stage of ADL.

## Milestone Shape

The canonical v0.85 structure is now a four-sprint, twenty-five-work-package program:

1. Sprint 1: milestone reorganization and execution substrate
2. Sprint 2: authoring surfaces and review tooling
3. Sprint 3: Gödel, affect, reasoning graphs, and AEE/runtime progress
4. Sprint 4: demos, quality gate, review, release, and next-milestone planning

This structure exists so the milestone produces visible capability, not only planning refinement. In particular:

- `#886` is the umbrella reorganization issue until the issue graph is aligned.
- `#674` is the canonical queue/checkpoint/steering issue.
- Gödel issues `#748` through `#752` are central milestone work, not optional extras.
- the provisional generated issue set `#866` through `#882` is useful scaffolding, but not the final source of truth.

---

## Architectural Shape of ADL in v0.85

By the end of this milestone, the ADL architecture should read as a coherent stack:

### 1. Execution Layer

The deterministic runtime, queueing, replay, checkpointing, and distributed execution surfaces that make workflows dependable.

### 2. Workflow Layer

Cards, specs, prompts, DAG structure, and explicit task boundaries that make agent systems understandable and governable.

### 3. Cognitive Layer

Reasoning graphs, observational memory, hypotheses, experiment records, and other structured representations that allow the system to carry forward and refine thought.

### 4. Evaluation Layer

Affect signals, evaluation surfaces, experiment ranking, and bounded internal prioritization mechanisms that help the system choose what to examine, retry, or improve.

### 5. Adaptive Layer

The Adaptive Execution Engine (AEE), which coordinates bounded adaptation while preserving reproducibility, reviewability, and policy control.

This layered picture should become clear across the entire v0.85 doc set.

---

## 1. Execution Substrate

v0.85 expands ADL’s runtime capabilities so workflows can scale beyond single-machine experimentation.

Key objectives:

- deterministic execution queue
- checkpointing and resumable runs
- cluster / distributed execution
- work leasing and task claims
- retry and backoff policies
- bounded adaptive execution surfaces
- clearer transition away from older runtime-oriented execution assumptions

These capabilities move ADL from a workflow specification system toward a **deterministic execution substrate for agent systems**.

The runtime must guarantee:

- reproducibility
- deterministic ordering
- bounded mutation
- auditable artifacts
- inspectable execution state

This part of the milestone also clarifies an important direction in the current docs: ADL is shifting away from loosely defined experimental execution framing toward a more disciplined execution model.

---

## 2. Authoring Surfaces

ADL must become easier to use without sacrificing rigor.

v0.85 strengthens the authoring model through:

- structured prompt improvements
- Prompt Spec completeness and validation
- HTML authoring interface work
- real editor surfaces for issue prompts and input/output cards
- workflow visualization
- linting and validation for ADL artifacts
- stronger connections between prompts, cards, review surfaces, and editing/review GPT assets

The goal is to move from raw file editing toward **guided authoring surfaces** that still preserve ADL’s explicitness.

This matters because ADL is not just a runtime. It is also a language and methodology for building dependable agent workflows.

The authoring layer must help users:

- construct valid systems quickly
- understand workflow intent clearly
- preserve deterministic structure
- support review and correction before execution

---

## 3. Trust and Verification

A central design principle of ADL is **verifiable inference**.

Agent systems must not merely produce answers. They must produce **evidence-backed, inspectable results** whose origin and evaluation can be reviewed later.

v0.85 strengthens this pillar with:

- artifact provenance tracking
- reproducible run bundles
- evidence surfaces
- deterministic replay
- review artifacts
- milestone and release discipline
- stronger reviewer tooling and validation expectations

This work supports the broader principle of **dependable execution**.

Systems built with ADL must behave in ways that are:

- inspectable
- reproducible
- auditable
- reviewable
- bounded by explicit policy and structure

This is essential not only for enterprise adoption, but for ADL’s deeper claim that agent engineering should become a more rigorous discipline.

---

## 4. Cognitive Substrate

v0.85 continues development of ADL’s Gödel-style improvement architecture, but in a more concrete and integrated form.

The focus remains on **bounded scientific reasoning**, not unrestricted self-modification.

Key capabilities in this area include:

- hypothesis registry
- experiment ranking and evaluation
- improved ObsMem retrieval for experiments
- bounded mutation mechanisms
- structured reasoning representations
- better integration between memory, reasoning, and evaluation
- concrete progress on Gödel issues `#748` through `#752` as the first meaningful hypothesis-engine milestone

### Reasoning Graphs

One of the most important goals of v0.85 is to elevate reasoning graphs into a first-class architectural surface.

Reasoning graphs provide a structured representation for:

- claims
- evidence
- hypotheses
- dependencies
- contradictions
- refinement paths
- experiment outcomes

This is important because ADL should not rely only on token-stream reasoning or ephemeral chain-of-thought-like behavior. It should increasingly rely on explicit, inspectable reasoning structures that can be stored, reviewed, compared, and improved.

Reasoning graphs are therefore central to ADL’s longer-term cognitive architecture.

### Bounded Affect Modeling

v0.85 also introduces deeper work on a **bounded affect model**.

This is not an attempt to simulate human psychology for its own sake. It is a bounded engineering mechanism for representing:

- internal priorities
- salience
- tension between goals
- confidence or concern signals
- evaluation pressure within ongoing reasoning

These signals can influence experiment selection, prioritization, and adaptive behavior inside the Gödel–Hadamard–Bayes loop.

Within the v0.85 doc set, the distinction should become clearer:

- `EMOTION_MODEL.md` describes the broader conceptual and philosophical framing
- `AFFECT_MODEL_v0.85.md` describes the bounded implementation-oriented framing
- `AFFECTIVE_REASONING_MODEL.md` describes how affect interacts with reasoning and evaluation

Taken together, these documents define a bounded evaluation layer rather than anthropomorphic claims.

v0.85 should also produce a minimal working affect substrate, not only a design framing. The milestone should leave behind:

- a minimal working affect engine with explicit state and update rules
- emitted artifacts or traces showing affect state in use
- explicit linkage between affect, reasoning graphs, and Gödel-style hypothesis work
- a bounded demo where affect changes ranking, evaluation, prioritization, or related reasoning behavior in a legible way

---

## 5. Operational Maturity

To support real-world use, v0.85 must improve operational tooling and milestone discipline.

Important targets include:

- Card Reviewer GPT stabilization
- working issue/card editor surfaces
- improved review workflows
- better CI validation surfaces
- clearer milestone documentation
- stronger milestone checklists
- improved release-plan coherence across the doc set

The **Card Reviewer GPT** becomes an important part of the development workflow, helping enforce:

- deterministic artifacts
- schema correctness
- consistent card structure
- evidence-backed review findings
- better alignment between planning artifacts and implementation artifacts

This reduces review overhead and improves engineering velocity while reinforcing ADL’s commitment to explicit structure.

---

## Human-in-the-Loop as a First-Class Principle

One of the quieter but important themes of the v0.85 docs is that ADL is not designed to remove human judgment from the system.

Instead, ADL is designed to create better collaboration between:

- human judgment
- explicit workflow structure
- machine reasoning
- evidence-backed review

This means human-in-the-loop design is not a fallback or a temporary concession. It is a core architectural principle.

Humans remain responsible for:

- defining goals
- setting policy
- reviewing outputs
- correcting trajectories
- determining acceptable risk and quality thresholds

ADL should make this collaboration more structured, not less visible.

---

## The Adaptive Execution Engine (AEE)

The Adaptive Execution Engine becomes a central focus of v0.85.

Previous releases defined the architecture but deferred deeper implementation.

v0.85 advances AEE with:

- clearer strategy loop integration
- policy surfaces for retry and adaptation
- bounded mutation hooks
- improved experiment lifecycle support
- stronger alignment with reasoning and evaluation layers

The AEE is responsible for ensuring that learning and adaptation remain:

- bounded
- reproducible
- observable
- reviewable
- subordinate to explicit policy constraints

This keeps the system aligned with ADL’s core design philosophy: adaptation must increase capability without dissolving trust.

## Proof Through Demos

The milestone must prove its claims through multiple runnable bounded demos. At minimum, the demo program should include:

- a steering/queueing/checkpoint proof surface
- a HITL/editor/review workflow proof surface
- a Gödel hypothesis-engine proof surface
- an affect-engine proof surface
- an integrated affect-plus-Gödel proof surface

The purpose of these demos is not marketing polish. They are milestone evidence that the runtime, authoring, trust, and cognitive claims are backed by working behavior.

---

## Milestone Context

v0.8 represented a major architectural milestone by establishing much of the deterministic and experimental substrate.

v0.85 is intended to complete and tighten the current doc set so that the next stage is built on stronger conceptual footing.

In practical terms, this milestone should leave the project with:

- a clearer execution model
- a stronger reasoning model
- a bounded evaluation model
- a more coherent authoring model
- better operational and review discipline

The goal is to have nearly all foundational capabilities in place by **v0.9**, after which the project can shift toward smaller, more incremental releases.

---

## Long-Term Direction

ADL is being designed as an infrastructure layer for trustworthy agent systems.

Its long-term goals include:

- dependable execution
- verifiable inference
- structured agent workflows
- bounded adaptive learning
- durable reasoning artifacts
- explicit human governance

These principles aim to move agent engineering beyond ad-hoc orchestration toward a more rigorous and reliable discipline.

The long-term ambition is not merely to make agents more powerful. It is to make them:

- more governable
- more legible
- more correctable
- more scientifically improvable

---

## Summary

v0.85 is the milestone where ADL becomes more architecturally explicit.

It strengthens:

- the execution substrate
- the authoring surfaces
- the trust and verification model
- the reasoning and evaluation layers
- the operational discipline around the project itself

Most importantly, it integrates the current body of work into a more coherent whole.

By the end of v0.85, ADL should read not as a collection of interesting mechanisms, but as a serious and increasingly complete platform for **trustworthy, inspectable, bounded agent engineering**.

These improvements prepare the system for the next phase of development leading toward **v0.9 and eventually 1.0**.
