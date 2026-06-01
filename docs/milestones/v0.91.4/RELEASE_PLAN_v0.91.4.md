# v0.91.4 Release Plan

## Metadata

- Milestone: `v0.91.4`
- Version: `v0.91.4`
- Release date: pending release ceremony
- Release manager: ADL maintainers

## How To Use

Execute this plan during the release tail. Check items only when evidence
exists. The release ceremony must not hide implementation or truth-maintenance
work.

## 0. Release-Tail Convergence

- [ ] Demo/proof coverage complete or gaps routed:
  [DEMO_MATRIX_v0.91.4.md](DEMO_MATRIX_v0.91.4.md)
- [ ] Quality gate complete:
  [QUALITY_GATE_v0.91.4.md](QUALITY_GATE_v0.91.4.md)
- [ ] C-SDLC default-operation docs complete.
- [ ] Durable workflow-state migration proof complete:
  [C_SDLC_TRACKED_WORKFLOW_STATE_MIGRATION_PLAN](C_SDLC_TRACKED_WORKFLOW_STATE_MIGRATION_PLAN_v0.91.4.md)
- [ ] Signed trace proof and verification evidence complete.
- [ ] CodeFriend pre-alpha sidecar setup complete or truthfully blocked/routed.
- [ ] WildClawBench benchmark-spike sidecar complete or truthfully blocked/routed.
- [ ] v0.91.5 bridge work routed out of v0.91.4 release scope:
  multi-agent stabilization, provider/model matrix, public prompt records,
  demo readiness, and first-birthday preflight.
- [x] Internal review complete.
- [ ] External / third-party review complete.
- [ ] Review remediation complete.
- [ ] Next milestone planning complete.
- [ ] Next milestone handoff refreshed:
  [NEXT_MILESTONE_HANDOFF_v0.91.4.md](NEXT_MILESTONE_HANDOFF_v0.91.4.md)
- [ ] Final next-milestone review pass complete.
- [ ] Closed issue/card truth refreshed for the full issue wave.
- [ ] Release-truth docs aligned:
  - `README.md`
  - `CHANGELOG.md`
  - `adl/Cargo.toml`
  - [README](README.md)
  - [MILESTONE_CHECKLIST](MILESTONE_CHECKLIST_v0.91.4.md)
  - [RELEASE_NOTES](RELEASE_NOTES_v0.91.4.md)

## 1. Release Readiness

- [ ] Milestone checklist complete:
  [MILESTONE_CHECKLIST_v0.91.4.md](MILESTONE_CHECKLIST_v0.91.4.md)
- [ ] Release notes approved:
  [RELEASE_NOTES_v0.91.4.md](RELEASE_NOTES_v0.91.4.md)
- [ ] Go/no-go decision recorded in:
  [DECISIONS_v0.91.4.md](DECISIONS_v0.91.4.md)

## 2. Branch And Tag Preparation

- [ ] Target branch confirmed: `main`
- [ ] Working tree clean.
- [ ] Version strings validated.
- [ ] Tag created: `v0.91.4`
- [ ] Tag pushed and verified.

## 3. GitHub Release Steps

- [ ] GitHub Release draft created from `v0.91.4`.
- [ ] Release body populated from approved release notes.
- [ ] Links to key PRs/issues included.
- [ ] Release visibility confirmed.
- [ ] Release published.

## 4. Verification

- [ ] Post-release CI status checked.
- [ ] Release links tested.
- [ ] Immediate regressions triaged and tracked.

## 5. Communication

- [ ] Default C-SDLC operation announced internally.
- [ ] Roadmap/status docs updated.
- [ ] Deferred items routed to v0.91.5 or later backlog.

## Exit Criteria

- No hidden implementation remains in ceremony.
- Tag and release are published and accessible.
- Default C-SDLC operation is supported by tracked workflow records, signed trace
  proof, review, and closeout truth.
