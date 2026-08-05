# Issue 5779 design

Status: ready for independent design approval.

## Decision

Add a standalone `csdlc-clean` binary backed by a small `cleanup` library
module. Cleanup is housekeeping only: it consumes an explicit typed request,
discovers current Git worktree registration, classifies the target, and may
remove only one clean, non-primary, exactly matched worktree. It neither reads
nor changes lifecycle delivery state before deciding whether delivery
succeeded.

Legacy terminal projections and retained receipts become read-only
compatibility inputs. The same module builds a deterministic compatibility
index and validates the tracked v0.91.8 terminal census. A missing optional
receipt is reported, not repaired; when a receipt is present it must match the
tracked projection. No compatibility operation rewrites cards, projections,
receipts, or derived terminal envelopes.

## Typed interfaces

`CleanupRequestV1` contains:

- schema, issue, expected branch, and expected worktree path;
- operation: `classify` or `remove`.

`CleanupResultV1` contains the exact observed registration, dirty paths,
mutation flag, and one status:

- `cleanup_ready` for a clean exact target during classification;
- `cleanup_removed` after verified removal;
- `cleanup_already_absent` for idempotent repeat removal;
- `cleanup_skipped_dirty` with every porcelain path;
- `cleanup_skipped_missing` when no registered target exists;
- `cleanup_skipped_drift` when issue, branch, worktree, or primary-checkout
  identity does not match.

`LegacyTerminalIndexV1` contains one sorted entry per requested issue with the
tracked projection identity, optional receipt identity, optional derived
terminal identity, and compatibility diagnostics. `TerminalCensusReportV1`
compares those entries with the declared v0.91.8 audit packet and fails closed
on projection, claim-free, disposition, PR, observed-head, or present-receipt
drift.

## Cleanup algorithm

1. Resolve the repository Git common directory and acquire an issue-scoped
   cleanup lock there.
2. Read `git worktree list --porcelain`; never infer topology from a terminal
   receipt or stored released-worktree string.
3. Require one exact registered branch/path tuple, reject the primary checkout,
   and verify the target's issue projection embeds the requested issue.
4. Read `git status --porcelain=v1 -z --untracked-files=all` in the target.
   Any entry yields `cleanup_skipped_dirty` with the exact safe relative paths.
5. For classify, return `cleanup_ready` without mutation. For remove, invoke
   non-forced `git worktree remove` from the primary/common repository and
   verify the tuple disappeared. A concurrent repeat converges to
   `cleanup_already_absent`.

No cleanup path runs `--force`, resets files, restores tracked bytes, deletes a
branch, prunes global worktree metadata, or follows symlinked/unregistered
paths.

## Compatibility and migration algorithm

1. Read the census packet and require its declared issue count to equal its
   unique sorted issue set.
2. For each issue, read the tracked projection and verify closed-out,
   claim-free terminal identity against the census entry.
3. If a retained receipt exists in the Git-common compatibility directory,
   parse and validate it and require its record/cards to equal the tracked
   projection. Absence remains a readable `missing_optional` state.
4. If a derived terminal envelope exists, validate and index it independently.
5. Emit the sorted index/report. Never retain, transport, regenerate, or repair
   legacy state.

## Safety invariants

- Cleanup outcome cannot alter or downgrade delivery/terminal truth.
- Dirty, ambiguous, primary, unregistered, symlinked, or non-UTF-8 targets are
  non-destructive skipped results or fail-closed input errors.
- The issue-scoped lock makes concurrent cleanup deterministic.
- Compatibility reads are byte-preserving and path-confined.
- Machine-readable output remains on stdout; diagnostics remain on stderr.
- No AWS, hosted service, external archive, or legacy lifecycle wrapper is
  introduced.

## Proof plan

- Focused tests for clean classification/removal, idempotent absence, dirty
  tracked and untracked paths, missing and relocated targets, primary-checkout
  refusal, symlink/path safety, and two concurrent removers.
- A test proving the same derived terminal envelope is returned before and
  after deleting a legacy receipt from an isolated clone fixture.
- A census fixture and the tracked v0.91.8 audit packet both validate without
  rewriting projections, cards, or receipts; injected identity drift fails.
- Full `csdlc-v2` tests, format, strict Clippy, and exact-head review.
