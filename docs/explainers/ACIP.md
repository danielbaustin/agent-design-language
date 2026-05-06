# Agent Communication And Invocation Protocol

Agent Communication and Invocation Protocol (ACIP) is ADL's general substrate
for agent-to-agent communication.

The core rule is:

> Agents communicate through messages, not through hidden prompts.

An ACIP message may carry natural language, structured payload references,
artifact references, memory references, or an invocation request. It must still
preserve identity, ordering, visibility, traceability, and policy boundaries.

## What ACIP Covers

ACIP is designed to cover more than reviewer-agent work. It supports:

- ordinary conversation
- consultation
- bounded invocation
- review
- delegation
- task handoff
- negotiation
- broadcast

Those modes share one message substrate instead of creating a different
protocol for every agent relationship.

## Message And Invocation Boundary

The stable communication shape is:

```text
message -> interpretation -> optional invocation -> response
```

Messages can ask for work. They do not grant tool authority, repository
authority, merge authority, or cross-polis authority by themselves.

Governed action still routes through UTS, ACC, policy evaluation, Freedom Gate,
and governed execution. ACIP gives the request a durable envelope; ADL's runtime
governance decides what may actually happen.

## Security Boundary

The v1 ADL direction is secure intra-polis communication first. Internal
messages still need confidentiality, integrity, identity, redaction, and audit
surfaces, but they do not need to cross an external network boundary.

External or cross-polis communication remains a later security milestone until
the transport, TLS, identity, and zero-trust story is fully implemented.

## Why This Matters

ACIP turns multi-agent systems from an implicit chat transcript into an
inspectable communication system. Reviewers can ask:

- who sent the message?
- who received it?
- what did it request?
- what authority was claimed?
- what policy decision followed?
- what response, refusal, or artifact resulted?
- what should each audience be allowed to see?

That is how ADL keeps multi-agent collaboration expressive without losing
determinism, governance, or reviewability.

## Deeper References

- [Agent Comms v1 feature surface](../milestones/v0.90.5/features/AGENT_COMMS_v1.md)
- [v0.91 agent comms split plan](../milestones/v0.91/AGENT_COMMS_SPLIT_PLAN_v0.91.md)
- [A2A external agent adapter plan](../milestones/v0.91/features/A2A_EXTERNAL_AGENT_ADAPTER.md)
- [v0.91 ChatGPT + Gemini task handoff demo](../../demos/v0.91/chatgpt_gemini_task_handoff_demo.md)
- [v0.91 ChatGPT + Gemini + Claude review panel demo](../../demos/v0.91/chatgpt_gemini_claude_review_panel_demo.md)

