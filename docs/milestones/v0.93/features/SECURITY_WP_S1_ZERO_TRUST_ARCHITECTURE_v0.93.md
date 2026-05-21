# Security WP-S1: Zero-Trust Architecture v0.93

## Metadata

- Feature Name: Zero-trust architecture
- Milestone Target: v0.93
- Status: planned
- Doc Role: supporting enterprise-security feature contract
- Feature Types: architecture, policy, artifact
- Proof Modes: schema, fixtures, tests, demo, review

## Purpose

Define the ADL polis trust model so no citizen, guest, operator, service, tool,
message, projection, or data boundary receives implicit trust. Every meaningful
boundary crossing should be authenticated, authorized, state-aware, traceable,
and deny-by-default.

## Dependencies

- v0.90.3 citizen state, access control, projection, sanctuary, and quarantine.
- v0.90.5 governed tools, Universal Tool Schema, and ADL Capability Contract.
- v0.91 secure Agent Comms and ACIP boundary planning.
- v0.92 identity, continuity, names, and capability envelopes.
- v0.92 ACIP binary schema, public schema catalog, and optional WebSocket
  carrier planning.

## Required Work Products

- Trust-boundary contract for citizen, guest, operator, service, tool, polis,
  communication, projection, and data boundaries.
- Transport-boundary contract for HTTP, local, mock, and WebSocket-carried ACIP
  messages, including connection/session authority and schema authority.
- Actor and zone model with explicit authentication and authority requirements.
- Default-deny fixture set for unauthorized, stale, ambiguous, or
  overprivileged boundary crossings.
- Reviewer-facing trust-boundary report.

## Invariants

- No boundary crossing succeeds by default.
- Internal polis traffic is not automatically trusted merely because it is
  internal.
- Human/operator action is not citizen action unless mediated through identity,
  Freedom Gate, signed trace, temporal anchoring, and policy.
- Communication never grants private-state or private-ToM inspection rights.
- A live WebSocket connection is not authority. Every ACIP message still needs
  schema, identity, sequence, policy, and trace acceptance.

## Demo Candidate

Show a citizen, service, tool, or operator request crossing a protected
boundary. The accepted case should cite identity, standing, capability, and
policy authority. The denied case should fail closed with a redacted reason.

For WebSocket-carried ACIP, include a near-miss case where the connection is
open but the message is denied because schema authority, identity, sequence, or
policy evidence is missing or invalid.

## Acceptance Criteria

- The trust-boundary contract names every protected actor and data boundary.
- Unauthorized and ambiguous boundary crossings have negative fixtures.
- WebSocket-carried ACIP messages have explicit accept/deny fixtures that do not
  rely on connection state as implicit trust.
- The review packet explains what was denied without leaking protected data.
- Later WP cards can implement the contract without inventing new trust zones.

## Non-Goals

- No production certification claim.
- No production external federation or cross-polis networking claim.
- No replacement of v0.90.3 private-state access-control rules.
