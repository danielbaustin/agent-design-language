# Design: #5551 terminal plan-step truth repair

## Decision

Use #5551 as a separately claimed typed repair authority for closed-out, claim-free target #5527. Invoke the existing `repair-plan-step` operation three times with exact authority, target, and retained-receipt compare-and-swap values, advancing only S2, S3, and S4 from `pending` to `completed`.

## Invariants

- #5527 remains closed out and claim-free.
- S1 remains completed.
- No source or runtime files change.
- Every repair refreshes all card identities, index digest, audit, and retained receipt atomically.
- A doctor and receipt/local parity check follows the final repair.

## Failure handling

Any stale generation, digest, receipt, authority scope, unexpected step state, or doctor failure stops the operation without a hand edit.
