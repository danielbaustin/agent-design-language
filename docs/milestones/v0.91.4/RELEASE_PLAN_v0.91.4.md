# v0.91.4 Release Plan

## Metadata

- Milestone: `v0.91.4`
- Version: `v0.91.4`
- Release date: `2026-06-01`
- Release manager: ADL maintainers

## How To Use

Execute this plan during the release tail. Check items only when evidence
exists. The release ceremony must not hide implementation or truth-maintenance
work.

## 0. Release-Tail Convergence

- [x] Demo/proof coverage complete or gaps routed:
  [DEMO_MATRIX_v0.91.4.md](DEMO_MATRIX_v0.91.4.md)
- [x] Quality gate complete:
  [QUALITY_GATE_v0.91.4.md](QUALITY_GATE_v0.91.4.md)
- [x] C-SDLC default-operation docs complete.
- [x] Durable workflow-state migration proof complete:
  [C_SDLC_TRACKED_WORKFLOW_STATE_MIGRATION_PLAN](C_SDLC_TRACKED_WORKFLOW_STATE_MIGRATION_PLAN_v0.91.4.md)
- [x] Signed trace proof and verification evidence complete as a minimal
  release-input posture.
- [x] CodeFriend pre-alpha sidecar setup complete or truthfully blocked/routed.
- [x] WildClawBench benchmark-spike sidecar complete or truthfully blocked/routed.
- [x] v0.91.5 bridge work routed out of v0.91.4 release scope:
  multi-agent stabilization, provider/model matrix, public prompt records,
  demo readiness, and first-birthday preflight.
- [x] Internal review complete.
- [x] External / third-party review complete.
- [x] Review remediation complete.
- [x] Next milestone planning complete.
- [x] Next milestone handoff refreshed:
  [NEXT_MILESTONE_HANDOFF_v0.91.4.md](NEXT_MILESTONE_HANDOFF_v0.91.4.md)
- [x] Final next-milestone review pass complete.
- [x] Closed issue/card truth refreshed for the full issue wave, including
  closeout-normalization sweep `#3564`.
- [x] Release-truth docs aligned:
  - `README.md`
  - `CHANGELOG.md`
  - `adl/Cargo.toml`
  - [README](README.md)
  - [MILESTONE_CHECKLIST](MILESTONE_CHECKLIST_v0.91.4.md)
  - [RELEASE_NOTES](RELEASE_NOTES_v0.91.4.md)

## 1. Release Readiness

- [x] Milestone checklist complete:
  [MILESTONE_CHECKLIST_v0.91.4.md](MILESTONE_CHECKLIST_v0.91.4.md)
- [x] Release notes approved:
  [RELEASE_NOTES_v0.91.4.md](RELEASE_NOTES_v0.91.4.md)
- [x] Go/no-go decision recorded in:
  [DECISIONS_v0.91.4.md](DECISIONS_v0.91.4.md)

## 2. Branch And Tag Preparation

- [x] Target branch confirmed: `main`
- [x] Working tree clean before WP-21 binding.
- [x] Version strings validated.
- [ ] Tag created: `v0.91.4` after WP-21 merge.
- [ ] Tag pushed and verified after WP-21 merge.

## 3. GitHub Release Steps

- [ ] GitHub Release draft created from `v0.91.4` after WP-21 merge.
- [x] Release body source populated from approved release notes.
- [x] Links to key PRs/issues included in release evidence.
- [ ] Release visibility confirmed after GitHub Release creation.
- [ ] Release published after WP-21 merge and tag.

## 4. Verification

- [ ] Post-release CI status checked after tag/release publication.
- [ ] Release links tested after tag/release publication.
- [x] Immediate known regressions triaged and tracked or routed.

## 5. Communication

- [x] Default C-SDLC operation announced in release notes and milestone docs.
- [x] Roadmap/status docs updated.
- [x] Deferred items routed to v0.91.5 or later backlog.

## Exit Criteria

- No hidden implementation remains in ceremony.
- Tag and release are published and accessible after WP-21 merge.
- Default C-SDLC operation is supported by tracked workflow records, signed trace
  proof, review, and closeout truth.
