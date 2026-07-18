# ACIP/A2A Protobuf Implementation Decisions

## Metadata

- Feature Name: ACIP/A2A Protobuf Implementation Decisions
- Milestone Target: `v0.91.7`
- Status: bounded schema, transport, and access proof retained; broader protocol readiness not claimed
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

## WP-12 Schema Projection Decision

Issue `#4658` finalized the bounded schema/protobuf projection contract in
`adl::agent_comms::projection`.

The implemented posture is:

- JSON projection is the primary implemented carrier consumption posture for
  `v0.91.7`.
- The protobuf projection is a deterministic `proto3` profile over the current
  ACIP JSON schemas with stable message names, field numbers, JSON pointer
  bindings, repeated/scalar/message/enum classification, and required-field
  parity checks.
- WebSocket consumption remains text JSON frames for the #4900 loopback proof.
- Generated protobuf Rust types, protobuf wire encoding, and binary WebSocket
  frames remain non-claims until separately implemented and proven.

Retained evidence:

- `docs/milestones/v0.91.7/review/security/WP12_ACIP_SCHEMA_PROTOBUF_PROJECTION_4658.md`
- `docs/milestones/v0.91.7/review/security/wp12_acip_schema_protobuf_projection_4658.json`

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
