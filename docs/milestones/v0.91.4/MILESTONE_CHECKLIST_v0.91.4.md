# v0.91.4 Milestone Checklist

## Metadata

- Milestone: `v0.91.4`
- Version: `v0.91.4`
- Target release date: pending release ceremony
- Owner: ADL maintainers

## Purpose

Ship/no-ship gate for completing the C-SDLC rollout. Check items only when
evidence exists.

## Planning

- [x] Milestone goal defined: [README](README.md)
- [x] Scope and non-goals documented: [DESIGN](DESIGN_v0.91.4.md)
- [x] WBS created: [WBS](WBS_v0.91.4.md)
- [x] Decision log initialized: [DECISIONS](DECISIONS_v0.91.4.md)
- [x] Sprint plan created: [SPRINT](SPRINT_v0.91.4.md)
- [x] Planned issue wave drafted:
  [WP_ISSUE_WAVE](WP_ISSUE_WAVE_v0.91.4.yaml)
- [x] Next-milestone handoff scaffold present and refreshed by WP-19:
  [NEXT_MILESTONE_HANDOFF](NEXT_MILESTONE_HANDOFF_v0.91.4.md)
- [x] CodeFriend sidecar setup plan represented in the milestone package.
- [x] WP-01 opening gate created: `#3346`.
- [x] Sprint 1 issue batch seeded: Sprint 1 `#3347`, WP-02 `#3348`, WP-03
  `#3349`, and WP-04 `#3350`.
- [x] Sprint 2 issue batch seeded: Sprint 2 `#3352`, WP-05 `#3353`, WP-06
  `#3354`, WP-07 `#3355`, and WP-08 `#3356`.
- [x] Sprint 3 issue batch seeded: Sprint 3 `#3357`, WP-09 `#3358`, WP-10
  `#3359`, WP-11 `#3360`, and WP-12 `#3361`.
- [x] Sprint 4 issue batch seeded: Sprint 4 `#3362`, WP-13 `#3363`, WP-14
  `#3364`, WP-15 `#3365`, WP-16 `#3366`, WP-17 `#3367`, WP-18 `#3368`,
  WP-19 `#3369`, WP-20 `#3370`, and WP-21 `#3371`.
- [x] CodeFriend sidecar issue batch seeded: umbrella `#3372`, CF-PRE-01
  `#3373`, CF-PRE-02 `#3374`, CF-PRE-03 `#3375`, and CF-PRE-04 `#3376`.
- [x] WildClawBench sidecar issue batch seeded: umbrella `#3378`, WC-PRE-01
  `#3379`, WC-PRE-02 `#3380`, WC-PRE-03 `#3381`, and WC-PRE-04 `#3382`.

## Execution Discipline

- [x] Every currently opened issue bundle uses `SIP -> STP -> SPP -> SRP -> SOR`.
- [x] `SPP` records design-time issue-local operative plan truth before execution
  for the currently opened batch.
- [ ] New tests or test families land with explicit PVF lane/proof metadata;
  migration rationale is only for pre-existing uncategorized tests.
- [ ] Card edits use editor skills.
- [ ] Work executes in bound worktrees.
- [ ] Draft PR opens before merge for each issue.
- [ ] Pre-PR review is recorded for each implementation issue.
- [ ] Closeout truth is updated after issue closure.
- [ ] Sprint conductor cannot advance or close over stale child truth.

## Quality Gates

- [ ] Demo/proof coverage is complete:
  [DEMO_MATRIX](DEMO_MATRIX_v0.91.4.md)
- [ ] Quality gate is complete:
  [QUALITY_GATE](QUALITY_GATE_v0.91.4.md)
- [ ] Review checklist for new tests and C-SDLC proof includes PVF lane,
  proof role, determinism, resource profile, and release-gate checks.
- [ ] Lifecycle, doctor, conductor, and editor focused tests pass.
- [ ] Signed trace verification passes.
- [ ] Process-drift regression fixtures pass.
- [ ] CI is green on merged code changes.
- [ ] No unresolved P1/P0 findings remain.
- [ ] Deferred issues have owners and follow-on routing.
- [x] CodeFriend sidecar setup is complete or truthfully blocked/routed.
- [x] WildClawBench sidecar spike is complete or truthfully blocked/routed.
- [x] First-birthday readiness side issue `#3377` is routed to v0.91.5.
- [x] Remaining multi-agent, provider/model, public prompt-record, and
  demo-readiness bridge work is routed to v0.91.5.

## Release Packaging

- [ ] Release plan complete: [RELEASE_PLAN](RELEASE_PLAN_v0.91.4.md)
- [ ] Release notes finalized: [RELEASE_NOTES](RELEASE_NOTES_v0.91.4.md)
- [x] Next-milestone handoff refreshed:
  [NEXT_MILESTONE_HANDOFF](NEXT_MILESTONE_HANDOFF_v0.91.4.md)
- [ ] Durable workflow-state migration proof complete:
  [C_SDLC_TRACKED_WORKFLOW_STATE_MIGRATION_PLAN](C_SDLC_TRACKED_WORKFLOW_STATE_MIGRATION_PLAN_v0.91.4.md)
- [ ] Release tag verified: `v0.91.4`
- [ ] GitHub Release drafted and reviewed.
- [ ] Links validated in release body.
- [ ] Release published.

## Post-Release

- [ ] Milestone/epic issues closed with release links.
- [ ] Deferred items moved to the v0.91.5 bridge package or later backlog.
- [ ] Follow-up bugs/tech debt captured as issues.
- [ ] Roadmap/status docs updated.
- [ ] Retrospective or release closeout summary recorded.

## Exit Criteria

- All required gates are checked, or each exception has an owner and due date.
- Future ADL software-development issues have a tracked, repeatable C-SDLC
  default lane.
- Durable workflow records, signed trace proof, review, memory, and release
  evidence are auditable from tracked repo state.
