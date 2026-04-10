# Milestone Checklist - v0.88

## Metadata
- Milestone: `v0.88`
- Version: `v0.88`
- Target release date: `TBD`
- Owner: `Daniel Austin`

## Purpose
Ship/no-ship gate for the milestone. Check items only when evidence exists.

## Planning
- [ ] Milestone goal defined (`docs/milestones/v0.88/VISION_v0.88.md`)
- [ ] Scope + non-goals documented (`docs/milestones/v0.88/DESIGN_v0.88.md`)
- [ ] WBS created and mapped to issues (`docs/milestones/v0.88/WBS_v0.88.md`)
- [ ] Decision log initialized (`docs/milestones/v0.88/DECISIONS_v0.88.md`)
- [ ] Sprint plan created (`docs/milestones/v0.88/SPRINT_v0.88.md`)

## Execution Discipline
- [ ] Each issue has input/output cards under `.adl/cards/<issue>/`
- [ ] Each burst writes artifacts under `.adl/reports/burst/<timestamp>/`
- [ ] Draft PR opened for each issue before merge
- [ ] Transient failures retried and documented
- [ ] "Green-only merge" policy followed

## Quality Gates
- [ ] `cargo fmt` passes
- [ ] `cargo clippy --all-targets -- -D warnings` passes
- [ ] `cargo test` passes
- [ ] CI is green on the merge target
- [ ] Coverage signal is not red (or exception documented) (`TBD during v0.88 planning`)
- [ ] No unresolved high-priority blockers (`TBD during v0.88 planning`)

## Release Packaging
- [ ] Release notes finalized (`docs/milestones/v0.88/RELEASE_NOTES_v0.88.md`)
- [ ] Tag verified: `v0.88`
- [ ] GitHub Release drafted (`TBD during v0.88 planning`)
- [ ] Links validated in release body
- [ ] Release published

## Post-Release
- [ ] Milestone/epic issues closed with release links
- [ ] Deferred items moved to next milestone backlog
- [ ] Follow-up bugs/tech debt captured as issues
- [ ] Roadmap/status docs updated (`TBD during v0.88 planning`)
- [ ] Retrospective summary recorded (`TBD during v0.88 planning`)

## Exit Criteria
- All required gates are checked, or each exception has an owner + due date.
- Milestone can be audited end-to-end via the links captured above.
