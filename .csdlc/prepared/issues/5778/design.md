# #5778 derived terminal finish design

## Problem

C-SDLC v2 currently persists readiness, merge, closed-out, receipt, and
reconciliation state after the implementation PR has already merged. The
implementation PR cannot contain facts about its own future merge, so the
tracked projection requires another branch and PR. Receipts then duplicate the
record, six cards, and authored artifacts, creating two authorities that must be
reconciled before cleanup.

## Behavioral change

Terminal delivery becomes a derived observation rather than a tracked
post-merge transition. GitHub owns PR and issue terminal state; the immutable
merged Git graph owns integration truth. The final committed C-SDLC record owns
the pre-merge plan, execution, validation, publication identity, and exact
review evidence.

## Target contract

`csdlc-finish` accepts one typed request binding issue, repository, PR,
expected head, required checks, review policy, merge method, claim, and actor.
It acquires the existing issue lock, validates the canonical pre-merge record,
collects current GitHub state, and converges to one of:

- `ready_to_merge`: exact head, review, checks, and publication identity pass;
  merge with expected-SHA protection and observe again;
- `merged`: GitHub reports the governed PR merged and supplies the merge SHA;
- `closed_unmerged`: GitHub reports the governed PR closed without merge;
- `closed_no_pr`: a separately approved typed no-PR closure is present;
- `not_ready`: no mutation, with stable blocker codes.

The result is deterministic for the same canonical record and remote
observation. Repeating finish after merge returns the same terminal envelope.
An injected interruption after remote merge leaves no required tracked write;
the next invocation derives the merged result again.

## Exact review authority

The current reviewed record remains compatible. The new path additionally
supports an exact-SHA review attestation outside the reviewed Git tree so
recording review does not create a new unreviewed head. Any different PR head
invalidates the attestation.

## Claim behavior

A local claim remains useful while work is active. Once the governed GitHub
issue or PR is terminal, collision classification treats that claim as
logically released. A local audit/cache update may be attempted, but failure or
interruption cannot make an already-delivered issue block another issue.

## Compatibility

Legacy `merge_ready`, `merged`, `closed_out`, terminal evidence, and retained
receipts remain readable. This issue does not delete or rewrite them. The
resolver must prefer live terminal truth plus immutable Git identity and expose
whether its result came from the current derived contract or a legacy record.

## Failure and concurrency policy

- Validate exact record generation/digest and current head before merge.
- Use GitHub expected-SHA merge semantics.
- Serialize local cache/audit writes with the existing issue lock.
- Treat an already-merged PR as successful idempotent convergence.
- Never create a terminal projection, second closeout PR, or worktree deletion.
- Fail closed before merge on stale review, missing required checks, identity
  mismatch, or ambiguous no-PR authority.

## Validation boundary

Characterization and integration fixtures cover ready merge, already merged,
force-pushed head, failed checks, closed unmerged, approved no-PR, interruption
after merge, concurrent invocations, legacy records, and zero tracked
post-merge changes. Strict Clippy and the existing Gate 4-7 regression surface
remain required.
