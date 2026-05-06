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

## Required Work Products

- Trust-boundary contract for citizen, guest, operator, service, tool, polis,
  communication, projection, and data boundaries.
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

## Demo Candidate

Show a citizen, service, tool, or operator request crossing a protected
boundary. The accepted case should cite identity, standing, capability, and
policy authority. The denied case should fail closed with a redacted reason.

## Acceptance Criteria

- The trust-boundary contract names every protected actor and data boundary.
- Unauthorized and ambiguous boundary crossings have negative fixtures.
- The review packet explains what was denied without leaking protected data.
- Later WP cards can implement the contract without inventing new trust zones.

## Non-Goals

- No production certification claim.
- No external federation or cross-polis networking claim.
- No replacement of v0.90.3 private-state access-control rules.
