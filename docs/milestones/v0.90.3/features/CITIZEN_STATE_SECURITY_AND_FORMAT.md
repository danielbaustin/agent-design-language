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
root; JSON remains a projection and review surface, not private-state authority.

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

If the ledger and materialized head disagree, recovery must reconstruct from the
ledger or enter quarantine.

## Continuity Witnesses And Receipts

Major identity transitions should emit continuity witnesses and citizen-facing
receipts.

Required first transitions:

- admission
- snapshot
- wake
- quarantine
- release from quarantine

The receipt should explain why the polis believes the current state is a valid
continuation without exposing unrelated private state.

## Validation Targets

- deterministic serialization and projection generation
- signature and content-hash validation
- missing, unknown, revoked, and mismatched key rejection
- sequence regression rejection
- no duplicate active head
- anti-equivocation detection
- ambiguous wake rejection
- projection redaction and leakage checks
