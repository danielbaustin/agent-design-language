# Release Plan - v0.87.1

## Metadata
- Milestone: `v0.87.1`
- Version: `v0.87.1`
- Release date: `TBD`
- Release manager: Daniel Austin

## How To Use
- Execute sections in order and capture links for each completed step.
- Keep this doc focused on shipping mechanics; use release notes for narrative.
- Mark blockers immediately; do not publish until gates pass.
- Treat every unchecked or partially satisfied gate as a blocker unless an owner-bound disposition is recorded explicitly.

## Exception Rule
- If a release gate cannot be satisfied yet, record the owning issue/person, due date, and disposition before proceeding.
- Do not treat "known but acceptable" as implicit approval; unresolved exceptions must be visible in the release-tail record.

## 1) Release Readiness
- [ ] Milestone checklist complete (`docs/milestones/v0.87.1/MILESTONE_CHECKLIST_v0.87.1.md`)
- [ ] WBS acceptance mapping reviewed against demo, quality, review, and release-tail evidence (`docs/milestones/v0.87.1/WBS_v0.87.1.md`)
- [ ] Sprint sequencing and handoff gates reviewed against the release-tail issue order (`docs/milestones/v0.87.1/SPRINT_v0.87.1.md`)
- [ ] Demo matrix coverage and quality posture reviewed before review-tail closeout (`docs/milestones/v0.87.1/DEMO_MATRIX_v0.87.1.md`, `#1463`)
- [ ] Release notes approved (`docs/milestones/v0.87.1/RELEASE_NOTES_v0.87.1.md`)
- [ ] Go/no-go decision recorded (`docs/milestones/v0.87.1/DECISIONS_v0.87.1.md` or milestone issue thread)
- [ ] Internal review and external / 3rd-party review outcomes dispositioned
- [ ] Accepted findings remediated or explicitly deferred with owner, rationale, and follow-up issue before release ceremony begins
- [ ] Next-milestone planning package prepared before release closeout

## 2) Branch And Tag Preparation
- [ ] Target branch confirmed (main)
- [ ] Working tree clean
- [ ] Version string(s) validated (milestone docs + release notes reviewed for v0.87.1 consistency)
- [ ] Release blockers and accepted exceptions rechecked immediately before tagging
- [ ] Tag created: v0.87.1
- [ ] Tag pushed and verified

## 3) GitHub Release Steps
- [ ] GitHub Release draft created from v0.87.1 (GitHub Releases UI)
- [ ] Release body populated from approved notes
- [ ] Links to key PRs/issues included
- [ ] Release visibility confirmed (draft/prerelease/final)
- [ ] Release published

## 4) Verification
- [ ] Post-release CI status checked (GitHub Actions / CI runs for main)
- [ ] Release links tested (docs, artifacts, notes)
- [ ] Immediate regressions triaged and tracked (GitHub issues / milestone thread)
- [ ] Any post-release exception is linked back to the owning milestone gate or follow-up issue

## 5) Communication
- [ ] Community announcement published (release notes / GitHub Release or explicitly skipped if this milestone remains internal)
- [ ] Internal update posted (project notes / milestone thread)
- [ ] Roadmap/status updated (docs/milestones/ROADMAP.md or equivalent)

## Exit Criteria
- Tag and GitHub Release are published and accessible.
- Verification completed with no unknown critical failures.
- Communication links captured.
- Remaining exceptions, if any, are explicitly dispositioned rather than left implicit.
