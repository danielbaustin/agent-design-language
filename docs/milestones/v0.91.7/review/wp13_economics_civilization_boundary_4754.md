# WP-13 Economics/Civilization Boundary (#4754)

## Scope

`#4754` keeps economics and civilization language out of the `v0.92` activation
path unless a later tracked issue explicitly promotes a bounded test with
operator approval.

## Implemented Surface

- Runtime module:
  `adl/src/runtime_v2/economics_civilization_boundary.rs`
- Runtime tests:
  `adl/src/runtime_v2/tests/economics_civilization_boundary.rs`
- Feature doc:
  `docs/milestones/v0.91.7/features/ECONOMICS_CONTEXT_DECISION_v0.91.7.md`

## Decision

Economics is context-only for `v0.92`.

`v0.92` may consume:

- scheduler and resource-stewardship context;
- public claim non-claims;
- future issue-routing inputs.

`v0.92` may not consume:

- payments or settlement implementation;
- market mechanism proof;
- civilization runtime;
- autonomous economy;
- runtime economic optimization;
- product-readiness evidence.

## Validation Plan

Focused local proof:

```sh
cargo test --manifest-path adl/Cargo.toml runtime_v2_economics_civilization_boundary
```

The tests validate the context-only posture, deny promoted activation tests
without a separate approved issue, preserve required non-claims, and assert
stable path-safe JSON serialization.

## Residual Risk

This packet does not implement payments, markets, settlement, economic
optimization, or civilization mechanics. That is intentional scope control for
`v0.92`; any future promotion needs a new tracked issue and proof plan.
