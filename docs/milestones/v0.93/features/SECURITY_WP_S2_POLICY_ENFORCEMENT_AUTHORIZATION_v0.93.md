# Security WP-S2: Policy Enforcement And Authorization v0.93

## Metadata

- Feature Name: Policy enforcement and authorization
- Milestone Target: v0.93
- Status: planned
- Doc Role: supporting enterprise-security feature contract
- Feature Types: runtime, policy, artifact
- Proof Modes: tests, fixtures, schema, demo, review

## Purpose

Make IAM, delegation, standing, tool authority, capability envelopes, and
citizen/action policy enforceable under least privilege. v0.93 should not only
describe authority; it should produce reviewable allow/deny evidence that fails
closed when authority is missing, stale, ambiguous, revoked, or overbroad.

## Dependencies

- WP-S1 zero-trust architecture.
- v0.90.5 ACC/UTS governed-tool authority.
- v0.92 ACIP binary schema, public schema catalog, and optional WebSocket
  carrier planning.
- v0.92 identity, capability envelopes, and continuity.
- v0.93 delegation, IAM, standing, rights, and duties models.

## Required Work Products

- Policy decision contract for subject, action, resource, context, authority
  chain, standing, capability, and disposition.
- Per-message authorization contract for ACIP messages carried over persistent
  transports such as WebSocket.
- Least-privilege fixtures for citizen, guest, operator, service, tool, and
  delegated action.
- Deny-by-default tests for missing, expired, conflicting, or overbroad
  authority.
- Reviewer report explaining decisions without exposing private state.

## Invariants

- Missing authority denies.
- Stale or revoked authority denies.
- Delegation cannot exceed the delegator's authority.
- Tool authority must bind both capability and policy, not just tool name.
- Standing restrictions must constrain otherwise valid authority.
- Persistent transport sessions do not confer blanket permission. Each
  WebSocket-carried ACIP message must be evaluated as its own policy event.

## Demo Candidate

Show a delegated tool action. One request should be accepted only when identity,
standing, delegation, capability, tool authority, and policy all align. A near
miss should be denied with a reviewable reason.

For WebSocket-carried ACIP, show an authorized session carrying one allowed
message and one denied message so reviewers can see that session establishment
does not bypass per-message policy.

## Acceptance Criteria

- Policy decisions are deterministic for identical inputs.
- Allow and deny outputs cite explicit authority evidence.
- Negative fixtures cover missing, stale, revoked, and overbroad authority.
- Transport fixtures cover malformed, replayed, out-of-order, and
  policy-invalid WebSocket-carried ACIP messages.
- The contract composes with constitutional review and audit evidence.

## Non-Goals

- No blanket administrator bypass.
- No hidden operator override treated as citizen authority.
- No production IAM product claim.
