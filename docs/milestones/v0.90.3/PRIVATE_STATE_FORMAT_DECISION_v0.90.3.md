# Private State Format Decision - v0.90.3

## Status

Landed in WP-03 / #2329.

## Decision

ADL v0.90.3 treats private citizen state as a deterministic tagged-binary
artifact. JSON is a derived projection for review and operator visibility, not
authoritative state.

The selected v1 format is:

- `runtime_v2.private_citizen_state.v1`
- deterministic tagged binary
- field-numbered and protobuf-compatible in meaning
- hash-linked with `sha256` over the exact canonical byte stream
- projected through `runtime_v2.private_state_projection.v1`

WP-03 does not add signed envelopes, trust-root validation, encryption, local
sealing, append-only lineage replay, or anti-equivocation handling. Those are
owned by later v0.90.3 WPs.

## Authority Boundary

The canonical private-state bytes are authority for this WP. The JSON projection
is review evidence only. A projection may help an operator understand continuity
status, but it cannot authorize wake, restore, migration, inspection, or
identity transfer.

The projection must carry:

- citizen id
- manifold id
- lineage id
- state sequence
- canonical source schema
- canonical source artifact reference
- canonical source hash
- explicit `projection_not_authority` status
- redaction boundaries

## Runtime Evidence

The Runtime v2 implementation introduces the private-state contract in:

- `adl/src/runtime_v2/private_state.rs`
- `adl/src/runtime_v2/tests/private_state.rs`

The fixture evidence is:

- `adl/tests/fixtures/runtime_v2/private_state/format_decision.json`
- `adl/tests/fixtures/runtime_v2/private_state/proto-citizen-alpha.projection.json`

The private-state code also exposes `RuntimeV2PrivateCitizenState::canonical_bytes`
as the authoritative deterministic byte fixture. Tests assert that the bytes are
stable, begin with the private-state binary magic header, and are not JSON.

## Schema Evolution Rules

The v1 schema reserves field numbers for identity, lineage, sequence,
predecessor hash, projection linkage, ledger reference, private sections,
projection policy, and schema-evolution policy.

Unknown required fields fail closed until a migration witness accepts them.
Future protobuf/prost bindings must preserve the same field meanings and hash
input boundaries.

## Compatibility Notes

The v0.90.2 JSON citizen, wake, quarantine, and Observatory artifacts remain
inheritance evidence. They are not durable private-state authority.

This decision gives WP-04 through WP-09 a stable target for signed envelopes,
local sealing, append-only lineage, continuity witnesses, anti-equivocation, and
sanctuary/quarantine behavior.

## Validation

Focused validation:

```bash
cargo test --manifest-path adl/Cargo.toml runtime_v2_private_state -- --nocapture
```

The validation covers:

- deterministic canonical binary serialization
- source hash linking from projection back to canonical bytes
- rejection of missing identity fields
- rejection of missing lineage fields
- rejection of wrong canonical schema
- rejection of missing projection schema
- rejection of JSON projection authority claims

## Non-Claims

- This does not implement signed envelopes or trust-root validation.
- This does not implement encryption or sealed checkpoint storage.
- This does not implement append-only lineage replay.
- This does not implement duplicate active-head quarantine.
- This does not claim first true Gödel-agent birth.
