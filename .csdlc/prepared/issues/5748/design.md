# Issue #5748 design: remaining terminal truth recovery

## Outcome

Recover truthful typed C-SDLC v2 terminal state for the closed v0.91.8 issues
excluded from the clean #5746 projection wave. Each issue must end either with
remote disposition, retained receipt, tracked projection, claim release, and
retained artifacts in agreement, or with an exact fail-closed exception that
preserves the blocking evidence and does not invent terminal authority.

## Authority model

- Issue-local worktrees create or repair their own terminal receipts.
- The dedicated #5748 authority worktree materializes receipt-backed terminal
  projections after exact receipt identity checks and may also converge exact
  typed, claim-released nonterminal projections for documented fail-closed
  exceptions when independent review prohibits a receipt.
- False terminal dispositions require a typed receipt-aware correction route;
  they are never patched directly.
- Exception convergence preserves the complete typed issue projection and
  records its exact phase, generation, digest, claim state, and review blocker;
  it never changes product code or upgrades the exception to `closed_out`.
- Dirty or foreign-owned worktrees are preserved and reported rather than
  overwritten or force-pruned.

## Special cases in the first recovery lane

- #4760: close out merged PR #5740 through the normal typed readiness/closeout
  route, then reconcile the retained receipt.
- #5548: preserve current typed evidence and record an intentional
  superseded/closed-no-PR disposition only if remote and source evidence agree.
- #5632: correct the false `closed_no_pr` receipt to merged PR #5634 through a
  typed receipt-aware route.
- #5665 and #5670: materialize or repair retained design/diagram hygiene while
  preserving issue identity, terminal PR identity, and receipt consistency.

## Validation

- Verify exact-head owner-binary provenance before trusting `csdlc-doctor`.
- Compare the retained live closed-issue universe exactly with the declared
  terminal, exception, and noneligible sets.
- Run `csdlc-doctor` for every reconciled issue.
- Compare the tracked record and all six cards byte-for-byte with the retained
  terminal receipt.
- Validate each full retained receipt digest and identity, then verify retained
  design and diagram files are canonical regular files whose bytes match the
  receipt-authored artifact content.
- Pin exact complete projection identity for intentionally corrupt exceptions
  and reject final, parent-component, or dangling symlinks.
- Run `git diff --check` and reject unrelated, lock, or request-file scope.

## Boundaries

- Typed C-SDLC v2 operations only; no sunset v1/import route.
- No product implementation, AWS, raw GitHub commands, force-prune, or manual
  generated-card/index/receipt edits.
- Never write tracked changes on `main` and never touch #5746's worktree.
