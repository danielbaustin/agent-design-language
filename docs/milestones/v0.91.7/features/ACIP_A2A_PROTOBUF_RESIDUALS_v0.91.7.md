# ACIP/A2A Protobuf Implementation Decisions

## Metadata

- Feature Name: ACIP/A2A Protobuf Implementation Decisions
- Milestone Target: `v0.91.7`
- Status: planned
- Owner: ADL maintainers
- Doc Role: primary
- Feature Types: architecture, schema, policy
- Proof Modes: schema, review

## Purpose

Resolve JSON/protobuf/WebSocket/access-rule implementation decisions after
the first ACIP/A2A readiness tranche.

## Scope

In scope:

- JSON projection consumption posture;
- protobuf wire-format decision;
- WebSocket transport decisions;
- access-rule decisions;
- provider and external-agent message posture.

Out of scope:

- full protocol implementation;
- broad interop certification;
- transport productization.

## Required Decisions

- Does `v0.92` consume JSON projection, protobuf, or another implemented carrier?
- Which access rules are required before any external-agent communication?
- Which WebSocket decisions block activation?
- Which protocol proofs are schema-only versus runtime?

## Dependencies

- `v0.91.6` ACIP/A2A/provider communications doc.
- Security implementation readiness.
- Constructability Gate.

## Validation And Review

- Review schema and access decisions together.
- Validate JSON/protobuf projection determinism where claimed.
- Resolve unresolved wire-format choices or record them as evidence-backed blockers with operator approval.

## v0.92 Consumption

`v0.92` must know whether it consumes JSON projection, protobuf, or another
implemented carrier. Ambiguity is a blocker.

## Non-Goals

- No protocol completion claim.
- No default external-agent trust.
- No hidden WebSocket implementation.

## Blocker Rule

Any unresolved activation-path decision blocks v0.92 unless the operator
explicitly scopes it out with evidence and risk.
