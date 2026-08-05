# Issue #5727 Design: Safe Claim Reacquisition

## Problem

A nonterminal C-SDLC v2 record may intentionally have no active claim after a
lease expires or is deliberately released. Today record validation classifies
that dormant state as corruption, while `csdlc-bind` has no compare-and-swap
operation for restoring a writer lease without rebuilding or rewinding the
record.

## Design

Add a typed claim-reacquisition request and result to the bind lifecycle
surface. The request identifies the issue and expected record generation and
digest, supplies the complete replacement claim, and records the actor and
reason. Reacquisition:

1. locks and loads the existing record;
2. verifies its expected generation and digest;
3. requires a resumable nonterminal phase and an absent or expired claim;
4. validates branch, worktree, lease timestamps, purpose, and protected paths;
5. checks every other live claim for protected-path overlap;
6. installs the replacement claim without changing lifecycle phase;
7. appends an audit event and commits the next generation atomically.

Read-only doctor inspection treats an absent claim on a resumable nonterminal
record as dormant and reports reacquisition as the next action. Mutating
operations continue to require a valid live claim.

## Proof

Focused Rust tests cover deliberate release, expiry, overlap collision, stale
generation, stale digest, invalid branch/worktree, audit preservation, and the
real prepared #5354 reproduction. Existing lifecycle tests must continue to
prove collision safety and write authorization.

## Non-goals

No new lifecycle phase, compatibility wrapper, receipt dependency, direct
record editing, or WP-15 product execution.
