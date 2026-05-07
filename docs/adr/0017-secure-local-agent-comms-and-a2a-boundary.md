# ADR 0017: Secure Local Agent Comms And A2A Boundary

- Status: Accepted
- Date: 2026-05-07
- Related milestone: v0.91
- Related release line: v0.91.0
- Builds on: ADR 0013, ADR 0015, ADR 0016

## Context

v0.90.5 introduced the first Agent Communication and Invocation Protocol
planning and governed-tool alignment. v0.91 promotes agent communication into a
first-class substrate feature for secure intra-polis messages and invocation
evidence, while keeping external or cross-polis transport out of scope until the
TLS, identity, authority, replay, and audit story is deliberately accepted.

This ADR is grounded in:

- `docs/milestones/v0.91/AGENT_COMMS_SPLIT_PLAN_v0.91.md`
- `docs/milestones/v0.91/features/A2A_EXTERNAL_AGENT_ADAPTER.md`
- `docs/milestones/v0.91/DEMO_MATRIX_v0.91.md`
- `docs/milestones/v0.91/FEATURE_PROOF_COVERAGE_v0.91.md`
- `docs/explainers/ACIP.md`
- `demos/v0.91/cognitive_being_flagship_demo.md`
- `demos/v0.91/chatgpt_gemini_task_handoff_demo.md`
- `demos/v0.91/chatgpt_gemini_claude_review_panel_demo.md`
- `adl/src/agent_comms.rs`
- `adl/src/agent_comms/transport.inc`
- `adl/src/agent_comms/orchestrate/proof_demo.inc`
- `adl/src/agent_comms/orchestrate/conformance.inc`
- `adl/src/agent_comms/a2a.inc`

## Decision

ADL adopts secure local Agent Communication and Invocation Protocol semantics as
the v0.91 communication boundary.

Inside a single CSM polis, agent messages are substrate-local events. They do
not leave the polis or cross an external network boundary by default, but they
still require security, reviewability, and governance.

This decision requires:

1. ACIP is a general communication substrate.

   ACIP covers conversation, consultation, bounded invocation, review,
   delegation, task handoff, negotiation, and broadcast. It is not merely an SRP
   helper or reviewer-agent convenience.

2. Messages are not authority.

   A message can request work or carry an invocation proposal. It does not
   grant tool authority, repository authority, merge authority, or cross-polis
   authority. Governed action still routes through the ADL authority stack.

3. Local does not mean unsecured.

   Intra-polis messages must carry authenticated sender and recipient identity,
   correlation and causal ordering, visibility policy, trace references,
   integrity posture, and redaction posture.

4. Sensitive payloads need protected references.

   Sensitive content should be carried by encrypted payload references or
   encrypted attachments when raw content should not be exposed to every
   audience. Review evidence must preserve metadata without private disclosure.

5. A2A is an adapter over ACIP, not a parallel communication model.

   Agent Cards and external-agent discovery are claims about identity and
   capability, not execution grants. External-agent invocation must consume ADL
   identity, invocation, policy, and trace boundaries.

6. External transport remains deferred until secure transport is accepted.

   Cross-polis or public-network communication remains unsupported or explicitly
   TLS/mTLS-equivalent-gated until v0.91.1 or later hardening accepts the
   transport, identity, replay, and audit posture.

## Rationale

Multi-agent systems become brittle when communication is only an implicit chat
transcript. ADL needs messages to be first-class runtime evidence, but the
project must not accidentally imply external federation or tool authority from
message exchange alone.

This decision lets v0.91 prove secure local communication while preserving the
stronger v0.91.1 and later security boundary for A2A hardening and external
transport.

## Consequences

### Positive

- Establishes ACIP as the shared substrate for local agent communication and
  invocation.
- Keeps the line between message, invocation proposal, and governed execution
  explicit.
- Gives A2A a safe adapter shape without creating a parallel authority model.
- Protects the project from premature external-transport claims.

### Negative

- Follow-on work must preserve ACIP compatibility when adding A2A, conformance,
  encryption, redaction, or external transport.
- Public docs must stay careful: v0.91 proves local, policy-bound
  communication evidence, not production cross-polis networking.

## Alternatives Considered

### 1. Treat A2A as the primary communication architecture

This would follow an external interoperability story first, but it would risk
letting external-agent claims bypass ADL identity, policy, and trace semantics.

### 2. Defer all communication work to v0.91.1

This would simplify v0.91, but it would leave the cognitive-being flagship and
review-policy work without a substrate-local communication proof.

### 3. Treat local comms as trusted because they stay inside the polis

This would be cheaper, but it would weaken ADL's security posture. Internal
agent communication still needs identity, integrity, visibility, redaction, and
audit evidence.

## Validation Evidence

The decision is supported by:

- the v0.91 Agent Comms split plan
- the A2A adapter feature doc
- demo matrix row D12 and flagship row D13
- feature-proof coverage mapping secure local comms and A2A boundary proof
- the `agent_comms` Rust module family and focused tests referenced by the
  proof coverage record

## Non-Claims

This ADR does not claim:

- public internet agent transport
- cross-polis federation
- production remote provider messaging
- A2A as execution authority
- external transport without TLS/mTLS-equivalent protection
- full v0.91.1 ACIP hardening completion
