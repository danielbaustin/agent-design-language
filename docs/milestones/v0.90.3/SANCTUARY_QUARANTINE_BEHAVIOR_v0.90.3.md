# Sanctuary And Quarantine Behavior - v0.90.3

## Status

Landed in WP-09 / #2335.

## Purpose

WP-09 turns the WP-08 `sanctuary_or_quarantine` disposition into executable
private-state safety-state behavior.

The proof covers the bounded case where a wake successor is ambiguous because
it competes with another signed successor for the same citizen lineage,
predecessor, and sequence. In that case, the runtime must not activate the wake
candidate, must not call quarantine a recovery success, and must preserve the
evidence required for operator review.

## Runtime Evidence

The Runtime v2 implementation introduces sanctuary/quarantine evidence in:

- `adl/src/runtime_v2/private_state_sanctuary.rs`
- `adl/src/runtime_v2/tests/private_state_sanctuary.rs`

The focused contract entrypoint is:

- `runtime_v2_private_state_sanctuary_contract`

The fixture evidence is:

- `adl/tests/fixtures/runtime_v2/private_state/sanctuary_quarantine_state_policy.json`
- `adl/tests/fixtures/runtime_v2/private_state/ambiguous_wake_fixture.json`
- `adl/tests/fixtures/runtime_v2/private_state/sanctuary_quarantine_artifact.json`
- `adl/tests/fixtures/runtime_v2/private_state/sanctuary_quarantine_operator_report.json`
- `adl/tests/fixtures/runtime_v2/private_state/sanctuary_quarantine_negative_cases.json`

## State Semantics

The state policy defines two conservative safety states:

- `sanctuary_pending_review`
- `quarantine_pending_review`

Both states block activation, recovery-success claims, destructive transition,
and evidence mutation.

The policy explicitly blocks:

- activating an ambiguous wake
- marking quarantine as recovery success
- mutating safety-state evidence before review
- pruning evidence before review
- release without continuity review

Release requires:

- operator review record
- continuity witness or review resolution
- single successor selected by policy
- evidence preservation verified

## Ambiguous Wake Fixture

The ambiguous wake fixture consumes the WP-08 anti-equivocation conflict and
disposition. It binds to:

- citizen id, manifold id, and lineage id
- contested successor sequence
- predecessor entry hash and predecessor state hash
- candidate ids and signed claim hashes
- source conflict and source disposition refs

It records `activation_allowed: false`, `recovery_success: false`, and the
expected safety state `sanctuary_or_quarantine_pending_review`.

## Quarantine Artifact

The quarantine artifact records the state-machine path:

- `wake_requested` to `activation_blocked`
- `activation_blocked` to `evidence_preserved`
- `evidence_preserved` to `sanctuary_or_quarantine_pending_review`

It preserves:

- lineage ledger
- continuity witnesses
- citizen receipts
- anti-equivocation conflict
- anti-equivocation disposition
- each candidate signed envelope
- each candidate sealed checkpoint

The artifact hash binds the produced safety-state artifact to its refs,
candidate evidence, blocked actions, and release requirements.

## Operator Report

The operator report is review evidence, not recovery success. It records:

- `operator_review_required`
- `safe_to_activate: false`
- `recovery_success: false`
- all preserved evidence refs
- findings for activation block, candidate evidence preservation, and
  quarantine-not-recovery-success
- a recommendation to keep the state held until continuity review selects one
  valid successor

## Negative Cases

Focused tests and fixtures cover:

- ambiguous wake activation
- treating quarantine as recovery success
- missing candidate evidence
- operator report that skips preserved evidence

## Validation

Focused validation:

```bash
cargo test --manifest-path adl/Cargo.toml runtime_v2_private_state_sanctuary -- --nocapture
cargo test --manifest-path adl/Cargo.toml runtime_v2_private_state -- --nocapture
```

## Non-Claims

- This does not implement continuity challenge or appeal.
- This does not implement access control or projection policy.
- This does not implement the redacted Observatory projection.
- This does not implement migration continuity or cross-polis continuity.
- This does not allow unrestricted operator inspection of private citizen
  state.
- This does not claim first true Godel-agent birth.
- This does not implement v0.92 identity rebinding.
