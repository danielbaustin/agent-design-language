# Economics Context Decision

## Metadata

- Feature Name: Economics Context Decision
- Milestone Target: `v0.91.7`
- Status: planned
- Owner: ADL maintainers
- Doc Role: primary
- Feature Types: policy
- Proof Modes: review

## Purpose

Decide whether economics is context-only for `v0.92` or requires explicit
activation tests.

## Scope

In scope:

- economics context boundary;
- activation-test decision;
- relationship to governance, resource stewardship, and later payments work;
- non-goals for `v0.92`.

Out of scope:

- payment/settlement implementation;
- marketplace productization;
- economic optimization runtime.

## Required Decisions

- Is economics context-only for `v0.92`?
- If tests are required, what do they prove?
- Which economics surfaces are explicitly postponed to `v0.94.1` or post-MVP?
- Which claims are unsafe before governance/security mature?

## Dependencies

- Governance/security milestone planning.
- Resource stewardship history.
- `v0.94.1` payments/settlement planning if promoted later.

## Validation And Review

- Review economics language for scope creep.
- Ensure no payment/product claim enters `v0.92` without a tracked decision.
- Record implementation that does not land before `v0.92` as explicitly postponed, with operator approval where it affects activation claims.

## v0.92 Consumption

Default posture: economics is context-only for `v0.92` unless an explicit
operator decision promotes a bounded test requirement.

## Non-Goals

- No payments implementation.
- No market mechanism proof.
- No economics-led activation scope expansion.
