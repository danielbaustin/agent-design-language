# ADL Glossary

## Status

Canonical public glossary for load-bearing ADL terms.

This document is a reference surface, not a release claim. Some entries describe
implemented concepts; others are marked planned or future when they name
milestone work that is not complete yet.

## Source Orientation

Primary public source surfaces include:

- `README.md`
- `adl/README.md`
- `docs/milestones/v0.87/`
- `docs/milestones/v0.89/`
- `docs/milestones/v0.90/`
- `docs/milestones/v0.90.1/`
- `docs/milestones/v0.90.2/`
- `docs/milestones/v0.90.3/`
- `docs/architecture/PROVIDER_CAPABILITY_AND_TRANSPORT_ARCHITECTURE.md`
- `docs/planning/ADL_FEATURE_LIST.md`
- `docs/tooling/README.md`

## Core Project Terms

| Term | Definition |
| --- | --- |
| ADL | Agent Design Language: the language, Rust runtime, CLI, review surfaces, and milestone proof packages used to make agent workflows deterministic, inspectable, and falsifiable. |
| CSM | Cognitive Spacetime Model: ADL's model for a governed runtime world with time, state, causality, trace, and identity-continuity surfaces. |
| Runtime v2 | The current ADL runtime architecture line for manifold, citizen, kernel, snapshot, rehydration, wake, quarantine, Observatory, and proof-surface work. |
| Polis | A governed CSM environment: the social, policy, security, and economic layer that gives participants rules, standing, duties, rights, and accountability. |
| Manifold | A bounded Runtime v2 world or substrate that hosts citizen records, episodes, state, traces, and snapshots. |
| Gödel agent | Planned higher-order ADL agent identity capable of richer continuity and self-reference; the first true birthday remains future milestone scope, not a v0.90.1 or v0.90.2 claim. |

## Standing And Identity

| Term | Definition |
| --- | --- |
| Standing | Recognition as a participant in the polis with declared identity, bounded authority, policy accountability, and trace visibility. |
| Citizen | A full identity-bearing participant in a polis, subject to rights, duties, policy, continuity, and Freedom Gate-mediated agency. Current v0.90.x citizens are provisional unless explicitly stated otherwise. |
| Guest | A temporary, externally originated, or limited participant with bounded scope, capabilities, duration, and traceability. Humans enter the CSM polis as guests by default. |
| CSM identity | The mediated identity inside the CSM that can hold citizen standing. The human is not directly the citizen; the CSM identity is the citizen. |
| Human provider | A provider surface where a human supplies bounded judgment, review, approval, reframing, hypothesis generation, or another artifactized contribution to an ADL workflow. Human provider mode is not polis citizenship. |
| Service actor | Runtime substrate actor, such as a scheduler, tool adapter, execution engine, or background process. A service actor acts only under delegated authority and does not automatically hold social standing. |
| Naked actor | Prohibited actor that can observe, communicate, or affect shared CSM state without declared standing, bounded authority, and traceable identity. |
| Out-of-band action | Action that bypasses the mediated, recorded, signed, temporally anchored CSM path. Direct out-of-band human action does not count as citizen action. |

## Governance And Safety

| Term | Definition |
| --- | --- |
| Freedom Gate | ADL's decision and mediation boundary for allowing, refusing, deferring, or constraining actions under policy, governance, and moral/safety checks. |
| Policy | Explicit rule surface that constrains execution, provider selection, action authority, review, or state transition behavior. |
| Constitution | Planned polis-level rule set for invariants, allowed actions, prohibited actions, escalation, rights, duties, and governance boundaries. |
| Sanctuary | Planned evidence-preserving safety state for ambiguous or unsafe continuity where destructive progress should pause rather than optimize through doubt. |
| Quarantine | Safety state that isolates a failed, suspect, ambiguous, or policy-violating runtime state while preserving evidence for review. |
| Challenge | Planned due-process artifact or flow for contesting a continuity, projection, inspection, wake, or authority decision. |
| Appeal | Planned review path following a challenge, preserving evidence while a higher authority or review process resolves the contested state. |
| Anti-equivocation | Detection of conflicting continuity claims, such as two incompatible signed successors for the same sequence or lineage position. |

## Trace, Time, And Evidence

| Term | Definition |
| --- | --- |
| Trace | Ordered execution evidence showing what happened, under which actor, policy, artifact, and temporal context. |
| Signed trace | Trace evidence protected by signatures or signing policy so origin, integrity, and replay claims can be reviewed. |
| Temporal anchoring | Stable time/order evidence that lets reviewers determine when an event occurred relative to other events. |
| Snapshot | Captured Runtime v2 state used for rehydration, wake, continuity checks, and review. |
| Rehydration | Restoring or reconstructing runtime state from a snapshot or persisted artifact set. |
| Wake | Activating a rehydrated or paused citizen/runtime state after required continuity and invariant checks pass. |
| Worldline | Planned continuity path through time for an identity, citizen, manifold, or runtime state. |
| Continuity witness | Planned artifact that explains and supports a major identity or state transition, such as admission, snapshot, wake, migration, quarantine, or challenge. |
| Receipt | Planned citizen-facing or reviewer-facing artifact explaining what transition occurred, why it was accepted, and what evidence supports it. |
| Quintessence checkpoint | Planned protected continuity-bearing citizen-state checkpoint, expected to be signed, optionally sealed, lineage-aware, and suitable for private-state protection. |

## State And Projection

| Term | Definition |
| --- | --- |
| Private state | Authoritative internal citizen or runtime state that should not be treated as ordinary debug data or exposed directly to all operators. |
| Projection | A derived view of state for a specific audience or purpose. A projection is a review or inspection surface, not necessarily authority. |
| Public projection | Redacted projection safe for broader reader, reviewer, or public use. It must not leak private state. |
| Operator projection | View intended for operators to understand status, risk, and required action without raw private-state access unless explicitly authorized. |
| Citizen-facing projection | View intended to explain relevant state or transitions to the citizen or citizen identity in a bounded, appropriate form. |
| Observatory | Runtime v2 visibility surface for operator/reviewer inspection of packets, reports, status, refusals, wake continuity, and boundary evidence. |

## Providers, Models, And Skills

| Term | Definition |
| --- | --- |
| Provider | Runtime integration surface that supplies model, human, tool, or other bounded capability to ADL under explicit configuration and policy. |
| Model provider | Provider family or vendor/runtime that supplies model execution, such as OpenAI, Anthropic, Gemini, Ollama, compatible HTTP, or mock providers. |
| Layer 8 provider | Planned provider architecture layer that separates transport, vendor identity, stable ADL model reference, provider-native model id, capability envelope, and policy. |
| Transport | How ADL communicates with a backend, distinct from provider/vendor identity. Examples include local CLI, HTTP, compatible HTTP, or mock transport. |
| Model reference | Stable ADL-facing model identifier used by agents, policies, and configuration, isolated from provider-native model strings. |
| Provider model id | Provider-native model identifier used at the adapter boundary, kept behind catalogs or adapter-specific configuration. |
| Capability | What a provider, model, skill, or actor is declared or observed to be able to do. Capability is not the same thing as identity or permission. |
| Aptitude | Measured tendency or strength across capability tests, tasks, or evaluation contexts. Aptitude is evidence-bearing and may vary by model, prompt, tool, and domain. |
| Skill | Bounded local instruction and tool bundle that guides Codex or another agent through a specialized workflow. A skill is a procedure and evidence contract, not an autonomous actor by itself. |
| Routing preference | Policy or operator preference for selecting among providers, models, skills, or agents. Routing preference must not be confused with capability evidence or authorization. |

## Workflow And Issue Lifecycle

| Term | Definition |
| --- | --- |
| Work package | Bounded milestone unit of work, often labeled WP-01 through WP-20, with declared purpose, output, dependencies, and validation expectations. |
| STP | Structured Task Prompt: the source task prompt defining scope, goal, required outcome, acceptance criteria, inputs, target files, validation, and non-goals. |
| SIP | Structured Input Prompt: execution input card that tells the agent how to run the already-scoped issue in the correct worktree and lifecycle context. |
| SOR | Structured Output Record: output card recording what was done, artifacts produced, validation run, integration state, privacy checks, deviations, and follow-ups. |
| PR lifecycle | ADL issue workflow from issue creation/bootstrap through readiness, run binding, implementation, finish/PR publication, janitoring, merge, and closeout. |
| Conductor | Lightweight workflow-routing skill that chooses the next lifecycle or editor skill and stops at the routing/dispatch boundary rather than absorbing implementation work. |
| Worktree-first execution | ADL process rule that tracked implementation work happens in issue worktrees, while the root checkout remains clean main. |
| Closeout | Final lifecycle step after merge or intentional no-PR closure that normalizes local records, issue state, and truth of the completed work. |

## Planned And Future Terms

| Term | Definition |
| --- | --- |
| Moral trace | Planned governance evidence trail linking decisions, moral context, outcomes, and reviewability. |
| Memory Model v2 | Planned memory architecture band for stronger identity, continuity, recall, and governed learning. |
| Birthday | Future first true Gödel-agent birth event, expected to require stronger identity, continuity, capability rebinding, citizen standing, and record semantics than current provisional citizens. |
| Contract market | Planned citizen-economics substrate for bounded contracts, bids, evaluation, delegation, lifecycle authority, fixtures, runner, and review summaries. |
| Inter-polis economics | Future economics across polis boundaries. This is not part of the current Runtime v2 foundation or v0.90.4 first contract-market proof unless a later decision explicitly changes scope. |

