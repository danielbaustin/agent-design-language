# Milestone Checklist - v0.87.1

## Metadata
- Milestone: `v0.87.1`
- Version: `v0.87.1`
- Target release date: `TBD`
- Owner: `Daniel Austin`

## Purpose
Ship/no-ship gate for the milestone. Check items only when evidence exists.

## Evidence Rule
- Every checked item should point to one concrete proof surface, command result, issue/PR, or published artifact.
- If an item cannot yet be checked, leave it open rather than inferring readiness from surrounding progress.
- If an exception is accepted, record the owner, due date, and disposition in the linked issue or release-tail record instead of silently treating it as green.

## Planning
- [ ] Milestone goal defined (`docs/milestones/v0.87.1/VISION_v0.87.1.md`)
- [ ] Scope + non-goals documented (`docs/milestones/v0.87.1/VISION_v0.87.1.md`)
- [ ] WBS created and mapped to issues (`docs/milestones/v0.87.1/WBS_v0.87.1.md`)
- [ ] WBS acceptance mapping finalized and used as the canonical done contract for downstream demo, checklist, quality, review, and release-tail gates (`docs/milestones/v0.87.1/WBS_v0.87.1.md`)
- [ ] Feature-doc index is present and aligned to the promoted runtime feature set (`docs/milestones/v0.87.1/FEATURE_DOCS_v0.87.1.md`)
- [ ] Decision log initialized (`docs/milestones/v0.87.1/DECISIONS_v0.87.1.md`)
- [ ] Sprint plan created (`docs/milestones/v0.87.1/SPRINT_v0.87.1.md`)
- [ ] Sprint 1 runtime-foundation issues are created and mapped in WBS / sprint docs (`#1435` through `#1442`)
- [ ] Sprint 2 handoff gates and release-surface ordering are explicit and aligned to the WBS (`#1458` through `#1464`)

## Execution Discipline
- [ ] Each issue has input/output cards under `.adl/cards/<issue>/`
- [ ] Each burst writes artifacts under `.adl/reports/burst/<timestamp>/`
- [ ] Draft PR opened for each issue before merge
- [ ] Transient failures retried and documented
- [ ] "Green-only merge" policy followed
- [ ] Canonical milestone docs remain aligned with implementation and proof surfaces throughout execution
- [ ] No downstream sprint slice starts before the prior sprint handoff gate is satisfied (`docs/milestones/v0.87.1/SPRINT_v0.87.1.md`)

## Quality Gates
- [ ] `cargo fmt` passes
- [ ] `cargo clippy --all-targets -- -D warnings` passes
- [ ] `cargo test` passes
- [ ] CI is green on the merge target
- [ ] Coverage signal is not red (or exception documented)
- [ ] No unresolved high-priority blockers (tracked via GitHub issues for v0.87.1)
- [ ] Runtime demo program passes or each non-passing demo has an explicit bounded disposition
- [ ] Quality, demo, and review exceptions map back to a WBS acceptance criterion and include owner-bound disposition
- [ ] Any accepted quality exception records owner, due date, and disposition before release work proceeds

## Review Surfaces
- [ ] Demo matrix finalized (`docs/milestones/v0.87.1/DEMO_MATRIX_v0.87.1.md`)
- [ ] Demo matrix coverage reviewed against WBS acceptance criteria before internal review
- [ ] Internal review complete
- [ ] External / 3rd-party review complete
- [ ] Accepted findings remediated or explicitly deferred with owner and rationale
- [ ] Sprint 3 review-tail order remains intact: internal review -> external / 3rd-party review -> findings remediation -> next milestone planning -> release ceremony

## Release Packaging
- [ ] Release readiness reviewed in the order documented by `docs/milestones/v0.87.1/RELEASE_PLAN_v0.87.1.md`
- [ ] Release notes finalized (`docs/milestones/v0.87.1/RELEASE_NOTES_v0.87.1.md`)
- [ ] Tag verified: `v0.87.1`
- [ ] GitHub Release drafted (GitHub Releases UI)
- [ ] Links validated in release body
- [ ] Release published

## Post-Release
- [ ] Milestone/epic issues closed with release links
- [ ] Deferred items moved to next milestone backlog
- [ ] Follow-up bugs/tech debt captured as issues
- [ ] Roadmap/status docs updated (`docs/milestones/ROADMAP.md` or equivalent)
- [ ] Retrospective summary recorded (project notes or milestone issue thread)

## Exit Criteria
- All required gates are checked, or each exception has an owner + due date.
- Milestone can be audited end-to-end via the links captured above.
- No unchecked gate is being bypassed through undocumented verbal approval.
