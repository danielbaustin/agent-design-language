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
- `docs/milestones/v0.75/`
- `docs/milestones/v0.8/`
- `docs/milestones/v0.85/`
- `docs/milestones/v0.86/`
- `docs/milestones/v0.87/`
- `docs/milestones/v0.87.1/`
- `docs/milestones/v0.88/`
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
| AEE | Adaptive Execution Engine: the older ADL execution-adaptation line for bounded strategy selection, recovery, learning, and policy-aware execution. Later runtime work inherits parts of this lineage without treating adaptation as hidden magic. |
| ObsMem | Observational Memory: ADL's evidence-adjusted retrieval and indexing concept for using prior run evidence, scores, traces, and provenance without turning memory into unverifiable lore. |
| Chronosense | ADL's temporal-awareness line for time, commitments, deadlines, temporal causality, identity continuity, and explanation across runs. |
| Polis | A governed CSM environment: the social, policy, security, and economic layer that gives participants rules, standing, duties, rights, and accountability. |
| Manifold | A bounded Runtime v2 world or substrate that hosts citizen records, episodes, state, traces, and snapshots. |
| Kernel | Runtime v2 control substrate responsible for bounded services such as scheduling, policy checks, state transition coordination, packet emission, and operator-visible control-plane behavior. |
| Control plane | The operator- and runtime-facing coordination layer that governs execution, review, state transition, and safety actions without being the citizen or model itself. |
| Episode | A bounded runtime interval or activity record within a longer-lived stateful system, used to reason about what happened during one unit of work or interaction. |
| Long-lived agent | Agent identity or runtime participant designed to persist across cycles, carry state forward, survive interruptions, and expose continuity evidence rather than behaving as a one-shot prompt. |
| Gödel agent | Planned higher-order ADL agent identity capable of richer continuity and self-reference; the first true birthday remains future milestone scope, not a v0.90.1 or v0.90.2 claim. |
| ANRM | Agent-native reasoning model: planned or experimental small-model/shepherd work aimed at giving ADL a local house-model path for bounded reasoning, scaffolding, and later training experiments. |

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
| Invariant | Condition that must remain true across a runtime transition, demo proof, workflow step, or policy boundary. Runtime v2 uses invariants to make safety and continuity reviewable. |
| Security boundary | Declared limit around authority, data, state, capability, or execution flow that must be protected and evidenced rather than assumed. |
| Refusal | Governed decision to decline an action, transition, provider request, or unsafe path while preserving enough explanation and trace evidence for review. |
| Sanctuary | Planned evidence-preserving safety state for ambiguous or unsafe continuity where destructive progress should pause rather than optimize through doubt. |
| Quarantine | Safety state that isolates a failed, suspect, ambiguous, or policy-violating runtime state while preserving evidence for review. |
| Recovery | Controlled return from interruption, failure, quarantine, or degraded state using preserved evidence, state, and policy checks. |
| Challenge | Planned due-process artifact or flow for contesting a continuity, projection, inspection, wake, or authority decision. |
| Appeal | Planned review path following a challenge, preserving evidence while a higher authority or review process resolves the contested state. |
| Anti-equivocation | Detection of conflicting continuity claims, such as two incompatible signed successors for the same sequence or lineage position. |
| Threat model | Source-grounded account of assets, trust boundaries, attacker capabilities, abuse paths, and mitigations for a bounded system or feature. |

## Trace, Time, And Evidence

| Term | Definition |
| --- | --- |
| Trace | Ordered execution evidence showing what happened, under which actor, policy, artifact, and temporal context. |
| Signed trace | Trace evidence protected by signatures or signing policy so origin, integrity, and replay claims can be reviewed. |
| Temporal anchoring | Stable time/order evidence that lets reviewers determine when an event occurred relative to other events. |
| Artifact | Durable file, packet, trace, report, schema, manifest, or generated output that can be inspected as evidence of work or runtime behavior. |
| Proof surface | Artifact or command result intended to let reviewers verify a claim without relying only on prose. |
| Run packet | Bounded bundle of runtime inputs, outputs, metadata, state summaries, evidence, and review surfaces for one CSM or demo run. |
| Trace bundle | Packaged trace evidence for a run, usually including ordered events, metadata, artifacts, and replay or review hooks. |
| Replay | Re-executing or re-inspecting a bounded artifact set to verify determinism, ordering, evidence, or failure classification. |
| Canonical evidence view | Deterministic normalized evidence representation used by older Gödel/AEE work to compare run outcomes without volatile fields. |
| ExperimentRecord | Historical Gödel/AEE schema concept for recording a hypothesis, mutation, evaluation plan, observed evidence, and outcome. |
| EvaluationPlan | Historical Gödel/AEE schema concept describing how a proposed mutation or experiment should be evaluated. |
| Mutation | Declared change candidate in the Gödel/AEE lineage, intended to be evaluated against evidence rather than silently applied. |
| Violation artifact | Runtime v2 hardening artifact that records an invariant, safety, policy, or boundary violation in a reviewable form. |
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
| Operator report | Human-readable runtime review artifact summarizing status, risks, decisions, refusals, continuity evidence, and recommended operator action. |
| Visibility packet | Structured Observatory artifact exposing enough status and evidence for review while respecting private-state and projection boundaries. |

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
| Shepherd | Runtime or model role that helps preserve, guide, recover, or supervise long-lived work without becoming the sole source of authority. Current ANRM/Gemma shepherd work is experimental. |
| Gemma | Google open model family used in ADL planning and experiments as a plausible local or house-model candidate; model behavior still requires evidence, scaffolding, and evaluation. |

## Cognition, Learning, And Time

| Term | Definition |
| --- | --- |
| Cognitive stack | ADL conceptual stack connecting signals, arbitration, reasoning, action mediation, evaluation, memory participation, and Freedom Gate review. |
| Cognitive loop | Repeating bounded process by which ADL observes signals, proposes or evaluates actions, records evidence, and feeds results back into future reasoning. |
| Cognitive arbitration | Selection and prioritization layer for competing signals, goals, hypotheses, or actions before execution or Freedom Gate mediation. |
| Fast/slow thinking | ADL planning model that distinguishes quick heuristic response from slower evidence-heavy review, deliberation, and verification paths. |
| Hypothesis engine | Gödel/AEE lineage component that proposes hypotheses or explanations from evidence so later work can test rather than merely assert improvement. |
| Reasoning graph | Structured representation of hypotheses, evidence, decisions, dependencies, and outcomes used to make reasoning inspectable. |
| Affective signal | Bounded non-authoritative signal about urgency, confidence, concern, fatigue, happiness, or similar state used to inform arbitration without overriding evidence or policy. |
| GHB | Gödel-Hadamard-Bayes: exploratory ADL idea combining hypothesis generation, structured transformation, and evidence-adjusted updating. It remains an idea lineage unless a milestone marks a concrete implementation. |
| Commitment | Temporal promise, obligation, or intended future action that should be visible to the runtime rather than hidden in prose. |
| Deadline | Time-bound commitment or constraint used by Chronosense and scheduling work to reason about urgency and accountability. |
| Temporal causality | Relationship between events where ordering and context matter for explaining why a later state, decision, or obligation exists. |
| Temporal query | Query over trace, memory, or run history that asks when something happened, what preceded it, what followed it, or what commitments were active. |

## Demos, Reviews, And Product Surfaces

| Term | Definition |
| --- | --- |
| Demo matrix | Milestone document that maps claims to runnable demos, proof surfaces, success signals, determinism notes, and status. |
| Quality gate | Milestone validation surface that records required checks, evidence, thresholds, and release-readiness constraints before ceremony or review. |
| Internal review | ADL review pass run by the project before third-party review, focused on stale truth, missing proof, implementation risk, docs drift, and release readiness. |
| Third-party review | External or independent review pass intended to test whether the milestone package is understandable, truthful, and reviewable by someone outside the immediate implementation loop. |
| Review packet | Bounded packet of source evidence, scope, findings, specialist notes, diagrams, reports, and residual risks prepared for review or customer-facing synthesis. |
| CodeBuddy | Planned ADL-powered product/business line for repository-wide code and architecture review, diagrams, tests, reports, and remediation planning. |
| Aptitude Atlas | Planned ADL-powered evaluation product/business line for measuring model capabilities and aptitudes in leaderboard-style, evidence-bearing tasks. |
| Paper Sonata | ADL publication and writing workflow line for turning structured source packets into article, paper, and manuscript artifacts with reviewable evidence boundaries. |
| ArXiv writer | Skill/demo line for producing arXiv-style manuscript packets from bounded source packets without claiming publication or inventing citations. |
| Medium article writer | Skill/demo line for producing Medium-style article packets from bounded briefs without directly publishing them. |
| Rust transpiler | ADL demo and potential product line for translating or refactoring code toward Rust with reviewable proof, tests, and migration evidence. |

## Adversarial And Security Terms

| Term | Definition |
| --- | --- |
| Adversarial runtime | ADL runtime mode or feature line where attack, defense, exploit generation, replay, and mitigation happen inside bounded policy and evidence rules. |
| Self-attacking system | System that runs governed internal adversarial pressure against itself to find weaknesses before external attackers do. |
| Red agent | Adversarial role that searches for weaknesses, exploit paths, unsafe assumptions, or policy failures under a declared boundary. |
| Blue agent | Defensive role that analyzes, mitigates, patches, tests, or hardens against red-agent findings under a declared boundary. |
| Exploit artifact | Evidence artifact describing a discovered exploit path, trigger, impact, constraints, replayability, and mitigation status. |
| Adversarial replay manifest | Manifest that makes adversarial findings replayable or reviewable without turning exploit details into uncontrolled behavior. |
| Continuous verification | Ongoing process of checking whether runtime, policy, docs, demos, or security claims still hold as the system changes. |

## Workflow And Issue Lifecycle

| Term | Definition |
| --- | --- |
| Work package | Bounded milestone unit of work, often labeled WP-01 through WP-20, with declared purpose, output, dependencies, and validation expectations. |
| Issue wave | Ordered set of GitHub issues generated from a milestone work-package plan so execution can proceed with traceable scope and dependencies. |
| Issue card | Machine-readable or structured issue definition used to create, review, or track an issue's scope, labels, dependencies, and required outcome. |
| Prompt spec | Contract describing required prompt sections, supported fields, ordering, automation hints, and reviewer expectations for structured ADL prompts. |
| Structured prompt | Prompt artifact with explicit goal, inputs, acceptance criteria, validation, constraints, non-goals, and output expectations instead of free-form instructions. |
| STP | Structured Task Prompt: the source task prompt defining scope, goal, required outcome, acceptance criteria, inputs, target files, validation, and non-goals. |
| SIP | Structured Input Prompt: execution input card that tells the agent how to run the already-scoped issue in the correct worktree and lifecycle context. |
| SOR | Structured Output Record: output card recording what was done, artifacts produced, validation run, integration state, privacy checks, deviations, and follow-ups. |
| PR lifecycle | ADL issue workflow from issue creation/bootstrap through readiness, run binding, implementation, finish/PR publication, janitoring, merge, and closeout. |
| Conductor | Lightweight workflow-routing skill that chooses the next lifecycle or editor skill and stops at the routing/dispatch boundary rather than absorbing implementation work. |
| Worktree-first execution | ADL process rule that tracked implementation work happens in issue worktrees, while the root checkout remains clean main. |
| Milestone compression | ADL process line for reducing issue overhead by preparing cards, docs, validation profiles, and state truth earlier while preserving reviewability and safety. |
| Early planning lane | Process lane where future milestone docs can be drafted and reviewed before the current milestone closes, without making tracked implementation claims early. |
| Closeout | Final lifecycle step after merge or intentional no-PR closure that normalizes local records, issue state, and truth of the completed work. |

## Planned And Future Terms

| Term | Definition |
| --- | --- |
| Moral trace | Planned governance evidence trail linking decisions, moral context, outcomes, and reviewability. |
| Memory Model v2 | Planned memory architecture band for stronger identity, continuity, recall, and governed learning. |
| Birthday | Future first true Gödel-agent birth event, expected to require stronger identity, continuity, capability rebinding, citizen standing, and record semantics than current provisional citizens. |
| Contract market | Planned citizen-economics substrate for bounded contracts, bids, evaluation, delegation, lifecycle authority, fixtures, runner, and review summaries. |
| Inter-polis economics | Future economics across polis boundaries. This is not part of the current Runtime v2 foundation or v0.90.4 first contract-market proof unless a later decision explicitly changes scope. |
