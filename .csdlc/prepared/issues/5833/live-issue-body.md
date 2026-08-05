## Summary
Execute **WP-15** in v0.92: birth witnesses and citizen-facing receipts.

## Dependencies
- WP-09 issue #5826
- WP-10 issue #5827
- WP-11 issue #5828
- WP-12 issue #5829
- WP-13 issue #5830

## Required Outcome
Produce the witness set, receipt contract, fixtures, validation report, and claim-boundary evidence over the complete identity, continuity, memory, capability, and cognitive-profile inputs.

## Acceptance Criteria
- Witnesses and receipts bind exact inputs, decisions, revisions, and evidence digests.
- Missing, stale, forged, contradictory, privacy-unsafe, and overclaiming packets fail closed.
- Exact tests and native receipts are source-SHA, argv, runner, output-digest, and artifact bound.
- The implementation PR includes `Closes #5833`.

## Non-goals
- No identity, continuity, memory, capability, profile, governance, or transport implementation.

<!-- csdlc-github-operation:v092-wp15-dependency-reconciled -->
