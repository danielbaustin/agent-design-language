# v0.91.6 Workflow-Control Tools Mini-Sprint SEP

Status: `closed_child_wave_ready_for_umbrella_closeout`
Date: 2026-06-19
Sprint umbrella: `#4149`
Execution mode: `hybrid`
Sprint review path:
`docs/milestones/v0.91.6/review/V0916_WORKFLOW_CONTROL_TOOLS_MINI_SPRINT_REVIEW_4149.md`

This Sprint Execution Packet records the bounded execution contract for the
workflow-control tools mini-sprint. It does not replace issue-local
`SIP -> STP -> SPP -> SRP -> SOR` truth, and it does not authorize scope
outside the listed child issues.

## Sprint Goal

Repair the remaining v0.91.6 workflow-control friction around PR lifecycle
truth, GitHub token/context handling, fresh worktree bundles, stale worktree
closeout, publication-path diagnostics, and queue-pressure gating.

## Scope Boundary

In scope:

- `#4083`
- `#4085`
- `#4086`
- `#4087`
- `#4088`
- `#4094`
- umbrella readiness, review, and closeout truth for `#4149`

Out of scope:

- `#4047`
- `#4109`
- `#4113`
- `#4095`
- `#4096`
- refactoring-sprint or toolkit-simplification work not named above

## Child Issue Wave

| Issue | Role | Status | Notes |
|---|---|---|---|
| `#4085` | non-closing lifecycle PR declaration | completed | PR `#4170` merged and issue `#4085` closed as completed on 2026-06-19. |
| `#4087` | configured token context inheritance | completed | PR `#4172` merged and issue `#4087` closed as completed on 2026-06-19. |
| `#4088` | fresh-worktree task-bundle materialization | completed | PR `#4173` merged and issue `#4088` closed as completed on 2026-06-19. |
| `#4086` | stale dirty worktree closeout quarantine | completed | PR `#4174` merged and issue `#4086` closed as completed on 2026-06-19. |
| `#4094` | `pr finish` output-card/card-path routing | completed | PR `#4175` merged and issue `#4094` closed as completed on 2026-06-19. |
| `#4083` | doctor/open-wave queue pressure behavior | completed | PR `#4184` merged and issue `#4083` closed as completed on 2026-06-19. |

## Recommended Execution Order

1. `#4085` completed through PR `#4170`.
2. `#4087` completed through PR `#4172`.
3. `#4088` completed through PR `#4173`.
4. `#4086` completed through PR `#4174`.
5. `#4094` completed through PR `#4175`.
6. `#4083` completed through PR `#4184`.
7. `#4149` may close after final umbrella SOR truth records this child-wave completion and the sprint review packet below remains current.

## Safe Parallel Lanes

| Lane | Issues | Why parallel-safe | Required coordination |
|---|---|---|---|
| credential/context | `#4087` | Token/context loading can proceed independently if it does not edit publication or closeout routing surfaces. | Verify no secrets are printed and covered operations avoid raw `gh` fallback. |
| publication-contract | `#4085`, `#4094` | Both concern `pr finish`, but can only overlap if the active write sets are proven disjoint. | Prefer `#4085` first; if parallelized, compare touched files before editing and reconcile diagnostics before PR publication. |
| worktree/closeout | `#4088`, `#4086` | Same conceptual area, but not parallel-safe by default because closeout worktree resolution can collide. | Treat as sequential unless a fresh route check proves isolation. |
| doctor/queue | `#4083` | Queue-pressure policy can be reviewed independently after lifecycle fixes settle. | Run late and include current open-wave behavior in proof. |

## Serial Gates

| Gate | Blocks | Exit condition |
|---|---|---|
| sprint readiness gate | all child execution | This SEP exists, child bundles exist, and child `SIP`/`STP`/`SPP`/`SRP` design-time readiness passes. |
| finish contract gate | `#4094` | Satisfied by merged PRs `#4170` and `#4175`. |
| fresh-worktree gate | `#4086` | Satisfied by merged PRs `#4173` and `#4174`. |
| queue gate | `#4083` | Satisfied by merged PR `#4184`; doctor now reports issue-local readiness as `PASS` with unrelated open-wave pressure classified separately. |
| closeout gate | `#4149` closure | Satisfied for the child wave: all six child issues are closed, their PRs are merged, child worktrees are pruned, and this sprint review packet is recorded. |

## Watcher Policy

- Every child issue PR must have an issue-local watcher or janitor handoff for checks, review state, mergeability, and closeout readiness.
- Healthy PR-open states are watcher-owned lifecycle states, not sprint stop points.
- Blocked, flaky, stale, skipped, deferred, or out-of-scope findings must be recorded in the child issue cards and the sprint activity log.
- A child issue may not be treated as complete until GitHub issue/PR state, local lifecycle cards, and closeout truth agree.
- Completed issues must not remain open; merged worktrees must be pruned through closeout.

## PVF Notes

- `#4085`: prove non-closing PR body/linkage guardrail behavior.
- `#4087`: prove configured token/context loading with no printed secrets and no raw `gh` fallback for covered operations.
- `#4088`: prove fresh-worktree task-bundle materialization.
- `#4086`: prove stale dirty worktree quarantine or ignore behavior during closeout.
- `#4094`: prove `pr finish` output-card/card-path diagnostics and routing behavior.
- `#4083`: prove doctor queue-pressure classification and override guidance.

Validation must be focused per child issue. Aggregate sprint proof must not hide
failed, pending, blocked, skipped, or deferred lanes.

## Sprint Review And Closeout

Sprint review must collect child issue PRs, changed implementation surfaces,
validation notes, blocked/deferred findings, and closeout evidence. The review
must include code-facing and test-facing review coverage before `#4149` closes.
The review artifact is recorded at
`docs/milestones/v0.91.6/review/V0916_WORKFLOW_CONTROL_TOOLS_MINI_SPRINT_REVIEW_4149.md`
with the child-wave execution evidence reviewed for umbrella closeout.

Closeout must verify:

- all six child issues are closed;
- each completed child has PR, validation, SRP, SOR, and closeout truth aligned;
- all merged child worktrees are pruned;
- the sprint review artifact exists;
- the `#4149` SOR records final integration truth.

## Current Preparation Notes

- `#4149` was bound under `--allow-open-pr-wave` because draft PR `#4152`
  was unrelated queue pressure.
- The sprint child list remains exactly `#4085`, `#4087`, `#4088`, `#4086`,
  `#4094`, and `#4083`.
- Child issue execution completed for all six children on 2026-06-19.
- PR `#4184` for `#4083` exposed one workflow-tools remediation note: the
  repo-native `pr finish --merge` path saw green checks but did not transition
  the draft PR to ready before waiting on aggregate validation. A bounded manual
  `gh pr ready 4184` intervention was used, then `pr finish --merge` completed
  the merge and closeout. This is recorded as sprint residual tooling friction,
  not as additional sprint implementation scope.
