# Issue 5737 Design

## Boundary

Issue #5737 repairs C-SDLC v2 claim authority recovery so unrelated stale terminal receipts and stale non-current checkout projections do not block non-overlapping claim scans, while real live protected-path overlaps still fail closed.

## Approach

- Check terminal-release identity only after a protected-path overlap is detected.
- Ignore stale claim projections whose recorded branch and worktree do not match their active checkout before treating them as collision authority.
- Preserve checkout, lease, CAS, active-branch, and protected-path collision checks.
- Use an authority-only record replacement for claim reacquisition so dormant stale design inputs can be repaired through typed design reapproval.
- Add Gate 2 regressions for init, bind, reacquire, dormant design reapproval, stale projection filtering, and live overlap fail-closed behavior.

## Validation

Focused proof is the Gate 2 regression test suite for the authority-recovery cases plus strict Clippy over `csdlc-v2`.
