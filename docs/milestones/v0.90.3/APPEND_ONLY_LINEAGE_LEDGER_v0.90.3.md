# Append-Only Lineage Ledger - v0.90.3

## Status

Landed in WP-06 / #2332.

## Purpose

WP-06 adds the append-only lineage ledger above the WP-03 canonical
private-state format, the WP-04 signed envelope, and the WP-05 local sealed
checkpoint. It proves that the ledger, not the convenient materialized head
file, is the authority for citizen-state continuity.

## Runtime Evidence

The Runtime v2 implementation introduces private-state lineage authority in:

- `adl/src/runtime_v2/private_state_lineage.rs`
- `adl/src/runtime_v2/tests/private_state_lineage.rs`

The focused contract entrypoint is:

- `runtime_v2_private_state_lineage_contract`

The fixture evidence is:

- `adl/tests/fixtures/runtime_v2/private_state/lineage_ledger.json`
- `adl/tests/fixtures/runtime_v2/private_state/materialized_head.json`
- `adl/tests/fixtures/runtime_v2/private_state/lineage_negative_cases.json`

## Ledger Authority

The lineage ledger records:

- ledger schema and artifact path
- citizen id, manifold id, and lineage id
- authority rule for ledger-over-head validation
- append-only rule for truncation, replay, and fork rejection
- ordered lineage entries
- accepted head entry hash
- ledger root hash
- recovery policy

Each accepted entry binds:

- previous entry hash
- transition type
- state sequence
- predecessor state hash
- signed envelope reference and hash
- sealed checkpoint reference and hash
- canonical state hash
- writer identity
- optional witness and receipt references
- deterministic logical tick
- entry disposition

For WP-06, the prototype ledger contains the first accepted admission entry for
`proto-citizen-alpha`. WP-07 will add continuity witnesses and citizen-facing
receipts, and WP-08 will add broader anti-equivocation behavior.

## Accepted Head

The accepted head is calculated by replaying the ledger in order and requiring:

- every entry hash to recompute exactly
- every previous entry hash to match the prior accepted entry
- state sequences to be contiguous
- predecessor state hashes to match prior canonical state hashes
- no replayed entry hashes
- no replayed sequence positions
- the recorded accepted head hash to equal the final append-only entry
- the ledger root hash to recompute from the accepted entry hashes

The materialized head file is valid only when it matches the accepted ledger
head. It is a projection of the ledger head, not an independent authority.

## Negative Cases

Focused tests and fixtures cover:

- tampered entry hash or canonical state hash
- truncated ledger
- forked successor at the same sequence position
- replayed entry
- materialized head disagreement

Materialized head disagreement produces a `recovery_or_quarantine` disposition.
The runtime must reconstruct from the ledger or quarantine before any wake,
restore, migration, or activation path trusts that head.

## Validation

Focused validation:

```bash
cargo test --manifest-path adl/Cargo.toml runtime_v2_private_state_lineage -- --nocapture
cargo test --manifest-path adl/Cargo.toml runtime_v2_private_state -- --nocapture
```

## Non-Claims

- This does not emit continuity witnesses or citizen-facing receipts.
- This does not complete anti-equivocation across multiple signed successors.
- This does not implement sanctuary/quarantine UX or operator review flows.
- This does not implement access-control semantics.
- This does not claim first true Godel-agent birth.
- This does not allow unrestricted operator inspection of private citizen
  state.
