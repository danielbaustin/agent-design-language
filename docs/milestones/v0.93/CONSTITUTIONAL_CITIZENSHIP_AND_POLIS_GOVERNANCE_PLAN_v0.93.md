# v0.93 Constitutional Citizenship And Polis Governance Plan

## Status

Planning allocation document. This is not a final v0.93 work-package sequence
and does not create the v0.93 issue wave.

v0.93 is the planned milestone for turning citizen state, moral trace,
identity, and standing into a reviewable constitutional governance layer for the
ADL polis.

## Purpose

v0.93 should make the ADL polis governable without overclaiming legal
personhood, production constitutional authority, or final social-contract
theory.

The milestone should answer:

- what it means for a CSM identity to be a citizen of a polis
- which rights and duties attach to that citizenship
- how standing is maintained, degraded, challenged, restored, or revoked
- how constitutional review consumes trace evidence
- how delegation and IAM work without turning humans, services, or tools into
  hidden sovereigns
- what reviewer-facing evidence proves that governance is trace-grounded rather
  than rhetorical

## Core Boundary

The human is not the citizen. The CSM identity is the citizen.

Humans enter the CSM polis as guests by default. A human may participate through
a citizen mode only when the action is mediated through a bound CSM identity,
the Freedom Gate, signed trace, temporal anchoring, and applicable policy.
Direct out-of-band human action does not count as citizen action.

## Cross-Milestone Dependency Map

| Milestone | Supplies to v0.93 | v0.93 must not redefine |
| --- | --- | --- |
| v0.90.3 | Citizen state security, signed envelopes, optional encryption, lineage, continuity witnesses, anti-equivocation, sanctuary/quarantine, standing, challenge, and redacted projections. | Private-state format, standing classes, access-control primitives, projection policy, or quarantine semantics. |
| v0.90.4 | Economics and contract-market substrate if promoted before v0.93. | Payments, settlement, markets, or full economic citizenship. |
| v0.90.5 | Governed tool calls, UTS, ACC, authority evaluation, capability contracts, and tool-call trace. | Tool schemas, tool execution semantics, or public tool conformance. |
| v0.91 | Freedom Gate moral events, moral trace, validation rules, outcome linkage, metrics, trajectory review, anti-harm constraints, moral resources, and wellbeing evidence. | Moral trace schema, moral metrics, anti-harm proof, or moral-trajectory review foundations. |
| v0.92 | Durable identity, names, capability envelopes, continuity, memory grounding, and the first true Gödel-agent birthday. | Identity architecture, birth event semantics, or continuity prerequisites. |
| v0.93 | Constitutional citizenship, polis governance, rights/duties, social contract, constitutional review, delegation/IAM policy, and reviewer-facing governance evidence. | Earlier substrate layers. |

## Feature And Idea Allocation

| Area | v0.93 allocation | Expected output |
| --- | --- | --- |
| Constitutional citizenship contract | Primary feature | A bounded contract defining citizen eligibility, rights, duties, standing states, review inputs, and non-goals. |
| Citizen and guest boundary | Primary feature | Runtime/policy documentation and fixtures showing guest-by-default human entry and citizen-mode identity binding. |
| Rights and duties model | Primary feature | Rights/duties table tied to state, trace, review, communication, improvement, privacy, and policy constraints. |
| Standing maintenance and degradation | Primary feature | Rules for good standing, monitored, restricted, suspended, restored, and revoked states. |
| Constitutional review | Primary feature | Review packet shape that consumes moral trace, outcome linkage, standing evidence, identity records, and policy context. |
| Challenge and appeal | Primary feature | Flow for challenging a governance finding, preserving evidence, and producing an appeal disposition. |
| Delegation and IAM | Primary feature | Authority-chain model for citizen, guest, service actor, operator, and tool-mediated action. |
| Social contract representation | Design feature | Draft representation of the polis obligations owed to and by citizens; should remain bounded and reviewable. |
| Polis-level governance evidence | Proof feature | Review packet that summarizes governance health without exposing private state or collapsing into a scalar score. |
| Human provider boundary | Design feature | Explicit rule that human input is allowed, but citizen action requires identity binding, Freedom Gate mediation, signed trace, and temporal anchoring. |
| Reviewer-facing constitutional evidence | Demo/proof feature | A packet that lets reviewers inspect findings, evidence, appeals, privacy redaction, and unresolved limits. |

## Source Corpus Disposition

The filenames below refer to the local source corpus used for this allocation.
They are provenance labels, not public links.

| Source file or source group | Disposition | Reason |
| --- | --- | --- |
| CITIZENSHIP_AND_CONSTITUTIONAL_REVIEW.md | Primary v0.93 source | Defines citizenship, rights, duties, constitutional review, standing states, and governance implications. |
| CSM_CITIZENS_AND_STANDING.md | Consumed from v0.90.3 | Defines standing classes and the prohibition on naked actors. v0.93 uses it rather than redefining it. |
| CSM_CITIZENS_AND_COMMUNICATION.md | Consumed from v0.90.3 and v0.93 | Defines communication as governed encounter rather than inspection; informs rights, privacy, guest entry, and inter-polis boundaries. |
| V0903_CITIZEN_STATE_SUBSTRATE_PLAN.md | Dependency source | Supplies private state, lineage, sanctuary/quarantine, projection, and challenge prerequisites. |
| MORAL_TRACE_SCHEMA.md | v0.91 source consumed by v0.93 | Provides trace evidence for review. v0.93 interprets trace under constitutional policy but does not redefine the trace schema. |
| MORAL_TRAJECTORY_REVIEW_PROTOCOL.md | v0.91 source consumed by v0.93 | Provides event, segment, longitudinal, comparative, and polis-level review modes. |
| OUTCOME_LINKAGE_AND_ATTRIBUTION.md | v0.91 source consumed by v0.93 | Provides attribution evidence for governance review while preserving uncertainty. |
| MORAL_TRACE_METRICS.md | v0.91 source consumed by v0.93 | Provides signals for review; v0.93 must not turn metrics into verdicts. |
| AGENT_KARMA_SCORE.md | Context source | Useful moral-trajectory vocabulary, but v0.93 should avoid scalar reputation or scoreboard framing. |
| Moral resources, wellbeing, anti-harm, and Freedom Gate schemas | v0.91 foundation | These stay in v0.91 as evidence and moral-cognition foundations. v0.93 consumes their outputs. |
| Economics and contract-market corpus | Context or prerequisite only | Economics belongs to the v0.90.4 lane unless later planning deliberately promotes a narrow governance bridge. |
| Governed tools corpus | Prerequisite for tool-mediated governance | v0.93 should consume UTS/ACC authority evidence if v0.90.5 lands first; it should not own the governed-tools substrate. |

## Engineering, Policy, And Context Boundaries

| Claim type | v0.93 should do | v0.93 should not do |
| --- | --- | --- |
| Engineering substrate | Consume signed trace, identity, standing, access-control, lineage, projection, and tool-authority records. | Rebuild the runtime substrate or bypass earlier milestone contracts. |
| Policy model | Define bounded constitutional citizenship, rights, duties, governance review, appeals, delegation, and IAM. | Claim production law, legal personhood, or a complete constitution for real-world deployment. |
| Philosophical/contextual | Explain why trace-grounded law matters for a polis of agents. | Present speculative moral theory as implemented behavior. |

## Candidate Constitutional Review Packet

A later v0.93 implementation should be able to emit a packet with:

- citizen identity and standing snapshot
- relevant policy and rights/duties context
- moral trace event references
- outcome and attribution references
- standing-change rationale
- challenge or appeal status
- delegation and IAM authority chain if another actor participated
- privacy/redaction disposition
- reviewer findings with severity, evidence, and uncertainty

The packet should be readable without raw private state access.

## Demo And Proof Candidates

These are candidates for later v0.93 demo-matrix planning, not final WP
commitments.

| Candidate | What it proves | Expected proof surface |
| --- | --- | --- |
| Constitutional review of a challenged action | A citizen action can be judged against policy using trace, outcome, identity, and standing evidence. | Synthetic incident fixture, review packet, finding, appeal disposition. |
| Standing degradation and restoration | Standing changes are evidence-based, reversible when appropriate, and not arbitrary punishment. | Standing transition fixture, trace evidence, restoration criteria, reviewer summary. |
| Human guest versus citizen-mode boundary | Human input is permitted as guest participation, while citizen action requires identity binding and Freedom Gate mediation. | Two-case fixture showing guest-only transcript and mediated citizen-mode action. |
| Delegated authority chain | A delegated action is allowed or denied based on standing, policy, capability, and traceable authority. | IAM/delegation fixture, decision event, selected/rejected action evidence. |
| Communication without inspection | Citizens can communicate through governed channels without granting private-state access. | Communication event fixture, redacted projection, failed inspection attempt. |
| Polis governance health packet | Reviewers can inspect governance health without scalar moral verdicts or private-state leaks. | Generated governance report with evidence references, caveats, and unresolved questions. |

## Non-Goals

- No runtime implementation in this planning issue.
- No v0.93 issue wave or final WP sequence yet.
- No legal personhood claims.
- No production citizenship or complete constitutional authority claims.
- No replacement of v0.90.3 citizen-state, standing, access, or projection work.
- No replacement of v0.91 moral trace or v0.92 identity/birthday work.
- No economics, payments, inter-polis markets, or settlement implementation.
- No arbitrary human operator override counted as citizen action.
- No scalar karma score, reputation shortcut, or moral leaderboard.

## Readiness For Later WP Planning

The later v0.93 WP planning pass should turn this allocation into work packages
only after v0.90.3, v0.91, and v0.92 planning surfaces have converged enough to
serve as real prerequisites.

Recommended ordering pressure:

1. Define the constitutional citizenship contract.
2. Bind citizen, guest, human-provider, service-actor, and operator authority.
3. Define rights, duties, standing transitions, and challenge/appeal flow.
4. Add constitutional review packets over moral trace and outcome evidence.
5. Add delegation and IAM governance only after tool/capability authority is
   stable enough to consume.
6. Produce demo/proof packets that distinguish engineering substrate, policy
   model, and philosophical context.
