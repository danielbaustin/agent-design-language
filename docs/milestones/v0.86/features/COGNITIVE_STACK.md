# Cognitive Stack v0.86

**Status:** Draft  
**Version:** v0.86 planning  
**Scope:** Architecture synthesis  
**Related:** instinct model, affect model, ObsMem, reasoning graphs, freedom gate, adaptive execution, Gödel/Hadamard loop

---

## Overview

This document captures an emerging view of the ADL architecture as a **cognitive stack** rather than merely an orchestration stack.

ADL began as a framework for deterministic, reviewable, multi-agent execution.  
Over time, several architectural needs emerged that collectively resemble a broader cognitive substrate:

- memory
- evaluation
- structured reasoning
- meta-reasoning
- bounded autonomy
- persistence
- internal drives

This document synthesizes those ideas into a single conceptual stack for v0.86 planning.  
This stack also supports a dual-process model of cognition (fast vs slow thinking), with the Cognitive Arbitration Layer determining which path is used.

---

## Core Claim

The central architectural claim is:

> Reliable long-running agents require more than tools, prompts, and execution graphs.  
> They require a cognitive substrate that supports memory, evaluation, motivation, reasoning, governance, and bounded adaptation.

This does not imply anthropomorphic simulation.

It implies that if an agent is expected to:

- persist across time
- investigate anomalies
- revise plans
- maintain internal coherence
- operate within governance boundaries
- improve in a bounded way

then the agent architecture must contain primitives that support those behaviors.

---

## Why a Cognitive Stack

Many current agent frameworks focus primarily on:

- prompt chains
- planners
- tool calls
- orchestration loops
- execution graphs

These are useful, but incomplete.

They do not fully explain:

- why some anomalies should matter more than others
- why unfinished work should create pressure
- why contradictions should trigger deliberate repair
- why agents should remain coherent across time
- how agents should remain autonomous without becoming unsafe

ADL has gradually added architectural layers that address these missing pieces.

Taken together, those layers form a cognitive stack.

---

## Proposed Conceptual Stack

A current synthesis is:

```text
1. Policy / Freedom Layer
2. Instinct Layer
3. Affect Layer
4. Cognitive Arbitration Layer
5. Meta-Reasoning Layer
6. Reasoning Representation Layer
7. Memory Layer
8. Adaptive Execution Layer
9. Artifact / Action Layer
```

Each layer is described below.

---

## 1. Policy / Freedom Layer

This layer defines governance and permission boundaries.

Its role is to answer:

- what may the agent do?
- what must remain constrained?
- what is allowed to change?
- what requires review or approval?

The freedom layer is essential because a capable agent without constraints is unsafe.

This layer includes ideas such as:

- policy surfaces
- review requirements
- governance contracts
- bounded action permissions
- execution constraints

A useful shorthand is:

```text
freedom gate = permission boundary for agency
```

---

## 2. Instinct Layer

This layer provides persistent internal drives.

Its role is to answer:

- what matters continuously?
- what creates pressure before explicit deliberation?
- what prevents the system from becoming purely reactive?

Candidate instincts currently include:

- integrity
- curiosity
- coherence
- completion

This layer is a key candidate for making agents genuinely effective rather than merely responsive.

---

## 3. Affect Layer

This layer provides dynamic evaluation signals.

Its role is to answer:

- how is the current situation going relative to system pressures and goals?
- what tensions, saliences, or confidence shifts are present?
- what feels resolved vs unresolved at the system level?

Affect is not identical to instinct.

Instinct persists.  
Affect evaluates current state relative to persistent drives, goals, and observations.

This layer helps make reasoning more selective, contextual, and priority-aware.

---

## 3.5 Cognitive Arbitration Layer

This layer determines:

- whether to use fast-path or slow-path cognition
- how much cognitive effort to allocate
- when to escalate, defer, or refuse

It integrates:

- uncertainty
- cost
- risk
- policy constraints

This layer acts as the control surface for cognitive resource allocation.

See: `COGNITIVE_ARBITRATION.md`

---

## 4. Meta-Reasoning Layer

This layer reasons about reasoning.

Its role is to answer:

- what is missing?
- what assumptions may be wrong?
- where is the representation incomplete?
- what hypotheses should be generated next?

This is where Gödel-like incompleteness awareness belongs.

Gödel should not be collapsed into reasoning graphs themselves.  
Reasoning graphs are maps. Meta-reasoning is what notices their incompleteness.

This layer is closely associated with:

- incompleteness awareness
- hypothesis generation triggers
- self-critique
- gap detection
- higher-order planning

---

## 5. Reasoning Representation Layer

This layer holds explicit reasoning artifacts.

Its role is to provide:

- inspectable claims
- evidence links
- contradictions
- dependencies
- hypotheses
- structured reasoning state

Reasoning graphs belong here.

This layer is important for:

- reviewability
- auditability
- deterministic reasoning artifacts
- structured comparison of alternatives
- durable cognition across steps

But it must remain distinct from cognition itself.

Reasoning graphs are maps, not the territory.

---

## 6. Memory Layer

This layer preserves observations and usable history.

Its role is to answer:

- what happened before?
- what evidence or traces are available?
- what experiences matter for current reasoning?
- what recurring patterns should shape future decisions?

ObsMem belongs primarily here.

This layer supports:

- continuity across tasks
- longitudinal reasoning
- post hoc review
- adaptive learning surfaces
- richer context than ephemeral prompt state

---

## 7. Adaptive Execution Layer

This layer turns plans and reasoning into bounded actions.

Its role is to answer:

- what should be executed now?
- what experiment should be run next?
- how should plans adapt to new evidence?
- how do we preserve boundedness and replayability?

The AEE belongs here.

This layer translates cognition into:

- experiments
- bounded execution
- controlled adaptation
- artifact generation
- validation loops

---

## 8. Artifact / Action Layer

This is the surface where the system affects the world.

It includes:

- files
- reports
- reviews
- commits
- validation artifacts
- prompts/cards
- other generated outputs

This layer should remain governed, reviewable, and inspectable.

It is where the internal architecture becomes legible.

---

## Layer Interactions

A simplified interaction flow:

```text
policy/freedom constrains all lower layers
instinct creates persistent pressure
affect evaluates current state
cognitive arbitration selects fast vs slow path and allocates effort
meta-reasoning identifies gaps and generates next-step pressure
reasoning graphs formalize explicit reasoning
memory supplies evidence and historical context
adaptive execution performs bounded actions
artifacts/actions externalize results
```

This is conceptual, not necessarily a literal runtime pipeline.

Some layers may be concurrent or mutually informing rather than strictly sequential.

---

## Agency in the Stack

This stack clarifies the difference between an "agent" and "agency."

An agent may simply perform actions.

Agency requires more:

- persistent internal priorities
- bounded autonomy
- internal pressure to continue, revise, investigate, or complete work

Within this stack, agency arises from the interaction of:

- instinct
- affect
- meta-reasoning
- memory
- bounded execution
- policy constraints

This leads naturally to the idea of **bounded agency**.

---

## Why Bounded Agency Matters

ADL should not aim for unbounded autonomy.

Instead it should aim for:

- motivated but governable agents
- adaptable but reviewable agents
- persistent but bounded agents
- capable but auditable agents

This is one of the central architectural differentiators of ADL.

The point is not to unleash stochastic systems.  
The point is to civilize them.

---

## On “Civilizing” LLMs

A useful framing that emerged in discussion is that LLMs are currently a kind of **feral intelligence**:

- powerful
- reactive
- weakly disciplined
- poor at persistence
- poor at self-governed follow-through

ADL can be understood as a civilizing substrate that adds:

- structure
- memory
- reviewability
- governance
- bounded autonomy
- persistent cognitive organization

This framing is colorful, but it captures something real.

The architecture is not merely trying to make LLMs more useful.  
It is trying to make them more **legible, governable, and cognitively organized**.

---

## Why This Matters for v0.86

v0.86 is a natural place to unify the existing strands of work.

By this point, ADL has enough pieces that it benefits from an explicit architectural synthesis.

This synthesis can help:

- unify milestone planning
- clarify cross-WP relationships
- prevent conceptual drift
- show why apparently separate features belong to one coherent architecture
- position ADL as more than an orchestration framework

---

## Open Questions

The following should remain open for future planning:

- Which parts of the cognitive stack are conceptual only vs implementation-bound?
- Which surfaces should become explicit repo artifacts in v0.86?
- Where should instinct and affect signals appear in schema/contracts, if at all?
- How should meta-reasoning be represented without collapsing it into maps of reasoning?
- Which layers require deterministic replay and which may remain partially advisory?
- How should multi-agent variants of this stack be handled?

---

## Summary

ADL is evolving toward a cognitive stack with the following major layers:

- policy / freedom
- instinct
- affect
- cognitive arbitration
- meta-reasoning
- reasoning representation
- memory
- adaptive execution
- artifacts/actions

This stack offers a more complete explanation of what is required for bounded, trustworthy, and effective agents.

It also suggests that ADL is not merely an orchestration framework.  
It is becoming a cognitive substrate for artificial agents that are:

- inspectable
- reviewable
- persistent
- policy-governed
- scientifically improvable