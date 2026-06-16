# Constructability Gate

## Metadata

- Feature Name: Constructability Gate
- Milestone Target: `v0.91.7`
- Status: planned
- Owner: ADL maintainers
- Doc Role: primary
- Feature Types: architecture, policy, schema
- Proof Modes: schema, review, tests

## Purpose

Define the gate that separates provisional cognition from authoritative shared
reality.

## Scope

In scope:

- construction-event schema;
- external-anchor schema;
- admissibility validator;
- shared-reality boundary;
- proof path for constructed claims.

Out of scope:

- universal truth adjudication;
- runtime implementation;
- replacing human/operator review.

## Required Decisions

- Which internal events may become construction events?
- Which external anchors are admissible?
- Which validator failures block shared-reality publication?
- Which constructability proofs are required before `v0.92`?

## Dependencies

- Curiosity Engine feature doc.
- Security residual readiness.
- ACIP/A2A residual decisions.

## Validation And Review

- Review schemas for determinism and evidence boundaries.
- Validate that provisional claims cannot become public truth without anchors.
- Require blocked/deferred/routed status for missing validators.

## v0.92 Consumption

`v0.92` may consume Constructability only as a reviewed boundary and proof
route. It must not present provisional cognition as authoritative shared
reality.

## Non-Goals

- No runtime gate implementation.
- No universal epistemic authority claim.
- No public truth claim without anchors.
