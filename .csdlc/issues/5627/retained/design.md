# Issue 5627 Four-Command Lifecycle Design

## Decision

Collapse the routine post-implementation C-SDLC v2 path into four typed commands without weakening claims, exact scoped review, publication identity, or terminal receipts:

1. `csdlc-validate finalize` executes the declared validation and atomically records execution, passing validation, and `Implemented`.
2. `csdlc-review record` validates exact scoped revision evidence and atomically records the review and advances to `Reviewed`; no prior assignment is required on the new path.
3. `csdlc-publish publish` creates a ready pull request directly and records its exact identity once.
4. Existing `csdlc-closeout closeout` observes the merge, releases the claim, and retains the terminal receipt unchanged.

## Deletion-First Boundary

Routine callers no longer need separate execution, validation, phase-advance, review-assignment, reviewed-advance, draft-publication, or ready-transition operations. Compatibility remains only where an active publication record is already draft and must still be reconciled safely.

One-shot command requests are overwritten under shared untracked Git state. Durable tracked state is limited to canonical cards, audit/index truth, one publication intent, and the terminal receipt.

## Atomicity And Failure

Validation, review, and publication compute all preconditions before committing state. Failed validation, stale scoped revision, claim mismatch, malformed evidence, or wrong pull-request identity writes nothing. Existing store locking and generation checks remain authoritative.

## Measurement

The #5624 routine path is the baseline: nine typed lifecycle commands including closeout and eleven post-product request/evidence artifacts. Acceptance is four commands including closeout and at most two durable post-product artifacts. The #5590 trace remains the remediation-rich comparison and is not used to weaken recovery behavior.

## Scope

Only `csdlc-v2` lifecycle code, focused Gate 4-7 tests, and the three affected operator contracts are in scope. Runtime, AWS, unrelated workflow redesign, and new dependencies are excluded.
