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
- [x] Next-milestone handoff scaffold present:
  [NEXT_MILESTONE_HANDOFF](NEXT_MILESTONE_HANDOFF_v0.91.4.md)
- [x] CodeFriend sidecar setup plan represented in the milestone package.

## Execution Discipline

- [ ] Every opened issue bundle uses `SIP -> STP -> SPP -> SRP -> SOR`.
- [ ] `SPP` records tracked issue-local operative plan truth before execution.
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
- [ ] Lifecycle, doctor, conductor, and editor focused tests pass.
- [ ] Signed trace verification passes.
- [ ] Process-drift regression fixtures pass.
- [ ] CI is green on merged code changes.
- [ ] No unresolved P1/P0 findings remain.
- [ ] Deferred issues have owners and follow-on routing.
- [ ] CodeFriend sidecar setup is complete or truthfully blocked/routed.

## Release Packaging

- [ ] Release plan complete: [RELEASE_PLAN](RELEASE_PLAN_v0.91.4.md)
- [ ] Release notes finalized: [RELEASE_NOTES](RELEASE_NOTES_v0.91.4.md)
- [ ] Next-milestone handoff refreshed:
  [NEXT_MILESTONE_HANDOFF](NEXT_MILESTONE_HANDOFF_v0.91.4.md)
- [ ] Durable workflow-state migration proof complete:
  [C_SDLC_TRACKED_WORKFLOW_STATE_MIGRATION_PLAN](C_SDLC_TRACKED_WORKFLOW_STATE_MIGRATION_PLAN_v0.91.4.md)
- [ ] Release tag verified: `v0.91.4`
- [ ] GitHub Release drafted and reviewed.
- [ ] Links validated in release body.
- [ ] Release published.

## Post-Release

- [ ] Milestone/epic issues closed with release links.
- [ ] Deferred items moved to the next milestone backlog.
- [ ] Follow-up bugs/tech debt captured as issues.
- [ ] Roadmap/status docs updated.
- [ ] Retrospective or release closeout summary recorded.

## Exit Criteria

- All required gates are checked, or each exception has an owner and due date.
- Future ADL software-development issues have a tracked, repeatable C-SDLC
  default lane.
- Durable workflow records, signed trace proof, review, memory, and release
  evidence are auditable from tracked repo state.
