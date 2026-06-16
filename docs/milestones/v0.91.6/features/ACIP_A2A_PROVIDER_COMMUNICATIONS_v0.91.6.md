# ACIP, A2A, And Provider Communications

## Metadata

- Feature Name: ACIP, A2A, And Provider Communications
- Milestone Target: `v0.91.6`
- Status: planned
- Owner: ADL maintainers
- Doc Role: primary
- Feature Types: architecture, policy, schema
- Proof Modes: schema, review

## Purpose

Define first-tranche communication boundaries for ACIP, A2A, and provider
messages before `v0.92` consumes agent-communication surfaces.

## Scope

In scope:

- schema catalog and message families;
- access rules and authority boundaries;
- external-agent posture;
- provider-message boundary;
- WebSocket support posture;
- deterministic JSON projection;
- protobuf decision point.

Out of scope:

- full protocol implementation;
- broad transport productization;
- residual protobuf/wire-format closure owned by `v0.91.7`.

## Required Decisions

- Which schemas are canonical in `v0.91.6`?
- Which messages can cross provider or polis boundaries?
- Is protobuf required before `v0.92`, or is JSON projection sufficient?
- Which access-control failures are terminal?

## Dependencies

- Security bridge and CAV feature doc.
- Constructability Gate residual in `v0.91.7`.
- Existing ACIP and WebSocket planning notes.

## Validation And Review

- Review schema catalog for access and privacy boundaries.
- Validate JSON projection determinism before implementation claims.
- Route protobuf residuals explicitly to `v0.91.7` if unresolved.

## v0.92 Consumption

`v0.92` may consume only the reviewed communication posture. If protobuf,
WebSocket, or access rules remain unresolved, they must be blocked, deferred,
or routed before activation.

## Non-Goals

- No protocol completion claim.
- No implicit external-agent trust.
- No unreviewed provider-message exposure.
