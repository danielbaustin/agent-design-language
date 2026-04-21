# Signed Private-State Envelope - v0.90.3

## Status

Landed in WP-04 / #2330.

## Purpose

WP-04 adds the first local trust boundary around canonical private citizen
state. WP-03 made deterministic private-state bytes authoritative; WP-04 proves
that those bytes can be wrapped in a signed envelope and checked against a local
trust root before later WPs add sealing, ledger authority, witnesses, receipts,
and anti-equivocation.

## Runtime Evidence

The Runtime v2 implementation introduces signed private-state envelopes in:

- `adl/src/runtime_v2/private_state_envelope.rs`
- `adl/src/runtime_v2/tests/private_state_envelope.rs`

The focused contract entrypoint is:

- `runtime_v2_private_state_envelope_contract`

The fixture evidence is:

- `adl/tests/fixtures/runtime_v2/private_state/envelope.json`
- `adl/tests/fixtures/runtime_v2/private_state/trust_root.json`
- `adl/tests/fixtures/runtime_v2/private_state/envelope_negative_cases.json`

## Envelope Fields

The envelope binds:

- citizen id, manifold id, and lineage id
- state sequence
- predecessor state hash
- canonical private-state schema and artifact reference
- canonical content hash
- writer identity
- signature key id and algorithm
- signature bytes
- explicit WP-04 encryption non-claim metadata

The signing payload is deterministic and excludes the signature bytes
themselves.

## Trust Root

The local trust-root fixture defines:

- allowed signature algorithms
- active trusted writer keys
- revoked key ids
- allowed artifact kinds
- fail-closed validation policy
- non-claims for key rotation, encryption, and append-only ledger authority

The current prototype uses deterministic local Ed25519 keys for reviewable
fixture stability. It is not a production key-management design.

## Negative Cases

Focused tests reject:

- missing signatures
- unknown key ids
- revoked key ids
- content-hash mismatches
- sequence regression or mismatch
- broken predecessor linkage
- writer identity drift
- trust-root policy drift

## Validation

Focused validation:

```bash
cargo test --manifest-path adl/Cargo.toml runtime_v2_private_state_envelope -- --nocapture
cargo test --manifest-path adl/Cargo.toml runtime_v2_private_state_trust_root_matches_golden_fixture -- --nocapture
```

## Non-Claims

- This does not implement encrypted local sealing.
- This does not implement key rotation.
- This does not implement append-only ledger authority.
- This does not implement continuity witnesses, receipts, quarantine, or
  anti-equivocation.
- This does not allow unrestricted operator inspection of private citizen
  state.
