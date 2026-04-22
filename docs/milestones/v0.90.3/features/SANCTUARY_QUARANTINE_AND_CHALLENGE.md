# Sanctuary, Quarantine, And Challenge

## Status

Planning and implementation contract for v0.90.3. WP-08 has landed the first
anti-equivocation disposition into `sanctuary_or_quarantine`; WP-09 has landed
the bounded sanctuary/quarantine behavior proof for ambiguous wake. WP-13 has
landed the D11 challenge, freeze, appeal/review, threat-model, and economics
placement proof in `CONTINUITY_CHALLENGE_APPEAL_v0.90.3.md`.

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

WP-09 adds the runtime-backed D8 proof in
`SANCTUARY_QUARANTINE_BEHAVIOR_v0.90.3.md`. The proof defines sanctuary and
quarantine as evidence-preserving safety states, emits an ambiguous-wake
fixture, blocks activation, preserves candidate envelopes and sealed
checkpoints, and emits an operator report that explicitly does not mark
quarantine as recovery success.

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

WP-13 makes this concrete for the v0.90.3 proof slice:

- challenged wake freezes activation and cannot advance the active head
- challenged projection freezes publication and cannot widen into raw
  private-state disclosure
- appeal/review cannot release state without resolution evidence
- threat coverage includes insider/operator abuse, compromised key, malicious
  guest, equivocation, replay, projection leakage, and unsafe release from
  quarantine
- economics placement remains a resource-stewardship bridge only

## Validation Targets

- ambiguous wake enters sanctuary or quarantine
- challenged wake freezes destructive transition
- challenged projection authority blocks public projection
- quarantine preserves evidence references
- release from quarantine requires witness or review record
- equivocation disposition blocks activation and preserves candidate evidence
