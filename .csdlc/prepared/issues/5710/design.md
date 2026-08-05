# C-SDLC v2 terminal closeout and safe-prune recovery

## Problem

Closed issues can be blocked from truthful terminal closeout or safe pruning by
two distinct forms of drift:

1. publication evidence names an earlier reviewed/published revision while the
   merged pull request reached a later causally related head;
2. the terminal worktree contains generated lifecycle material, stale locks, or
   retained evidence that makes the worktree dirty even though source work is
   complete.

The current closeout path correctly fails closed, but it provides no typed
recovery operation for either condition.

## Design

Add two typed closeout recovery capabilities.

### Terminal publication reconciliation

The operation accepts the issue identity, active closeout authority, recorded
publication identity, live merged PR identity, and final head/merge revisions.
It succeeds only when repository, PR, branch, and commit ancestry prove that
the final merged head is an identity-preserving descendant of the recorded
publication head. It records the discrepancy and the final terminal evidence
in the audit trail. Ambiguous PR identity, unrelated heads, or missing ancestry
fail closed.

### Safe prune preparation

The operation inspects `git status --porcelain --untracked-files=all` and
classifies each dirty path into:

- safe generated state;
- evidence requiring retained-receipt equivalence;
- tracked lifecycle drift requiring reconciliation;
- source or unknown changes requiring operator review.

Safe generated state is narrowly limited to stale issue-local lock files and
reproducible prepared requests. Evidence is removable only after byte-equivalent
retention under Git-common terminal authority is proved. Tracked lifecycle
state and source/unknown files are never deleted by this operation.

Cleanup is followed by the existing `validate-prune` guard. The existing
receipt-backed `prune` operation remains the only worktree-removal authority.

### Closed issue repair classification

A read-only classifier maps a closed GitHub issue's local lifecycle phase to
the next legal typed action. It never advances lifecycle state by inference.

## Safety invariants

- No force prune.
- No manual card or `.csdlc` record mutation.
- No source file deletion.
- No deletion of evidence without retained-receipt equivalence.
- Every terminal reconciliation is repository-, issue-, PR-, branch-, and
  ancestry-bound.
- Repeating a successful cleanup or reconciliation is idempotent.

## Proof plan

- Focused unit/integration tests for accepted and rejected terminal drift.
- Focused tests for every dirty-path classification and cleanup boundary.
- Repeatability tests.
- Exact-head review before publication.
- A live v0.91.8 sweep after merge, reporting pruned and blocked issues
  separately.
