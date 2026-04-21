# Citizen State Security And Format

## Status

Planning contract for v0.90.3.

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

WP-03 is the formal decision gate. The expected direction is protobuf-backed
signed artifacts unless a better source-grounded alternative is recorded.

The format decision must include:

- deterministic serialization rules
- schema evolution rules
- reserved field behavior
- content-hash rules
- projection generation rules
- compatibility policy for old checkpoints

## Signed Envelope

The envelope should include:

- schema id
- artifact kind
- citizen id
- manifold id
- sequence
- prior hash
- content hash
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
