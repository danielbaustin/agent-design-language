# Issue 5850 Design: Exact Next-Milestone Closeout Plan

Status: design-time ready; execution waits for reviewed WP-28 handoff truth.

## Authority And Sources

Issue #5850 and WP-28A own the exact terminal sequence between the completed
v0.92 work graph, the WP-29 review, WP-30 ceremony, and later v0.93 activation.
Inputs are the WP-28 handoff, v0.92 WBS/issue-wave/sprint packets, all child and
umbrella typed records, publication intents, terminal receipts, live issues and
PRs, release plan/notes/checklist, and the candidate v0.93 issue universe.

## Outcome Contract

Produce a reviewed closeout plan and machine-readable issue universe. For every
v0.92 child, supporting issue, sprint umbrella, and release issue, record issue
state, PR/base/head/merge, required checks/review, typed phase, SOR integration
truth, terminal receipt, active claim, worktree cleanup eligibility, release
dependency, and exact next action. Define a fail-closed sequence for remaining
PRs, typed finish, claim release, worktree cleanup, WP-29 review, WP-30 tag and
release, umbrella closure, and v0.93 handoff acceptance.

GitHub closure alone is never terminal authority. The plan must distinguish
pre-merge readiness, merged-but-unreconciled state, receipt-backed closeout,
ceremony mutation, and later milestone activation.

## Execution Sequence

1. Verify WP-28 terminal/ancestral truth and freeze the v0.92 issue/PR universe.
2. Reconcile live GitHub state with canonical typed state, SOR, receipts,
   claims, worktrees, and release dependencies for every row.
3. Classify each row as complete, waiting, blocked, reconciliation-required,
   cleanup-ready, or ceremony-only, with one exact owner/action.
4. Topologically order PR completion, finish, claim release, cleanup, WP-29
   review, WP-30 ceremony, umbrella closeout, and v0.93 acceptance.
5. Test negative cases for stale heads, red CI, missing review, dirty worktrees,
   absent receipts, active claims, partial release state, and retry safety.
6. Run exact-head review of the complete universe and sequence.

## Owned Paths

- `docs/milestones/v0.92/V092_TERMINAL_CLOSEOUT_PLAN_5850.md`
- `.csdlc/evidence/5850`
- `.csdlc/prepared/issues/5850/validate-closeout-plan.rb`

## Read-Only Inputs

- Every repository path cited outside `## Owned Paths` is read-only unless it is repeated exactly in that section.
- Dependency records, sibling issue outputs, historical evidence, and external systems remain read-only inputs.

## Validation And Failure Policy

Required lanes are universe completeness, issue/PR/head/merge/check/review
readback, typed phase/SOR/receipt/claim reconciliation, DAG cycle/order checks,
stale/dirty/missing-proof negative fixtures, ceremony idempotence planning,
YAML/JSON/Markdown validation, and exact-head review. Any unknown row or
unowned action blocks plan approval rather than being treated as complete.

## Rollback

Discard the closeout-plan reconstruction and regenerate it from the live nonempty issue, PR, check, review, and typed-state universe. Preserve failed negative fixtures, and do not accept a plan until each one-field mutation produces exactly its expected blocker.

## Non-Goals
- No merge, typed finish, claim release, cleanup, tag, release, or issue close.
- No v0.93 activation or replacement of WP-29/WP-30 authority.
- No rewriting historical records to make the universe appear terminal.
