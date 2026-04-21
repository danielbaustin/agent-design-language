# Local Private-State Sealing - v0.90.3

## Status

Landed in WP-05 / #2331.

## Purpose

WP-05 adds the local-first sealing layer above the WP-03 canonical private-state
format and the WP-04 signed envelope. It proves that a canonical private citizen
state can be wrapped in a sealed checkpoint fixture, governed by a local key
policy, and opened only through a backend seam that preserves fail-closed key
behavior.

## Runtime Evidence

The Runtime v2 implementation introduces local private-state sealing in:

- `adl/src/runtime_v2/private_state_sealing.rs`
- `adl/src/runtime_v2/tests/private_state_sealing.rs`

The focused contract entrypoint is:

- `runtime_v2_private_state_sealing_contract`

The fixture evidence is:

- `adl/tests/fixtures/runtime_v2/private_state/key_policy.json`
- `adl/tests/fixtures/runtime_v2/private_state/sealing_backend_seam.json`
- `adl/tests/fixtures/runtime_v2/private_state/proto-citizen-alpha.sealed-checkpoint.json`
- `adl/tests/fixtures/runtime_v2/private_state/sealing_negative_cases.json`

## Key Policy

The local key policy records:

- active local sealing key id
- unavailable key id behavior
- allowed deterministic fixture sealing algorithm
- key-material digest rather than raw key material
- local storage policy
- validation policy for unavailable-key, wrong-key, and raw-payload failures

The policy is intentionally local-first. It does not require cloud confidential
computing and does not serialize usable key material into tracked artifacts.

## Backend Seam

The backend seam defines the adapter contract for the current deterministic
fixture backend and later backend families such as OS keychain, TPM, Secure
Enclave, HSM, or cloud confidential VM adapters.

Backend substitution must preserve:

- backend kind
- key id
- key-material digest checks
- associated-data binding
- content-hash verification
- fail-closed unavailable-key behavior

## Sealed Checkpoint Fixture

The sealed checkpoint binds:

- citizen id, manifold id, and lineage id
- state sequence and predecessor hash
- WP-04 envelope reference and envelope hash
- key policy reference
- backend seam reference
- sealing key id and algorithm
- nonce, associated-data hash, sealed payload, and sealed-payload hash
- plaintext content hash and schema
- projection reference

The sealed payload is deterministic fixture output. Tests prove it is not raw
JSON and not the raw canonical private-state byte stream.

## Negative Cases

Focused tests reject:

- unavailable sealing key
- wrong key material for the same key id
- raw JSON substituted as a sealed payload
- envelope-hash drift
- associated-data hash drift
- sealed-payload hash drift

## Validation

Focused validation:

```bash
cargo test --manifest-path adl/Cargo.toml private_state_sealing -- --nocapture
cargo test --manifest-path adl/Cargo.toml runtime_v2_private_state -- --nocapture
```

## Non-Claims

- This deterministic fixture is not production cryptography.
- This does not implement key rotation.
- This does not implement append-only ledger authority.
- This does not implement continuity witnesses, receipts, quarantine, or
  anti-equivocation.
- This does not require cloud enclaves or hardware isolation.
- This does not allow unrestricted operator inspection of private citizen
  state.
