# ACIP Hardening

## Metadata

- Feature Name: ACIP Conformance And Local Encryption Hardening
- Milestone Target: `v0.91.1`
- Status: landed
- Planned WP Home: WP-13
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

## Landed Surface

- `adl/src/runtime_v2/acip_hardening.rs`
- `adl/src/runtime_v2/tests/acip_hardening.rs`
- `adl/src/agent_comms/orchestrate/conformance.inc`
- `adl/src/runtime_v2/agent_lifecycle_state.rs`
- `adl/src/runtime_v2/access_control.rs`
- `adl/src/runtime_v2/private_state_observatory.rs`

## Proof Route

- Contract accessor: `runtime_v2_acip_hardening_contract()`
- Generated packet path: `runtime_v2/acip/acip_hardening_packet.json`
- Tracked proof surfaces:
  - `adl/src/runtime_v2/acip_hardening.rs`
  - `adl/src/runtime_v2/tests/acip_hardening.rs`
  - `adl/src/agent_comms/orchestrate/conformance.inc`
  - `adl/src/runtime_v2/agent_lifecycle_state.rs`
- Focused validation:
  - `cargo test --manifest-path adl/Cargo.toml runtime_v2_acip_hardening -- --nocapture`
  - `cargo test --manifest-path adl/Cargo.toml runtime_v2_agent_lifecycle_state -- --nocapture`
  - `cargo test --manifest-path adl/Cargo.toml acip_conformance_report -- --nocapture`

## Non-Claims

- This slice does not prove cross-polis or external transport security.
- This slice does not implement live TLS, mutual-TLS, or remote key exchange.
- This slice does not bypass lifecycle, Freedom Gate, ACC, trace, or redaction boundaries.
- This slice does not prove v0.92 federation, identity rebinding, or birthday completion.
