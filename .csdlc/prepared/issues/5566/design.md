# Design: activate an exact reserved claim from its existing issue worktree

## Context

`csdlc-bind` currently treats a call as issue-local only when the request uses
`worktree: "."`. A prepared claim, however, can truthfully retain the
repository-relative path of an already-created worktree. After typed repair of
the issue record on that branch, binding from the primary checkout would lose
the repaired state and binding from the issue worktree is rejected.

## Change

Recognize the invocation as issue-local when Git reports that the current
checkout is the requested branch and its canonical path is the same path named
by the exact reserved worktree claim. Preserve all existing exact-claim,
collision, path-safety, readiness, and branch checks. Reject any path or branch
mismatch.

## Validation

Add focused Gate 2 coverage for activation from an existing matching worktree
and for mismatched worktree rejection. Run focused tests and strict Clippy.
