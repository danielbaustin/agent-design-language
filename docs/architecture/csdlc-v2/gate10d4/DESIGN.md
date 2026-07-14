# Gate 10D4 read-only importer sunset design

## Boundary

Remove only the read-only importer after `2026-08-12T02:03:02.808013Z`. The
decision requires trusted time, explicit approval, current v2 health, completed
migration evidence, and proof that no active contract still names the importer.

The timestamp is the historical Gate 10C default. The 2026-07-14 accelerated
operator decision may waive it through an exact
`csdlc.deletion_approval.v2` record after 100% parity, independent validation,
active-contract proof, and review pass; prose or an unbound boolean cannot
waive it.

## Invariants

- Early, missing, stale, or ambiguous inputs yield zero mutation.
- Migration evidence remains durable after importer removal.
- Exact-revision review and green checks precede merge.

## Non-goals

No unrelated v1, ADL, or Runtime cleanup.
