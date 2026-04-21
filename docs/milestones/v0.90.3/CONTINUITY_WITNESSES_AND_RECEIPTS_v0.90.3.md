# Continuity Witnesses And Receipts - v0.90.3

## Status

Landed in WP-07 / #2333.

## Purpose

WP-07 adds the first executable continuity-witness and citizen-facing receipt
contracts above the WP-03 private-state format, WP-04 signed envelope, WP-05
sealed checkpoint, and WP-06 append-only lineage ledger.

A witness is runtime evidence that a continuity-sensitive transition was checked
against the accepted ledger head, signed envelope, sealed checkpoint, and
materialized-head projection. A receipt is a citizen-facing explanation of that
continuity basis.

## Runtime Evidence

The Runtime v2 implementation introduces witness and receipt authority in:

- `adl/src/runtime_v2/private_state_witness.rs`
- `adl/src/runtime_v2/tests/private_state_witness.rs`

The focused contract entrypoint is:

- `runtime_v2_private_state_witness_contract`

The fixture evidence is:

- `adl/tests/fixtures/runtime_v2/private_state/continuity_witnesses.json`
- `adl/tests/fixtures/runtime_v2/private_state/citizen_receipts.json`
- `adl/tests/fixtures/runtime_v2/private_state/witness_receipt_negative_cases.json`

## Witness Schema

The continuity witness set records:

- witness set schema, id, and artifact path
- ledger reference and ledger root hash
- materialized-head reference
- witness authority
- one witness per required transition
- explicit claim boundary

Each witness binds:

- transition type
- citizen id, manifold id, and lineage id
- state sequence
- predecessor entry hash
- ledger reference and ledger root hash
- lineage entry id and hash
- accepted head entry hash
- materialized-head reference
- signed envelope reference and hash
- sealed checkpoint reference and hash
- canonical state hash
- evidence binding rule
- checked invariants
- witness authority
- deterministic logical tick
- witness hash

The initial required transition examples are:

- admission
- snapshot
- wake
- quarantine
- release from quarantine

For this WP, the examples are bound to the current accepted ledger head for
`proto-citizen-alpha`. Later anti-equivocation and challenge/appeal WPs can add
multi-entry successor behavior without changing the witness evidence contract.

## Receipt Schema

The citizen-facing receipt set records:

- receipt set schema, id, and artifact path
- witness set reference
- ledger reference
- one receipt per required transition
- explicit claim boundary

Each receipt gives the citizen:

- transition type
- citizen id and lineage id
- state sequence
- witness id and witness hash
- ledger reference
- accepted head entry hash
- continuity explanation
- citizen-visible evidence hashes and refs
- privacy boundary statements
- withheld private-material categories
- receipt hash

Receipts must explain why the polis treats the state as the valid continuation
of the same governed participant. They must not expose unrelated private state,
raw private-state bytes, sealed payload material, private keys, or other
citizens' state.

## Negative Cases

Focused tests and fixtures cover:

- tampered witness hash
- mismatched ledger root
- sealed payload leakage into a receipt
- missing continuity explanation
- missing required transition coverage

## Validation

Focused validation:

```bash
cargo test --manifest-path adl/Cargo.toml runtime_v2_private_state_witness -- --nocapture
cargo test --manifest-path adl/Cargo.toml runtime_v2_private_state -- --nocapture
```

## Non-Claims

- This does not implement anti-equivocation across multiple signed successors.
- This does not implement sanctuary/quarantine UX or operator review flows.
- This does not implement full continuity challenge or appeal.
- This does not make JSON the authority for private citizen state.
- This does not allow unrestricted operator inspection of private citizen
  state.
- This does not claim first true Godel-agent birth.
