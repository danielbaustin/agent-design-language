# Issue 5624 Guarded Prune Topology Design

## Decision

Interpret terminal worktree identity with the same repository-rooted semantics used by typed binding. The prune guard resolves the recorded worktree to one canonical path and then requires an exact match among the current checkout, terminal branch, and live Git worktree topology.

## Resolution Rules

- `.` resolves only to the canonical current issue worktree.
- An absolute path resolves only to its canonical filesystem identity.
- A clean repository-relative path resolves from the primary repository root derived from Git's absolute common directory.
- Empty, missing, traversal-bearing, non-canonicalizable, wrong-branch, wrong-checkout, suffix-collision, and dirty candidates fail with `unsafe_checkout`.

## Safety Boundary

The repair changes interpretation only. It does not alter terminal evidence, terminal schemas, retained receipts, clean-worktree requirements, or prune execution. The guard must identify one exact current branch/path tuple in `git worktree list --porcelain` before reporting eligibility.

## Proof

Focused tests cover the issue-local sentinel, repository-relative and absolute identities, traversal and missing paths, wrong branches/checkouts, suffix collisions, dirt, duplicate branch topology, and byte-stable retained receipts. Validation runs from `/Volumes/FastWork` and includes formatting plus strict all-target Clippy.
