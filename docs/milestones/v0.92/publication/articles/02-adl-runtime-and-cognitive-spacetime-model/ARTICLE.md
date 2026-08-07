# ADL Runtime and the Cognitive Spacetime Manifold

A chat session has a beginning, a transcript, and an end. A long-lived agent needs something more demanding: a world.

It needs time that orders events, state that survives a request, memory tied to evidence, causal traces, policy, and a defensible account of identity across sleep, recovery, and change. Without those things, persistence is mostly the appearance of continuity created by passing text from one invocation to the next.

ADL calls its model for that richer world the **Cognitive Spacetime Manifold**, or CSM.

## From requests to inhabitants

Most software services are designed around jobs. A request arrives, computation happens, a response leaves. That model works well for many agent applications, but it becomes strained when we ask an agent to remember commitments, learn from prior episodes, coexist with other agents, or retain a stable identity.

CSM changes the unit of thought. An agent is not merely a function call. It is a potential inhabitant of a governed runtime world.

That world needs several kinds of structure:

- **Time and ordering**, so episodes and commitments have a sequence.
- **State and memory**, so later behavior can be grounded in witnessed artifacts rather than a reconstructed story.
- **Causality and trace**, so operators can connect a result to the events and policies that produced it.
- **Identity continuity**, so waking or restoring an agent is not confused with creating a new process that happens to reuse a label.
- **Governance**, so standing, authority, policy, and external effects remain explicit.
- **Visibility**, so operators and reviewers can inspect appropriate projections without exposing every private detail.

The phrase “spacetime” is doing architectural work here. An event has meaning partly because of where it sits in a worldline: what preceded it, which state it inherited, which constraints applied, and what it changed.

## The manifold and the polis

ADL distinguishes the runtime world from the society that governs it.

The **manifold** is the world: state, time, memory, causal relationships, execution, and continuity. The **polis** is the social, security, policy, and economic order within that world.

That distinction lets the runtime ask questions that a task queue cannot answer on its own. Which actor has standing? What authority was delegated? Is a resource available? What information may this observer see? What obligations survive the current episode? Can a refusal be appealed, and what evidence should the appeal retain?

The polis does not turn an agent into a legal person. It supplies explicit architecture for relationships that otherwise hide in prompts, application code, or operator intuition.

## Governed execution in Runtime v3

ADL's Runtime v3 contains a focused implementation of this posture. Runtime v3 is the current governed-execution architecture; earlier Runtime v2 surfaces remain in the repository and are cited later in this series for memory, citizen state, and Theory of Mind. Runtime v3 exposes typed services for governance ingress, Freedom Gate mediation, bounded actuation, and audit.

A proposed action arrives with identity, policy, resource, and authority information. The Freedom Gate admits or refuses it, and a bounded actuation layer executes only against a signed one-shot permit. Governance and actuation state participate in checkpoint and restore, so a restart cannot silently revive revoked authority. Article 4 covers those mechanics in detail.

This is meaningful implementation, but it is a bounded one. It does not yet claim a complete distributed authority system, production message-bus transport, universal policy language, or operator interface.

## Persistence is not identity

One of CSM's hardest problems is resisting a convenient shortcut: calling any restored state “the same agent.”

A process can restart. A snapshot can load. A name can reappear. None of those events alone proves identity continuity.

ADL's v0.92 birthday plan therefore sets a higher bar, requiring named identity architecture, continuity records, and witnessed evidence rather than a successful restart. Article 3 states the full acceptance boundary; that surface is still active work.

This stronger bar matters because identity claims affect every later layer. If continuity is ambiguous, commitments, rights, responsibility, learning, and social relationships become ambiguous too.

## Memory must be evidence, not mythology

The same discipline applies to memory. An agent's confident account of its past is not automatically a memory record. Durable memory needs provenance: an artifact, event, observation, or trace that can be connected to the present claim.

ADL uses runtime packets and memory-oriented surfaces to preserve that distinction. A system may summarize or compress its history, but the relationship between the summary and its source evidence should remain inspectable. Unknowns should remain unknown. Corrections should not disappear.

This creates a different kind of agent experience. Continuity is not produced by endlessly expanding context. It is produced by governed state transitions whose evidence can be retained, compressed, and revisited.

## The world is the safety mechanism

It is tempting to treat governance as a filter added after an agent decides what to do. CSM suggests a stronger approach: governance belongs in the structure of the world itself.

Authority, time, resource limits, privacy, causality, and review are not afterthoughts. They determine which transitions are possible and what evidence those transitions leave behind.

The current ADL repository proves bounded portions of that architecture. The fully inhabited runtime remains incomplete. That is not a minor footnote; it is the line between a serious prototype and an unsupported claim.

CSM's value today is therefore both practical and conceptual. It supplies implemented contracts for governed runtime work, and it gives engineers a vocabulary for asking what a persistent agent would actually need before persistence deserves to be called a life in a shared world.

## Repository Sources

- [`docs/explainers/CSM.md`](../../../../../explainers/CSM.md)
- [`docs/architecture/RUNTIME_V3_OPERATIONAL_COMPONENTS.md`](../../../../../architecture/RUNTIME_V3_OPERATIONAL_COMPONENTS.md)
- [`docs/architecture/RUNTIME_V3_GOVERNED_EXECUTION_ARCHITECTURE.md`](../../../../../architecture/RUNTIME_V3_GOVERNED_EXECUTION_ARCHITECTURE.md)
- [`docs/adr/0012-runtime-v2-bounded-csm-run.md`](../../../../../adr/0012-runtime-v2-bounded-csm-run.md)
- [`docs/milestones/v0.92/IDENTITY_CONTINUITY_AND_BIRTHDAY_PLAN_v0.92.md`](../../../IDENTITY_CONTINUITY_AND_BIRTHDAY_PLAN_v0.92.md)
