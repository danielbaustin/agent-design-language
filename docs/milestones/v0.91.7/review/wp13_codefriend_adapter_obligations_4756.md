# WP-13 CodeFriend Adapter Obligations Boundary (#4756)

## Status

Implemented as an executable Runtime v2 boundary packet.

## Purpose

This packet records the pre-v0.92 CodeFriend v1 / adapter v2 obligation truth.
It identifies the smallest CodeFriend v1 proof surfaces, names the adapter v2
dependencies, and records what the MVP/birthday path may consume before the
full external-repo proof is implemented.

The complete product build plan is tracked separately at
`docs/planning/codefriend/CODEFRIEND_V1_BUILD_PLAN.md`. That plan defines the
full CodeFriend v1 feature set and implementation sequence; this packet proves
the pre-v0.92 boundary and handoff to that plan.

## Runtime Contract

- Contract: `runtime_v2.codefriend_adapter_obligations.v1`
- Rust module: `adl/src/runtime_v2/codefriend_adapter_obligations.rs`
- Test module: `adl/src/runtime_v2/tests/codefriend_adapter_obligations.rs`
- Full v1 plan: `docs/planning/codefriend/CODEFRIEND_V1_BUILD_PLAN.md`
- Artifact path declared by the contract:
  `docs/milestones/v0.91.7/review/codefriend_adapter_obligations_4756/boundary_packet.json`
- Owning issue: `#4756`

## Smallest CodeFriend v1 Proof

The contract requires exactly four proof surfaces before CodeFriend v1 can claim
external-repo readiness:

- repository review packet
- specialist review lanes with synthesis and findings truth
- redaction/publication gate
- human-readable report generated from retained evidence

These surfaces are consumed by `v0.95` CodeFriend external-repo proof packaging,
not by `v0.92` birthday readiness.

## Adapter v2 Dependencies

The contract requires adapter v2 proof packaging to own:

- an external-repo input manifest
- a portable execution adapter
- retained proof artifacts
- operator publication approval

Each dependency is owned by `v0.95`, and none is allowed to block `v0.92`
birthday readiness. This keeps CodeFriend visible without turning later product
work into hidden launch scope.

## Complete v1 Plan

`CODEFRIEND_V1_BUILD_PLAN.md` defines the complete v1 product target:

- product shell
- adapter v2
- evidence core
- architecture cognition
- executable governance
- specialist review engine
- human review and publication gate
- memory and longitudinal intelligence
- integrations
- evaluation and quality

Future tracked issues must implement these features through the planned
milestones and release gates before CodeFriend v1 can be declared complete.

## v0.92 Consumption

`v0.92` may consume this packet only as bounded handoff truth:

- CodeFriend/adapter obligations are tracked and bounded, not complete.
- CodeFriend v1 proof packaging routes to `v0.95` MVP convergence.
- The complete v1 feature plan exists, but is not implementation evidence.
- Product-roadmap mentions must preserve evidence-boundary and human-review
  language.
- Birthday readiness may not depend on external-repo CodeFriend execution.

## Non-Claims

This packet does not claim:

- CodeFriend v1 product completion
- adapter v2 implementation
- external-repo execution proof
- autonomous code-review authority
- customer/publication readiness
- a v0.92 birthday blocker
- product repo migration completion

## Validation

Focused validation for this packet:

```sh
cargo test --manifest-path adl/Cargo.toml runtime_v2_codefriend_adapter_obligations
git diff --check
```

The SOR for `#4756` records the exact commands run and their results.
