# Structured Intent Prompt

Template: 1.0.0

Issue: 5865

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Integrate a maintained QUIC/TLS stack with bounded authenticated channels and no custom cryptography or framing.

## Required Outcome

Integrate a maintained QUIC/TLS stack with bounded authenticated channels and no custom cryptography or framing.

## Scope

- adl-runtime/src/distributed/transport.rs
- adl-runtime/tests/distributed_transport.rs
- adl-runtime/Cargo.toml
- adl-runtime/Cargo.lock

## Authority

- Issue 5865 exclusively owns the declared paths
- WP-04-IMP issue 5862 coordinates only
- WP-04.16 alone owns final module registration
- No sibling, Runtime v2, or v0.93 authority

## Assumptions

- none

## Operator Constraints

- Do not start before #5821 is terminal
- Bind only the exact exclusive paths
- Use nonzero exact test selection
- Fix all actionable pre-PR findings
