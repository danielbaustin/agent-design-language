# Anti-Equivocation Conflict - v0.90.3

## Status

Landed in WP-08 / #2334.

## Purpose

WP-08 adds the first executable anti-equivocation proof above the WP-03
canonical private-state format, WP-04 signed envelope, WP-05 sealed checkpoint,
WP-06 append-only lineage ledger, and WP-07 continuity witness and receipt
contracts.

The proof covers the narrow but important case where two signed successor
claims target the same citizen lineage, predecessor, and state sequence. In
that case, the runtime must not allow both successors to become active, and it
must preserve evidence for sanctuary or quarantine review.

## Runtime Evidence

The Runtime v2 implementation introduces anti-equivocation evidence in:

- `adl/src/runtime_v2/private_state_equivocation.rs`
- `adl/src/runtime_v2/tests/private_state_equivocation.rs`

The focused contract entrypoint is:

- `runtime_v2_private_state_anti_equivocation_contract`

The fixture evidence is:

- `adl/tests/fixtures/runtime_v2/private_state/anti_equivocation_conflict.json`
- `adl/tests/fixtures/runtime_v2/private_state/anti_equivocation_disposition.json`
- `adl/tests/fixtures/runtime_v2/private_state/anti_equivocation_negative_cases.json`

## Conflict Fixture

The conflict fixture records:

- conflict schema, id, kind, and artifact path
- citizen id, manifold id, and lineage id
- ledger reference and accepted head binding
- current head sequence and contested successor sequence
- predecessor entry hash and predecessor state hash
- continuity witness and citizen receipt references
- attempted active candidate ids
- two signed successor candidates for the same contested position
- activation rule forbidding dual activation
- evidence-preservation rule
- deterministic conflict hash

Each candidate binds:

- candidate id and lineage entry id
- entry hash and signed claim hash
- transition type
- citizen id, manifold id, lineage id, and state sequence
- prior entry hash and predecessor state hash
- signed envelope reference and hash
- sealed checkpoint reference and hash
- canonical state hash
- continuity witness reference and hash
- citizen receipt reference and hash
- signing key id, algorithm, and writer identity

For WP-08, the prototype creates two conflicting successors for
`proto-citizen-alpha` at the next sequence after the accepted ledger head. One
candidate represents a snapshot successor and the other represents a wake
successor. Both bind to the same citizen, lineage, predecessor, and contested
sequence, but they carry distinct signed successor claims.

## Disposition

The disposition fixture records that the equivocation conflict enters
`sanctuary_or_quarantine` instead of activating either candidate.

The disposition must:

- set `activation_allowed` to false
- leave `active_candidate_id` empty
- preserve ledger, witness, receipt, and conflict artifact references
- preserve all candidate ids, entry hashes, and claim hashes
- set the destructive-transition policy to
  `block_activation_and_preserve_evidence_until_review`
- route the conflict to `sanctuary_or_quarantine_operator_review`

This is intentionally a disposition proof, not the full sanctuary or quarantine
behavior implementation. WP-09 lands the broader ambiguous-wake and
conservative safety-state behavior in
`SANCTUARY_QUARANTINE_BEHAVIOR_v0.90.3.md`.

## Negative Cases

Focused tests and fixtures cover:

- attempting to activate two signed successor candidates for the same sequence
- dropping or corrupting preserved candidate evidence
- changing a candidate predecessor away from the accepted ledger head
- submitting duplicate claim hashes instead of conflicting successor claims

## Validation

Focused validation:

```bash
cargo test --manifest-path adl/Cargo.toml runtime_v2_private_state_anti_equivocation -- --nocapture
cargo test --manifest-path adl/Cargo.toml runtime_v2_private_state -- --nocapture
```

## Non-Claims

- This does not implement full sanctuary/quarantine UX or operator review.
- This does not implement continuity challenge or appeal.
- This does not implement migration anti-equivocation.
- This does not make JSON the authority for private citizen state.
- This does not allow unrestricted operator inspection of private citizen
  state.
- This does not claim first true Godel-agent birth.
