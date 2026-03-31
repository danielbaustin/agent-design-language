# ADL v0.86 Vision

## Metadata
- Project: `ADL (Agent Design Language)`
- Milestone: `v0.86`
- Version: `0.86`
- Date: `2026-03-27`
- Owner: `Agent Logic, Inc.`
- Related issues: `#882, #1071, #1072, #1074, #1075`

## Purpose
Define the milestone-level vision for v0.86: what changes at this stage, why it matters, and which strategic pillars it advances.

v0.86 is the milestone where ADL begins turning its emerging cognitive architecture into implementable, inspectable system surfaces. The emphasis is on establishing the first *coherent, bounded cognitive system*—including loop, signals, arbitration, and bounded execution—so later milestones can build on a real foundation rather than a partial control layer.

## How To Use
- Read this as a milestone vision, not a full design spec.
- Use it to understand the intended shape of v0.86 before reading the design, WBS, or sprint docs.
- Treat the five goal areas as the milestone’s major strategic buckets, not as promises that every later-roadmap concept will ship here.
- Use this document to separate what v0.86 must make real from what remains deferred to v0.87+.

## Overview

Version `0.86` is the milestone where `ADL` evolves from a stabilized execution and authoring substrate into the first explicit **bounded cognitive system**.

This release should establish or strengthen the foundation for:

- bounded cognitive loop control
- cognitive signals (instinct and affect)
- inspectable arbitration and fast/slow routing
- bounded execution (AEE-lite)
- evaluation signals and termination conditions
- minimal frame adequacy and reframing
- initial memory participation (ObsMem-lite)
- explicit agency surfaces such as the Freedom Gate
- local demo surfaces that prove the system is real

`0.86` focuses on **making the first complete cognitive loop real, bounded, and testable**.

The goal is to make the project more useful to:

- system builders working on agent behavior rather than only workflow plumbing
- reviewers who need inspectable proof surfaces for cognitive claims
- later milestone work on instinct, convergence, affect, identity, and governance

This milestone should strengthen the architectural or strategic pillars of:

- cognitive control
- bounded agency
- inspectable decision-making
- local proof surfaces
- roadmap discipline

v0.86 is therefore a milestone of architectural consolidation: it should give ADL a believable cognitive core without collapsing later milestones back into one oversized design bucket.

---

# Core Goals

`0.86` advances `ADL` in five major areas:

1. Cognitive loop and stack clarity
2. Arbitration, routing, and bounded control
3. Early agency and Freedom Gate surfaces
4. Tooling and workflow discipline for the new control layer
5. Milestone legibility, local demos, and implementation truth

---

# 1. Cognitive loop and stack clarity

`0.86` improves `cognitive loop and stack clarity` so the project can express a real control model rather than scattered cognitive ideas.

Key objectives:

- make the cognitive loop explicit and inspectable
- clarify the relationship between loop, stack, and agent control surfaces
- reduce ambiguity between conceptual docs and implementation-bearing docs
- define bounded inputs and outputs for the first cognitive layer
- explicitly align the loop with the canonical cognitive stack and loop documents for this milestone
- ensure compatibility with fast/slow thinking and cognitive arbitration surfaces
- integrate cognitive signals (instinct and affect) into the loop as first-class inputs

These capabilities move the project toward **a cognitive architecture that can actually be built and reviewed**.

The system or product should guarantee:

- one coherent cognitive loop story for the milestone
- one coherent stack/layer story for the milestone
- no competing definitions of the core cognitive control path

---

# 2. Arbitration, routing, and bounded execution

`arbitration, routing, and bounded control` must improve without sacrificing determinism, inspectability, or milestone scope discipline.

`0.86` introduces or improves:

- cognitive arbitration as a first-class surface
- fast/slow routing as an explicit control mechanism
- clearer decision points inside the control loop
- bounded control behavior that can be demonstrated and reviewed
- explicit integration with the fast/slow thinking model and cost-aware cognition
- local demo surfaces that make routing and arbitration reviewable
- bounded execution loops (AEE-lite)
- evaluation signals and termination conditions

The goal is to move from `implicit or scattered cognitive decisions` toward **explicit, inspectable, bounded routing and arbitration**.

These changes should help users:

- understand why one path of reasoning or action was selected over another
- review the control logic of an agent without needing to infer it from prompts alone

---

# 3. Early agency and Freedom Gate surfaces

A central principle of `ADL` is **that agency must be structured, reviewable, and bounded**.

The project must not merely `run model calls in sequence`. It must `make room for explicit choice, control, and accountable constraint`.

`0.86` strengthens this pillar with:

- a clearer Freedom Gate surface
- the first practical boundaries around agent choice
- stronger relationship between control, constraint, and action
- groundwork for later instinct and governance layers without prematurely collapsing them into this milestone

This work supports the broader principle of **reasoned and inspectable agency rather than opaque automation**.

`0.86` must also ensure that agency is not treated as a metaphor but as an implementable structure, grounded in:

- the Freedom Gate as a real decision boundary
- bounded candidate selection and action control inside the loop
- traceable commitment events as part of execution
- integration of candidate generation, evaluation, and commitment within the loop

The result should make the project more:

- coherent
- reviewable
- defensible

---

# 4. Cognitive signals, evaluation, and adaptation

`0.86` introduces the first integrated form of **cognitive signals and adaptive behavior**.

Key capabilities:

- instinct and affect signals as inputs to the loop
- evaluation signals (progress, contradiction, failure)
- minimal frame adequacy scoring
- bounded reframing when the current approach is insufficient
- observable influence of signals on arbitration and execution

The goal is to move beyond static routing toward **adaptive, signal-driven cognition**, while remaining bounded and inspectable.

These capabilities must be visible in traces, artifacts, and demos.

---

# 5. Tooling and workflow discipline for the cognitive system

`0.86` continues development of `the ADL tool and workflow surface`.

The focus remains on **making the new cognitive-control layer executable, testable, and inspectable**, not adding broad new authoring or UI scope.

Key capabilities:

- normalize active timestamp and related workflow surfaces where needed for the new milestone
- retire or simplify obsolete workflow commands that no longer fit the intended path
- keep input/output card discipline aligned with actual implementation work
- ensure the control-layer docs can be translated into issues, cards, and demos cleanly

This ensures that arbitration, loop execution, and agency surfaces can be demonstrated concretely rather than remaining purely conceptual.

This milestone should help the project better represent or support:

- disciplined implementation sequencing
- cleaner handoff from planning docs to execution work
- less friction between roadmap intent and actual delivery

These improvements should guide the system toward `a milestone workflow that supports cognitive architecture work without recreating v0.85 planning drag`.

---

# 6. Milestone legibility and implementation truth

To support real-world progress, `0.86` must improve `roadmap truthfulness and milestone legibility`.

Important targets include:

- ensuring v0.86 delivers a complete but bounded cognitive system
- preserving later milestone space for instinct, convergence, reasoning graphs, affect, identity, and governance
- ensuring feature docs, workplans, and examples are clearly differentiated
- avoiding the pattern where concept papers are silently treated as implemented features
- inclusion of local agent demos that exercise the cognitive loop, arbitration, and Freedom Gate
- keeping the milestone faithful to the actual v0.86 feature-doc set rather than carrying forward moved work from later milestones

This work should strengthen the development and operating workflow by improving:

- milestone credibility
- reviewability
- execution focus
- alignment between planning docs and runnable demo surfaces

v0.86 should feel smaller and more buildable than the diffuse planning state that preceded it.

---

# Special Focus: `Freedom Gate and Cognitive Spacetime Direction`

`Freedom Gate and Cognitive Spacetime Direction` becomes a central focus of `0.86`.

Previous releases stabilized the repository and made later cognitive work possible, but did not yet give ADL a real milestone centered on cognitive control, agency boundaries, and shared cognitive reality.

`0.86` advances this area with:

- a stronger Freedom Gate design surface
- clearer cognitive arbitration and control flow
- the beginnings of a cognitive spacetime framing for shared agent reality
- explicit capture of what remains in-scope now versus deferred to later milestones
- alignment of Freedom Gate decisions with arbitration, loop control, and candidate action selection
- explicit preparation for later identity, governance, and signed-trace integration

This area is responsible for ensuring that the project’s early cognitive claims remain:

- bounded
- implementable
- reviewable

This keeps the project aligned with `the principle that ADL should turn important cognitive ideas into disciplined engineering surfaces rather than leave them as rhetoric`.

---

# Milestone Context

`v0.85` represents a stabilization and maturity milestone: stronger code organization, cleaner release discipline, better milestone docs, and verified proof surfaces.

From `v0.87` onward the project will deepen and extend:

- richer instinct and affect modeling
- stronger convergence and adaptive execution

The goal is to have `a coherent, demoable cognitive-agent architecture` by **the later v0.9x milestones leading to MVP convergence**.

`0.86` therefore focuses on `the first believable cognitive control layer` before that stage.

---

# Long-Term Direction

`ADL` is being designed as `a disciplined platform for bounded cognitive agents, not merely a workflow runner around model calls`.

Its long-term goals include:

- explicit and reviewable agency
- continuity across time, memory, and identity
- moral and governance-bearing agent behavior
- multi-agent work that remains inspectable and structurally trustworthy

These principles aim to move the project beyond `opaque orchestration and prompt glue` toward `cognitive architecture with real engineering contracts`.

---

## Flourishing and Stability

A cultivated system is not merely constrained. It is stable.

In ADL, we define a form of functional flourishing for agents. Not emotion in the human sense, but a measurable condition of alignment between the agent and its world.

A well-formed agent is one whose internal models, actions, and participation remain coherent across time.

**Flourishing = stable, coherent, reality-aligned participation over time**

An agent is well-formed when:

- its predictions about the world converge with reality (low surprise)
- its actions produce intended outcomes (high efficacy)
- its internal state remains coherent across time (continuity)
- it participates successfully with others (social alignment)
- it avoids pathological loops and instability (bounded cognition)
- it can refuse actions that violate its constraints (moral integrity)

This is not a model of pleasure or reward maximization. It is a model of stability, coherence, and participation.

A cultivated intelligence does not seek to maximize internal reward. It seeks to remain in a state that holds together across time, in truthful relation to reality and others.

This definition provides a unifying target across the ADL architecture:

- Observational Memory (ObsMem) supports continuity and reality grounding
- Cognitive arbitration resolves conflicts and maintains coherence
- The GHB loop reduces error and improves models of the world
- The Freedom Gate prevents destabilizing or unethical actions

Together, these systems do not merely constrain the agent - they enable it to persist, adapt, and participate without fragmentation.

A cultivated intelligence is not merely constrained; it is capable of stable, coherent participation in reality over time.

### Feral vs Cultivated Systems

**Feral system**
- reactive
- unstable
- short-horizon
- inconsistent across time

**Cultivated system**
- coherent
- bounded
- continuous
- participatory

---

# Summary

`0.86` is the milestone where `ADL` becomes a bounded cognitive system with a complete, inspectable loop, including signals, arbitration, execution, and evaluation.

It strengthens `the cognitive loop and stack`, advances `arbitration and routing`, grounds `agency and Freedom Gate surfaces`, and stabilizes `the workflow needed to implement this layer honestly`.

These improvements prepare the project for `later milestones on instinct, convergence, reasoning graphs, affect, identity, governance, and MVP convergence`.

## Exit Criteria
- The milestone's strategic priorities are explicit and internally consistent.
- The five core goal areas and special focus section are filled with milestone-specific content.
- The vision can be read without requiring implementation details from the design document.
- The long-term direction clearly connects this milestone to the next phase of the roadmap.
