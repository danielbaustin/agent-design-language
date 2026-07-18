# #5470 Terminal projection and receipt crash consistency

## Problem

`reconcile_terminal` currently commits the canonical issue projection and then
refreshes the shared terminal receipt as separate filesystem operations. A
process interruption between those operations can leave different generations
or identities visible to readers. Receipt replacement also lacks explicit
file and parent-directory synchronization.

## Design

Use a small issue-scoped recovery journal containing the expected source
digest, target record digest, receipt bytes, and transaction phase. The commit
sequence writes and synchronizes the journal, writes temporary projection and
receipt files, synchronizes each file, atomically renames both, synchronizes
their parent directories, then removes and synchronizes the journal. Recovery
examines the journal and either completes the idempotent pair or rolls back
temporary artifacts before exposing success.

All transitions retain the existing issue lock, compare-and-swap identity,
receipt schema, rollback behavior, and idempotence. Fault injection is placed
before and after every write and rename boundary.

## Validation

- Focused Rust durability and lifecycle tests.
- Fault-injection tests for every journal, projection, receipt, rename, and
  directory-sync boundary.
- Existing terminal identity, rollback, idempotence, and typed reconciliation
  tests remain green.

## Non-goals

Runtime, AWS, and arbitrary post-closeout card changes remain outside scope.
