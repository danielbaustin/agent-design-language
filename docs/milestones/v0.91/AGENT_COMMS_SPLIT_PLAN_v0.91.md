# v0.91 Agent Communication Split Plan

## Status

First-pass milestone allocation for Agent Communication and Invocation Protocol
1.0 across v0.90.5, v0.91, v0.91.1 adjacent-system hardening, and the v0.92
birthday boundary. The reviewed candidate issue wave is now tracked in
[WP_ISSUE_WAVE_v0.91.yaml](WP_ISSUE_WAVE_v0.91.yaml), with card-authoring
requirements in [WP_EXECUTION_READINESS_v0.91.md](WP_EXECUTION_READINESS_v0.91.md).
This plan sets the communication implementation boundary for that wave.

## Source Inputs

- `.adl/docs/TBD/AGENT_COMMUNICATION_AND_INVOCATION_PROTOCOL.md`
- `.adl/docs/TBD/AGENT_COMMS_1_0_PLAN.md`
- v0.90.5 governed-tool, UTS, ACC, and tool-call authority planning
- v0.91 moral-governance, wellbeing, review, and cognitive-being planning
- v0.92 identity, continuity, and birthday planning

## Core Decision

Agent communication is a first-class ADL feature, not a narrow SRP or
review-agent helper.

Inside a single CSM polis, agent-to-agent messages should be substrate-local
events. They do not leave the polis or cross an external network boundary by
default. Even so, they still need security: authenticated identity, integrity
protection, visibility policy, redaction, and encryption for sensitive payloads
or attachments.

External or cross-polis communication should remain unsupported or explicitly
gated until the transport has TLS or mutual-TLS-equivalent protection, stable
identity, authority, replay, and audit semantics.

## Security Posture

| Layer | Required posture |
| --- | --- |
| Local intra-polis message | Authenticated sender and recipient, authority-bound visibility, integrity-protected envelope, trace event, redaction policy. |
| Sensitive local payload | Encrypted payload reference or encrypted attachment, key policy, access log, and reviewable metadata without private disclosure. |
| Local invocation | Explicit invocation contract, authority binding, Freedom Gate or ACC event where relevant, deterministic response record. |
| External transport | Deferred until TLS or mTLS-equivalent transport is available and policy-bound. |
| Cross-polis federation | Deferred beyond v0.91 unless deliberately scoped as planning only. |

## v0.90.5 Remainder

v0.90.5 may complete only the planning and prerequisite alignment needed for
governed tools and early review/coding-agent work:

- ACIP terminology and message-not-card boundary
- high-level envelope and invocation shape
- intra-polis security stance
- relationship to UTS, ACC, Freedom Gate, trace, and reviewer/coding-agent
  specializations
- no external transport implementation

## v0.91 Core Scope

v0.91 should make ACIP usable as a real substrate feature:

- secure message envelope v0
- agent identity binding for sender, recipient, audience, and delegation
- conversation, correlation, causal-ordering, and trace references
- invocation contract records
- local mailbox or router semantics inside the polis
- visibility, redaction, and self-access policy
- sensitive payload references and encryption-policy hooks
- durable structured planning and `SRP` policy targets for specialized
  invocation and review flows
- review-agent and coding-agent specializations only as examples, not as the
  whole feature
- fixtures for delegation, handoff, consultation, refusal, and review
- negative fixtures for unauthorized, unencrypted, unaudited, or ambiguous
  communication

## v0.91.1 Completion Scope

v0.91.1 is the home for adjacent-system ACIP completion and hardening:

- expanded conformance suite
- redaction and replay hardening
- stronger local encryption fixture coverage
- more invocation specializations
- A2A external-agent adapter implementation and hardening
- cross-agent demo variants
- review finding remediation
- capability-testing probes for communication behavior

By the end of v0.91 and v0.91.1 together, ACIP should be implemented strongly
enough that it is no longer a planning-only prerequisite for birthday work.

## Before v0.92 Birthday

Before the first birthday, ADL should be able to show that:

- agents can communicate inside the polis through authenticated, traceable,
  policy-bound messages
- private or sensitive payloads are protected instead of being leaked through
  transcripts
- invocation is explicit and authority-bound rather than implicit prompt magic
- reviewer-facing traces can explain communication without exposing private
  content
- external communication remains unsupported or TLS/mTLS-gated
- v0.92 identity can consume communication evidence without redefining ACIP

## Deferrals

- public internet agent transport
- cross-polis federation
- reputation systems
- social-contract negotiation
- constitutional citizenship semantics
- production remote provider messaging
- full secure transport implementation before TLS or mTLS-equivalent policy is
  accepted

## A2A Alignment

A2A should be treated as one governed adapter over ACIP rather than a separate
communications architecture.

That means:

- Agent Cards remain identity and capability claims, not execution grants
- all external-agent invocation still routes through explicit ADL invocation
  boundaries
- A2A planning belongs in `v0.91`
- runnable A2A adapter implementation and hardening belongs in `v0.91.1`
