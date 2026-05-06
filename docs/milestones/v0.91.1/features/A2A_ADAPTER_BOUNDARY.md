# A2A Adapter Boundary

## Metadata

- Feature Name: A2A Adapter Boundary And Compatibility Plan
- Milestone Target: `v0.91.1`
- Status: planned
- Planned WP Home: WP-13
- Source Docs: `.adl/docs/TBD/a2a/`
- Proof Modes: architecture, fixtures, review

## Purpose

Define A2A as an adapter over ADL's communication substrate, not a competing
communication architecture. A2A compatibility must preserve ADL identity,
authority, redaction, trace, and local/external transport boundaries.

## Scope

In scope:

- A2A-over-ACIP compatibility mapping.
- Adapter boundary and non-claims.
- Fixtures for allowed, denied, and unsupported adapter cases.
- Relationship to ACIP and ACC authority.

Out of scope:

- External federation readiness.
- Bypassing ACIP or ACC.
- Treating A2A as canonical internal comms.

## Acceptance Criteria

- Adapter flow cannot bypass ADL authority checks.
- Compatibility docs preserve one communication model.
- External transport remains gated until security posture is implemented.
