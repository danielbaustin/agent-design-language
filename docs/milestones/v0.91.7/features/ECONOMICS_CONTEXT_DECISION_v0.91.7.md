# Economics Context Decision

## Metadata

- Feature Name: Economics Context Decision
- Milestone Target: `v0.91.7`
- Status: implemented
- Owner: ADL maintainers
- Doc Role: primary
- Feature Types: policy, runtime boundary
- Proof Modes: review, tests

## Purpose

Decide whether economics is context-only for `v0.92` or requires explicit
activation tests, and keep that decision executable through Runtime v2 boundary
validation.

## Scope

In scope:

- economics context boundary;
- activation-test decision;
- relationship to governance, resource stewardship, and later payments work;
- non-goals for `v0.92`.
- executable Runtime v2 allowlist/non-claim validation.

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
- Run `cargo test --manifest-path adl/Cargo.toml runtime_v2_economics_civilization_boundary`.

## Implemented Boundary

`#4754` adds `runtime_v2.economics_civilization_boundary.v1` in
`adl/src/runtime_v2/economics_civilization_boundary.rs`.

The boundary packet requires:

- `activation_posture = context_only_for_v0_92`;
- no promoted activation tests;
- a fixed `v0.92` allowlist limited to scheduler/resource-stewardship context,
  public non-claims, and future issue-routing inputs;
- postponed rows for payments/settlement, market mechanisms,
  civilization-scale economics, and runtime economic optimization;
- promotion gates for any future activation test: operator approval, tracked
  issue, bounded test plan, security/governance review, and retained proof.

## v0.92 Consumption

Default posture: economics is context-only for `v0.92` unless an explicit
operator decision promotes a bounded test requirement through a tracked issue.
`v0.92` may consume the executable non-claim boundary; it may not consume this
as payments, markets, civilization runtime, autonomous economy, economic
optimization, or product-readiness evidence.

## Non-Goals

- No payments implementation.
- No market mechanism proof.
- No economics-led activation scope expansion.
