# Design - v0.90.3

## Design Intent

v0.90.3 defines the citizen-state substrate that later Runtime v2, moral,
identity, and birthday milestones rely on.

The key design move is separating authority from projection:

- authoritative state is compact, typed, signed, hash-linked, and optionally
  encrypted
- projections are generated views for citizens, operators, reviewers, public
  surfaces, and debug workflows
- append-only lineage is the continuity record
- witnesses and receipts explain major transitions
- ambiguous continuity enters sanctuary or quarantine

## Core Components

### Private State Format

WP-03 should make the canonical format decision. The expected direction is
protobuf-backed signed artifacts unless the issue records a better
source-grounded alternative.

JSON remains important, but only as deterministic projection, export, fixture,
or review surface.

### Signed Envelope And Trust Root

Every durable private-state checkpoint should carry envelope metadata:

- schema id
- artifact kind
- citizen id
- manifold id
- sequence
- prior hash
- content hash
- writer identity
- signature key id
- signature bytes
- optional encryption metadata

The first trust-root fixture should be local and simple enough to implement. It
does not need a cloud KMS.

### Sealed Quintessence Checkpoints

The protected checkpoint package should contain:

- sealed private state blob
- public or reviewer-safe redacted projection
- content hash
- prior checkpoint hash
- citizen id
- manifold id
- sequence number
- schema id
- key id
- writer signature
- continuity witness
- citizen-facing receipt

The term "quintessence checkpoint" is a useful internal name for the protected
continuity-bearing state core. Public docs may prefer "citizen continuity
checkpoint."

### Append-Only Lineage Ledger

The ledger is the authoritative history. A materialized state file is only the
current head.

If ledger and head disagree, continuity is unsafe. Recovery should reconstruct
from the ledger or preserve evidence in quarantine rather than trusting the
most convenient copy.

### Witnesses And Receipts

Major transitions emit evidence:

- admission
- snapshot
- wake
- key rotation
- quarantine
- release from quarantine
- challenge
- appeal or review resolution

A witness is the system-facing attestation of transition validity. A receipt is
the citizen-facing explanation of why the polis believes this state is a valid
continuation.

### Access And Projection

Access must be explicit, policy-mediated, and auditable. Operator convenience
must not bypass citizen privacy or continuity.

Projection classes:

- private authoritative state
- citizen-facing receipt
- operator projection
- reviewer projection
- public projection
- debug projection

Projections are never authority.

### Standing And Communication

The milestone consumes the canonical standing model:

- citizen
- guest
- service or system actor
- external actor
- naked actor, which is prohibited

Communication is a governed action. It never implies inspection.

### Sanctuary And Quarantine

Quarantine preserves evidence when state cannot be trusted. Sanctuary is the
conservative mode for ambiguous continuity or sensitive rights-preserving
pause.

Neither is punishment. Both are continuity-preserving safety behavior.

## Compression Shape

The milestone is intentionally contract-first:

- WP-02 audits inherited state
- WP-03 through WP-07 define format, envelope, ledger, witness, and key
  mechanics
- WP-08 through WP-12 prove negative and integrated behavior
- WP-13 through WP-18 fill policy, projection, challenge, threat, and economics
  placement
- WP-19 and WP-20 close the milestone with review and release truth
