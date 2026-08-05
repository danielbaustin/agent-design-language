# Issue 5834 Design: Reviewer-Facing Birthday Evidence Packet

## Outcome And Sources

Assemble WP-16's integrated review packet from `docs/milestones/v0.92/features/FIRST_BIRTHDAY_DEMO_AND_GOVERNANCE_HANDOFF_v0.92.md`, the requirement map and negative suite in `FIRST_BIRTHDAY_LAUNCH_PACKET_v0.92.md`, and the Birthday sprint packet. This issue packages proof; it does not manufacture missing child evidence.

## Owned Paths

- `docs/milestones/v0.92/review/FIRST_BIRTHDAY_REVIEW_PACKET_v0.92.md`
- `docs/milestones/v0.92/review/first-birthday-review-evidence.v1.json`
- `docs/milestones/v0.92/review/first-birthday-review-packet.schema.json`
- `docs/milestones/v0.92/DEMO_MATRIX_v0.92.md`
- `.csdlc/prepared/issues/5834/validate-review-packet.rb`
- `.csdlc/evidence/5834`

## Read-Only Inputs

- Every repository path cited outside `## Owned Paths` is read-only unless it is repeated exactly in that section.
- Dependency records, sibling issue outputs, historical evidence, and external systems remain read-only inputs.

## Serialization Gates

```json
[
  {
    "schema": "csdlc.serialization_gate.v1",
    "id": "v092-birthday-demo-matrix-v1",
    "paths": [
      "docs/milestones/v0.92/DEMO_MATRIX_v0.92.md"
    ],
    "issues": [
      5834,
      5836,
      5840
    ],
    "order": [
      5834,
      5836,
      5840
    ]
  }
]
```

## Contract

The packet inventories exact revisions and digests for WP-08 birth contract, WP-09 identity, WP-10 continuity, WP-11 memory, WP-12 capability, WP-13 profile, WP-14 communication evidence, WP-15 witnesses/receipt, negative-suite report, caveats, reviewer questions, and public non-claims. Missing, stale, contradictory, nonterminal, or unreviewed dependencies produce a named blocked disposition, never an inferred success.

## Dependencies And Invariants

WP-08 through WP-15, including WP-13A/#5831, WP-14/#5832, and WP-15/#5833, must satisfy sprint gate 4 and exact-head evidence checks. The canonical wave row and live issue must add WP-13A before execution so they exactly match this stricter card contract. One source digest appears once; private evidence is represented by approved redacted projections; presentation surfaces cannot replace canonical proof.

## Validation And Rollback

`validate-review-packet.rb` parses the schema and packet, recomputes every referenced digest, requires the exact WP-08 through WP-15 roster including WP-13A and WP-14, resolves every repo-relative link, and rejects missing or nonterminal proof. Its negative lane mutates one fixture at a time to prove stale revision, contradictory status, private-path leakage, personhood/citizenship/consciousness claims, and unauthorized publication-ready language fail closed. Rollback removes only the assembled packet/schema/validator and restores the WP-16-owned demo-matrix link.

## Non-Goals

Running the flagship demo, implementing child work, public publication, release approval, external review, v0.93 governance, and rewriting historical proof are excluded.
