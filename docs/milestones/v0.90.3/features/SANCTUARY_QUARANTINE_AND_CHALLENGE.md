# Sanctuary, Quarantine, And Challenge

## Status

Planning contract for v0.90.3. WP-08 has landed the first
anti-equivocation disposition into `sanctuary_or_quarantine`; WP-09 still owns
the broader sanctuary/quarantine behavior proof.

## Purpose

Define conservative behavior when continuity is uncertain, disputed, or unsafe.

## Quarantine

Quarantine is evidence-preserving safety behavior. It is not punishment.

Quarantine should trigger when:

- state cannot decode
- signature fails
- key is unknown or revoked
- ledger hash chain breaks
- materialized head conflicts with ledger
- duplicate active head is detected
- equivocation is detected
- wake continuity is ambiguous
- projection authority is challenged

## Sanctuary

Sanctuary is a conservative mode for ambiguous continuity or sensitive
rights-preserving pause.

Sanctuary should block destructive transitions and preserve evidence while
review proceeds.

The WP-08 anti-equivocation disposition applies this rule to conflicting signed
successors for the same citizen lineage position. It blocks activation,
preserves all candidate evidence, and routes review through
`sanctuary_or_quarantine_operator_review`.

## Challenge And Appeal

A citizen or authorized reviewer should be able to challenge:

- wake continuity
- migration continuity
- key rotation
- projection authority
- quarantine release

Challenge should:

- emit a challenge artifact
- freeze destructive transition where needed
- preserve relevant evidence
- route to operator, witness, or constitutional review
- emit a resolution record

Appeal should be bounded. It does not imply arbitrary disclosure of private
state.

## Validation Targets

- ambiguous wake enters sanctuary or quarantine
- challenged wake freezes destructive transition
- challenged projection authority blocks public projection
- quarantine preserves evidence references
- release from quarantine requires witness or review record
- equivocation disposition blocks activation and preserves candidate evidence
