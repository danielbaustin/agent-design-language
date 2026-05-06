# Security WP-S3: Secrets, Keys, And Cryptographic Trust v0.93

## Metadata

- Feature Name: Secrets, keys, and cryptographic trust
- Milestone Target: v0.93
- Status: planned
- Doc Role: supporting enterprise-security feature contract
- Feature Types: runtime, artifact, policy
- Proof Modes: fixtures, tests, schema, demo, review

## Purpose

Represent signing, encryption, key custody, key rotation, revocation, sealed
state access, and internal ACIP encryption as explicit lifecycle contracts
rather than hidden environment folklore.

## Dependencies

- WP-S1 zero-trust architecture.
- WP-S2 policy enforcement and authorization.
- v0.90.3 signed envelopes, local sealing, private state, and witnesses.
- v0.91 secure Agent Comms and ACIP planning.
- v0.92 identity and capability envelopes.

## Required Work Products

- Key/secrets lifecycle contract covering creation, custody, scope, use,
  rotation, revocation, expiration, and destruction.
- Signing and encryption acceptance rules tied to identity, standing, policy,
  and lifecycle state.
- Fixtures for accepted current keys and denied stale, revoked, malformed, or
  wrong-scope keys.
- Internal ACIP encryption and message-acceptance proof surface.

## Invariants

- Revoked keys cannot authorize new actions.
- Rotated keys change what signatures, messages, and sealed-state access are
  accepted.
- Secrets are never emitted into review packets or public projections.
- Encryption does not replace authorization.
- Signature validity does not imply policy permission.

## Demo Candidate

Show an internal ACIP message or sealed-state access accepted before key
rotation and denied after revocation, with audit evidence linking the lifecycle
change to the disposition.

## Acceptance Criteria

- Key lifecycle events are deterministic artifacts.
- Rotation and revocation have explicit negative cases.
- Review output cites key identity and lifecycle state without exposing secret
  material.
- The feature composes with audit, incident, and zero-trust evidence.

## Non-Goals

- No production KMS integration claim.
- No external cross-polis TLS/federation claim.
- No secret material in docs, fixtures, logs, or reviewer packets.
