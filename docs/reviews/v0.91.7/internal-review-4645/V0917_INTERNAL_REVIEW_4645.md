# v0.91.7 Internal Milestone Review (#4645)

Status: internal_review_recorded_with_blocking_findings

Issue: #4645

Captured: 2026-07-18

## Summary

The v0.91.7 internal review covered the milestone issue wave, retained sprint
review packets, live issue/PR state, C-SDLC card coverage, CI/coverage truth,
code/architecture surfaces, security/dependency posture, and release-evidence
boundaries.

The milestone is not release-ready. WP-19 external review should not start from
this packet until the blocking findings are resolved or explicitly dispositioned.

The most important current blocker is #5408: WP-07 runtime hardening remains
open and underway. At snapshot time, PR #5419 was open, draft, and still had
non-terminal CI/coverage checks. This review proceeds because the operator
explicitly asked to start #4645 while noting #5408 is still open; it does not
turn #5408 into completed evidence.

## Retained Artifacts

- Findings register: `docs/reviews/v0.91.7/internal-review-4645/FINDINGS_REGISTER.md`
- Specialist-lane results: `docs/reviews/v0.91.7/internal-review-4645/SPECIALIST_LANE_RESULTS.md`
- Validation summary: `docs/reviews/v0.91.7/internal-review-4645/VALIDATION.md`
- Repository packet: `docs/reviews/v0.91.7/internal-review-4645/packet/`
- Live-state snapshot: `docs/reviews/v0.91.7/internal-review-4645/live-state/`

## Live-State Snapshot

The live-state snapshot recorded:

- 425 GitHub issues matching `version:v0.91.7`, with 7 still open.
- 488 GitHub issue title hits for `v0.91.7`, with 19 still open.
- 438 v0.91.7 PR hits.
- 41 local C-SDLC issue bundles in this merged #4645 worktree.
- No missing local C-SDLC card files among those 41 bundles.
- #5408 open.
- PR #5419 open and draft, with two non-terminal checks at capture time.

The version-labeled open set included #4645, #4646, #4647, #4650, #5408,
#5489, and #5527. Some title-search hits also include v0.91.8-scoped carryover
issues and are not counted as v0.91.7 release blockers unless separately
labeled or routed.

## Findings Summary

P1 findings:

- #5408 remains an open WP-07 hardening blocker.
- WP-19 external review remains gated by incomplete WP-18/WP-20 truth.
- The canonical sprint review register is stale against current live and
  terminal remediation truth.
- Hosted provider endpoints can be overridden to plaintext HTTP while sending
  hosted-provider credentials.

P2 findings:

- C-SDLC terminal truth lags live GitHub closure for some release-tail issues.
- PR-fast coverage must not be treated as release coverage approval; nightly
  coverage per-file scope is narrower than the merge gate it claims to match.
- CSM metrics can parse the full operator event log on each request.
- ACIP websocket fail-closed statuses collapse to HTTP 500.
- C-SDLC v2 dependency/supply-chain release-review proof is incomplete.
- AWS CodeFriend manual build command boundary needs explicit documentation or
  enforcement before external review.

P3 findings:

- C-SDLC publication revision identity ignores declared review scope.
- Large core modules continue to concentrate review-sensitive responsibilities.

Full details and routes are in `FINDINGS_REGISTER.md`.

## WP Coverage

WP-01 through WP-23 were covered through the issue wave, sprint register, retained
review packets, live issue state, and downstream quality/demo/review records.
Coverage is not the same as approval.

Current review truth:

- WP-01 through WP-16 have substantial retained evidence, but the register needs
  reconciliation for recent WP-14/WP-15/WP-16 closure and later remediation
  truth.
- WP-17 is no longer in the version-labeled open set at the latest snapshot;
  title-search carryovers still need register reconciliation where stale.
- WP-18 is this review and remains open until this packet is published and routed.
- WP-19 must remain blocked.
- WP-20 is the natural remediation owner for this packet's findings.
- WP-21 is closed/superseded for old direct v0.92 candidate consumption.
- WP-21A is open for next milestone docs closeout planning.
- WP-22 is closed as retained next-milestone review evidence in current coverage
  docs but still needs register reconciliation where stale.
- WP-23 must remain open until review/remediation gates are clean or explicitly
  blocked with operator approval.

## Release Boundary

This internal review does not approve:

- v0.91.7 release readiness
- v0.92 activation readiness
- WP-19 external review
- WP-23 release ceremony
- default Runtime v3 cutover
- broader AWS/runtime/provider production readiness beyond retained proof
- full workspace coverage from PR-fast checks

## Required Next Actions

1. Continue #5408 / PR #5419 until terminal, or record explicit operator-approved
   blocker disposition.
2. Reconcile the sprint review register to current WP-14/WP-15/WP-16, #5404,
   #5413, and open closeout-reconciliation truth.
3. Route all P1/P2 findings through WP-20/#4647 or existing owner issues.
4. Refresh the external-review handoff only after the blocking set is resolved
   or explicitly dispositioned.
