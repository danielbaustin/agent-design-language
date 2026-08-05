# Structured Task Prompt

Template: 1.0.0

Issue: 5850

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Deliver exact terminal issue, PR, receipt, and ceremony sequence.

## Deliverables

- exact terminal issue, PR, receipt, and ceremony sequence
- reviewed closeout plan and issue universe

## Acceptance

1. AC-1: WP-28 is merged, terminal, claim-free, ancestral, and the complete v0.92 child/supporting/sprint/release issue universe is pinned to current live and repository truth.
2. AC-2: Every row records issue, PR/base/head/merge, checks/review, typed phase, SOR integration, terminal receipt, claim, worktree cleanup, release dependency, classification, owner, and exact next action.
3. AC-3: The plan distinguishes pre-merge, merged-unreconciled, receipt-backed closed_out, cleanup-ready, ceremony-only, and later v0.93 acceptance truth without treating GitHub closure alone as terminal.
4. AC-4: The dependency DAG orders PR completion, typed finish, claim release, cleanup, WP-29 review, WP-30 tag/release, umbrella closeout, and handoff acceptance without cycles or authority inversion.
5. AC-5: Negative cases reject stale heads, red checks, missing review/receipt, active claims, dirty worktrees, partial release identity, duplicate mutation, unknown rows, and unowned actions.
6. AC-6: Exact-head independent review has no actionable finding and the plan performs no merge, finish, cleanup, tag, release, close, or activation mutation.

## Dependencies

- WP-28

## Inputs

- Terminal WP-28 handoff and current v0.92 WBS, issue wave, sprint packets, release docs, and checklist
- Live GitHub issue/PR/check/review state plus canonical .csdlc phase, SOR, receipt, claim, publication, and cleanup truth
- Current v0.93 candidate handoff and activation boundary

## Non Goals

- Merging, finishing, releasing claims, cleaning worktrees, tagging, releasing, or closing issues/sprints
- Treating GitHub closure alone as terminal or rewriting historical records
- Executing WP-29/WP-30 or activating v0.93
