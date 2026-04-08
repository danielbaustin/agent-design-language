# Milestone Checklist - v0.87.1

## Metadata
- Milestone: `v0.87.1`
- Version: `v0.87.1`
- Target release date: `TBD`
- Owner: `Daniel Austin`

## Purpose
Ship/no-ship gate for the milestone. Check items only when evidence exists.

## Planning
- [ ] Milestone goal defined (`docs/milestones/v0.87.1/VISION_v0.87.1.md`)
- [ ] Scope + non-goals documented (`docs/milestones/v0.87.1/VISION_v0.87.1.md`)
- [ ] WBS created and mapped to issues (`docs/milestones/v0.87.1/WBS_v0.87.1.md`)
- [ ] Decision log initialized (`docs/milestones/v0.87.1/DECISIONS_v0.87.1.md`)
- [ ] Sprint plan created (`docs/milestones/v0.87.1/SPRINT_v0.87.1.md`)

## Execution Discipline
- [ ] Each issue has input/output cards under `.adl/cards/<issue>/`
- [ ] Each burst writes artifacts under `.adl/reports/burst/<timestamp>/`
- [ ] Draft PR opened for each issue before merge
- [ ] Transient failures retried and documented
- [ ] "Green-only merge" policy followed
- [ ] Canonical milestone docs remain aligned with implementation and proof surfaces throughout execution

## Quality Gates
- [ ] `cargo fmt` passes
- [ ] `cargo clippy --all-targets -- -D warnings` passes
- [ ] `cargo test` passes
- [ ] CI is green on the merge target
- [ ] Coverage signal is not red (or exception documented)
- [ ] No unresolved high-priority blockers (tracked via GitHub issues for v0.87.1)
- [ ] Runtime demo program passes or each non-passing demo has an explicit bounded disposition

## Review Surfaces
- [ ] Demo matrix finalized (`docs/milestones/v0.87.1/DEMO_MATRIX_v0.87.1.md`)
- [ ] Internal review complete
- [ ] External / 3rd-party review package prepared
- [ ] Accepted findings remediated or explicitly deferred with owner and rationale

## Release Packaging
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
