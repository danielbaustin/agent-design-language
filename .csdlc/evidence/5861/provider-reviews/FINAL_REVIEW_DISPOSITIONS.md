# Final Provider Review Dispositions

Final substantive exact head: `853fc85b68869bb48653f9ea33ed925ed09af567`

Gemini reviewed predecessor substantive head
`f66424111f06d50f24c68f71f4efd3aeb8197ecb`. Subsequent source-grounded
reviews found and drove repairs for existing-worktree rollback, in-place lock
reuse, executable permissions, audit projection parity, strict current-record
shape, and active command guidance.

Provider reviews are advisory. Final merge readiness is determined by the
source-grounded exact-head reviews and focused executable proof.

## Gemini 3.1 Pro

Gemini `gemini-3.1-pro-preview` completed through the hosted API. It raised
seven proposed blockers. Source inspection produced these dispositions:

1. **Installer hang / inherited stdin: not applicable.** The hung Gate 10A test
   installs and launches `csdlc-edit`; it does not exercise issue creation,
   validation, doctor, or binding. Those commands do not execute validation
   lane commands during readiness diagnosis.
2. **Existing-worktree destructive rollback: contradicted.** Materialization
   compares existing authored files before any write. Cleanup removes the issue
   projection and authored files only when that invocation created them
   (`csdlc-v2/src/lifecycle.rs:190-238`). It does not reset, restore, or overwrite
   unrelated working-tree content.
3. **Remote-tip readiness: outside the command contract.** Creation and
   pre-bind readiness operate on the declared local base and canonical issue
   state. Publication and integration own remote-head truth; validation does
   not claim remote synchronization.
4. **Distributed issue creation: outside the command contract.**
   `csdlc-issue create` creates local canonical C-SDLC state for an already
   identified GitHub issue. It neither allocates issue IDs nor pushes branches,
   so remote duplicate allocation cannot occur in this command.
5. **Doctor lock inversion: contradicted.** Doctor is read-only and acquires
   neither the Git-topology lock nor the issue lock. Create and bind both acquire
   the topology lock before the issue lock.
6. **Current digest bypass: contradicted.** Doctor and bind call
   `verify_cards`, which calls `verify_record` and rejects current index digest,
   card projection, rendered Markdown, design, and diagram drift
   (`csdlc-v2/src/store.rs:1521-1615`). Idempotent topology scans now use the
   same verification path.
7. **Lingering operator claim authority: contradicted on active surfaces.** The
   active root contract, default workflow, operator skills, architecture
   diagram, and creation/binding runbook all identify Git branch/worktree
   topology as lifecycle authority. Preparation paths are no longer classified
   as review-safe metadata.

No Gemini finding required another product change.

## Claude

Claude reviewed the earlier substantive head
`967f0b59a9229a6cc796de4dce8a16b9bcad280d` through the hosted API. Its four
summary-derived findings were either outside the repository integrity model or
were subsequently addressed before the current substantive head:

1. **Legacy digest anchoring:** repository digests detect stored-record drift;
   external signing is not part of #5861. The current head additionally rejects
   unknown unsigned fields in exact legacy shapes before normalization.
2. **Bind ordering:** source performs verified idempotence/conflict detection
   before Git mutation and records invocation-local creation flags for cleanup.
3. **Validation wrappers:** the current head rejects shell/environment wrappers,
   informational invocations, unresolved executables, and invalid affected
   paths. This is a readiness contract, not an arbitrary-command theorem prover.
4. **Create cleanup:** creation is serialized before existence checks and
   removes only authored files that the invocation created when bootstrap fails.

The Claude design and review records remain retained as transparent advisory
evidence; they are not represented as source-grounded exact-current-head proof.

## Source-Grounded Exact-Head Review

The final source review of predecessor head `245e59d06` found one concrete P1:
authored design and diagram writes could follow a symlinked parent outside the
repository. Head `853fc85b6` applies the existing canonical-parent and
regular-file protections to creation and worktree materialization. Gate 2 now
includes a negative symlink fixture and proves that neither outside artifact is
written. No other P0, P1, or P2 findings were reported.

## Focused Proof

- `cargo check --manifest-path csdlc-v2/Cargo.toml --all-targets`: pass
- `cargo test --manifest-path csdlc-v2/Cargo.toml --test gate2 -- --nocapture`:
  pass
- legacy unsigned-topology injection regression: pass
- in-place bind regression through focused Gate 4 test: pass
- existing-worktree post-materialization failure cleanup: pass
- non-executable validation command rejection: pass
- current unknown-field and audit-projection tamper rejection: pass
- authored-path symlink escape rejection: pass
- `git diff --check origin/main...853fc85b6`: pass
- Gate 10A: 15 tests passed; the final installed-edit executable smoke test
  hung and was stopped, so the suite is not recorded as passing
