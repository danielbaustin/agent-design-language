# #5427 — Typed card identity version repair

## Problem

The v2 semantic edit API cannot repair a retained issue whose six card
identities carry the wrong release version. Manual edits are forbidden, so a
stale `v0.91.8` identity on #5353 cannot be corrected to `v0.91.7` through the
canonical store.

## Design

Add one typed semantic operation that validates a non-empty `v0.x.y` version,
updates the canonical issue identity and all six card identities atomically,
and leaves card content unchanged. The operation must be replay-safe through
the existing generation/digest compare-and-swap path.

## Proof

Focused Rust tests cover valid round-trip repair, malformed-version rejection,
content preservation, and rollback/atomicity. The supported operation is then
used to repair #5353 and its resulting cards are validated through the v2
validator and doctor.
