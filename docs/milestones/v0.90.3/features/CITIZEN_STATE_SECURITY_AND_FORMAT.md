# Citizen State Security And Format

## Status

Planning contract for v0.90.3. WP-03 has landed the canonical private-state
format decision and fixture-backed projection boundary. WP-04 has landed the
signed envelope, local trust-root fixture, and fail-closed negative cases for
the canonical state artifact.

## Purpose

Define the authoritative citizen-state substrate that replaces provisional JSON
as the long-term source of continuity.

## Core Rules

- Private citizen state is a moral and security boundary.
- JSON is a projection, not authoritative state.
- Durable state should be typed, signed, hash-linked, and optionally encrypted.
- The append-only ledger is the continuity history.
- Materialized head state is valid only when it matches lineage.
- Key rotation must not sever prior verifiability.
- Debug output and replay artifacts are evidence, not active state.

## Expected Format Direction

WP-03 selected deterministic tagged binary with protobuf-compatible field-number
semantics as the v1 canonical private-state format. WP-04 wraps that canonical
state in a deterministic Ed25519-signed envelope checked against a local trust
root. WP-05 adds local-first sealed checkpoint fixtures. WP-06 adds the
append-only lineage ledger and materialized-head validation rule. WP-07 adds
continuity-witness and citizen-facing receipt fixtures for major transitions.
JSON remains a projection and review surface, not private-state authority.

The format decision must include:

- deterministic serialization rules
- schema evolution rules
- reserved field behavior
- content-hash rules
- projection generation rules
- compatibility policy for old checkpoints

The landed decision is recorded in
`docs/milestones/v0.90.3/PRIVATE_STATE_FORMAT_DECISION_v0.90.3.md`, with runtime
evidence in `adl/src/runtime_v2/private_state.rs` and fixture evidence under
`adl/tests/fixtures/runtime_v2/private_state/`.

The landed signed-envelope proof is recorded in
`docs/milestones/v0.90.3/SIGNED_PRIVATE_STATE_ENVELOPE_v0.90.3.md`, with runtime
evidence in `adl/src/runtime_v2/private_state_envelope.rs`.

The landed local sealed checkpoint proof is recorded in
`docs/milestones/v0.90.3/LOCAL_PRIVATE_STATE_SEALING_v0.90.3.md`, with runtime
evidence in `adl/src/runtime_v2/private_state_sealing.rs`.

The landed append-only lineage proof is recorded in
`docs/milestones/v0.90.3/APPEND_ONLY_LINEAGE_LEDGER_v0.90.3.md`, with runtime
evidence in `adl/src/runtime_v2/private_state_lineage.rs`.

The landed continuity witness and receipt proof is recorded in
`docs/milestones/v0.90.3/CONTINUITY_WITNESSES_AND_RECEIPTS_v0.90.3.md`, with
runtime evidence in `adl/src/runtime_v2/private_state_witness.rs`.

## Signed Envelope

The WP-04 envelope includes:

- schema id
- artifact kind
- citizen id
- manifold id
- lineage id
- sequence
- prior hash
- content hash
- canonical state schema and reference
- writer identity
- signature key id
- signature algorithm
- signature bytes
- optional encryption metadata

Validation must reject missing signatures, unknown keys, revoked keys, hash
mismatches, sequence regression, and invalid predecessor linkage.

## Lineage Ledger

The ledger should be append-only and should record:

- prior entry hash
- transition type
- actor or writer identity
- content hash
- witness reference
- receipt reference where applicable
- signature

WP-06 lands the first ledger contract. The ledger is authoritative over the
materialized head. Accepted-head calculation replays the ordered entries,
recomputes entry and ledger hashes, rejects truncation/replay/fork/tamper
fixtures, and treats the materialized head as valid only when it matches the
accepted ledger head.

If the ledger and materialized head disagree, recovery must reconstruct from the
ledger or enter quarantine. The runtime must not trust whichever copy is most
convenient.

## Continuity Witnesses And Receipts

Major identity transitions emit continuity witnesses and citizen-facing
receipts.

Required first transitions:

- admission
- snapshot
- wake
- quarantine
- release from quarantine

The WP-07 witness set covers those transition examples and binds each witness to
ledger, materialized-head, envelope, sealed-checkpoint, and canonical-state
hash evidence. The receipt explains why the polis believes the current state is
a valid continuation without exposing unrelated private state, raw private-state
bytes, sealed payload material, private keys, or other citizens' state.

## Validation Targets

- deterministic serialization and projection generation
- signature and content-hash validation
- missing, unknown, revoked, and mismatched key rejection
- sequence regression rejection
- no duplicate active head
- anti-equivocation detection
- ambiguous wake rejection
- projection redaction and leakage checks
