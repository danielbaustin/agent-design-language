# ACIP Hardening

## Metadata

- Feature Name: ACIP Conformance And Local Encryption Hardening
- Milestone Target: `v0.91.1`
- Status: planned
- Planned WP Home: WP-12
- Source Docs: `.adl/docs/TBD/acip/`
- Proof Modes: fixtures, schema, tests, review

## Purpose

Harden intra-polis Agent Communication and Invocation Protocol messages so
agent-to-agent communication is authenticated, encrypted or encryption-ready,
traceable, redacted, and policy-bound before v0.92 identity work depends on it.

## Scope

In scope:

- Secure local communication envelope.
- Authentication, redaction, and conformance fixtures.
- Rejection cases for malformed, unsigned, unauthorized, or overexposed
  messages.
- External transport boundary language.

Out of scope:

- Cross-polis communication without TLS or mutual-TLS equivalent protection.
- Hidden side channels.
- Replacing ACC or Freedom Gate authority checks.

## Acceptance Criteria

- Local messages are identity-bound and reviewable.
- Invalid messages fail closed.
- External communication remains explicitly gated.
