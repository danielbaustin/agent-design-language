# Provider design review disposition: issue 5861

## Review identity

- Gemini: direct-hosted `gemini-3.1-pro-preview`, result `ok`.
- Claude: direct-hosted `claude-opus-4-1-20250805`, result `ok`.
- Both reviews are advisory design evidence, not lifecycle approval.

## Accepted

- Define the repository-wide path-reservation linearization point before Git
  mutation.
- Pin readiness to a semantic payload digest and dependency version vector;
  exclude non-semantic tracker metadata.
- Make substantive edits invalidate the current receipt and return to
  `prepared` without requiring network sync.
- Inventory intent-created Git artifacts and make compensation ownership-safe.
- Serialize release against bind and clear abandoned write-ahead intent.
- Add preparation-time batch cycle/overlap analysis and truthful partial batch
  status.
- Add restartable, audited migration with quarantine and one explicit repair
  operation for ambiguous records.
- Add crash-window, race, stale-receipt, migration, and compensation tests.

## Accepted With Modification

- Gemini proposed failing every overlapping child seal. The design instead
  blocks the batch readiness claim and marks affected children while preserving
  independent point-in-time receipts; live overlap remains a bind predicate.
- Claude proposed an external unforgeable session token. The design reuses the
  governed session ledger and exact intent identity so the operator never has
  to coordinate a hidden token.
- Claude proposed all-or-nothing batch preparation. The design preserves
  successful child receipts and records per-child outcomes so partial success
  cannot be overstated or needlessly rolled back.
- Claude proposed globally unique CAS generation IDs across issues. Generation
  CAS is issue-local because current manifests are issue-local; content digests
  prevent ABA while unrelated issues remain parallel.

## Rejected

- `csdlc-bind release --legacy-force` is not used for ambiguous migration.
  Legacy repair belongs to a distinct audited migration command and cannot be
  confused with ordinary owner release.
- Branch names do not include transient intent IDs. Exact intent ownership and
  the artifact ledger distinguish created artifacts while preserving stable,
  predictable issue branch names.

## Result

All actionable blockers are incorporated into the prepared design. No product
implementation, canonical initialization, claim reservation, execution bind,
publication, or closeout was performed by the provider review.
