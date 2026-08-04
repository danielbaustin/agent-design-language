# Issue #5648: operator-authorized active-claim revoke

## Boundary

Add one explicit typed lifecycle operation for an operator to revoke an
abandoned active claim before lease expiry. This is control-plane recovery
only: it does not modify product code, cards, branches, worktrees, or remote
issue state. Normal recovery remains expiry-based; normal closeout remains the
preferred release path.

## Contract

`csdlc-bind --revoke-request` accepts a versioned request containing issue,
repository, expected generation/digest, expected claim id, observed current
time, actor, reason, and an explicit operator-authority marker. The store takes
the issue lock, checks all compare-and-swap fields, requires a non-empty reason
and authority marker, and requires the observed time to be before lease expiry.
It clears only the matching claim, appends an audit event identifying the prior
owner and authority, recomputes the record digest, and atomically replaces the
record. A stale request, expired claim, or missing claim fails closed without
mutation; expired claims remain on the existing expiry-recovery route.

The operation releases the claim and protected-path ownership; it does not
advance lifecycle phase or infer terminal issue state. A later bind may claim
the issue through the ordinary typed route.

## Proof and limits

Focused tests cover exact success, stale generation/digest, claim-id mismatch,
missing authority, expiry rejection, and audit/release truth. The request and result are
JSON-schema-visible. No shell/Python lifecycle, raw GitHub CLI, AWS, network,
or direct Markdown/state edit is introduced.
