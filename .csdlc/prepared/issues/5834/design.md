# Issue 5834 Design: Reviewer-Facing Birthday Evidence Packet

## Outcome And Sources

Assemble WP-16's integrated review packet from `docs/milestones/v0.92/features/FIRST_BIRTHDAY_DEMO_AND_GOVERNANCE_HANDOFF_v0.92.md`, the requirement map and negative suite in `FIRST_BIRTHDAY_LAUNCH_PACKET_v0.92.md`, and the Birthday sprint packet. This issue packages proof; it does not manufacture missing child evidence.

## Owned Surface

Protected implementation paths are `docs/milestones/v0.92/review/FIRST_BIRTHDAY_REVIEW_PACKET_v0.92.md`, `docs/milestones/v0.92/review/first-birthday-review-evidence.v1.json`, `docs/milestones/v0.92/review/first-birthday-review-packet.schema.json`, `.csdlc/prepared/issues/5834/validate-review-packet.rb`, `docs/milestones/v0.92/DEMO_MATRIX_v0.92.md` only for the WP-16 link/status row, and `.csdlc/evidence/5834/`. The feature contract is a read-only source unless a fresh claim explicitly adds it. Shared release, launch, and milestone status docs require separate owners.

## Contract

The packet inventories exact revisions and digests for WP-08 birth contract, WP-09 identity, WP-10 continuity, WP-11 memory, WP-12 capability, WP-13 profile, WP-14 communication evidence, WP-15 witnesses/receipt, negative-suite report, caveats, reviewer questions, and public non-claims. Missing, stale, contradictory, nonterminal, or unreviewed dependencies produce a named blocked disposition, never an inferred success.

## Dependencies And Invariants

WP-08 through WP-15, including WP-13A/#5831, WP-14/#5832, and WP-15/#5833, must satisfy sprint gate 4 and exact-head evidence checks. The canonical issue row currently omits the stricter WP-13A gate carried by the sprint gate; pre-execution dependency reconciliation must align the live issue, canonical wave, and cards before claim acquisition, preserving WP-13A as required. One source digest appears once; private evidence is represented by approved redacted projections; presentation surfaces cannot replace canonical proof.

## Validation And Rollback

`validate-review-packet.rb` parses the schema and packet, recomputes every referenced digest, requires the exact WP-08 through WP-15 roster including WP-13A and WP-14, resolves every repo-relative link, and rejects missing or nonterminal proof. Its negative lane mutates one fixture at a time to prove stale revision, contradictory status, private-path leakage, personhood/citizenship/consciousness claims, and unauthorized publication-ready language fail closed. Rollback removes only the assembled packet/schema/validator and restores the WP-16-owned demo-matrix link.

## Non-Goals

Running the flagship demo, implementing child work, public publication, release approval, external review, v0.93 governance, and rewriting historical proof are excluded.
