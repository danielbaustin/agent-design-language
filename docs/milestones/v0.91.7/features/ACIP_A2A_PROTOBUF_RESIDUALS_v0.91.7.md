# ACIP/A2A Protobuf Residuals

## Metadata

- Feature Name: ACIP/A2A Protobuf Residuals
- Milestone Target: `v0.91.7`
- Status: planned
- Owner: ADL maintainers
- Doc Role: primary
- Feature Types: architecture, schema, policy
- Proof Modes: schema, review

## Purpose

Resolve remaining JSON/protobuf/WebSocket/access-rule decisions after
the first ACIP/A2A bridge tranche.

## Scope

In scope:

- JSON projection consumption posture;
- protobuf wire-format decision;
- WebSocket transport residuals;
- access-rule residuals;
- provider and external-agent message posture.

Out of scope:

- full protocol implementation;
- broad interop certification;
- transport productization.

## Required Decisions

- Does `v0.92` consume JSON projection, protobuf, mock carrier, or a deferred route?
- Which access rules are required before any external-agent communication?
- Which WebSocket residuals block activation?
- Which protocol proofs are schema-only versus runtime?

## Dependencies

- `v0.91.6` ACIP/A2A/provider communications doc.
- Security residual readiness.
- Constructability Gate.

## Validation And Review

- Review schema and access decisions together.
- Validate JSON/protobuf projection determinism where claimed.
- Route unresolved wire-format choices explicitly.

## v0.92 Consumption

`v0.92` must know whether it consumes JSON projection, protobuf, mock carrier,
or a deferred route. Ambiguity is a blocker.

## Non-Goals

- No protocol completion claim.
- No default external-agent trust.
- No hidden WebSocket implementation.

## Blocker Rule

Any unresolved activation-path decision blocks v0.92 unless the operator
explicitly approves a non-claim with evidence and residual risk.
