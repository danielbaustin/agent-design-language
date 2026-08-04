# Invariants

- `csdlc-finish` remains the sole terminal mutation authority and still validates the exact published head, required checks, review evidence, live GitHub identity, and claim lineage.
- `csdlc-clean` remains independent of delivery truth and continues to read historical terminal projections and receipts without mutating them.
- Existing `merge_ready`, `merged`, and `closed_out` records and terminal receipts continue to deserialize and resolve to the same compatibility outcomes.
- No supported binary, skill, schema writer, or library export can create or reconcile a tracked post-merge terminal projection or full terminal receipt.
- No second PR is required solely to record that an implementation PR merged.
- Historical tracked records and evidence are not rewritten or deleted.
- Current issue initialization, binding, validation, review, publication, finish, status, and cleanup behavior remains deterministic and fail-closed.
- All tracked implementation changes remain in the issue 5780 worktree.
