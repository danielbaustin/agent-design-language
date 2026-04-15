# Milestone Checklist - v0.89

## Metadata
- Milestone: `v0.89`
- Version: `v0.89`
- Target release date: `TBD`
- Owner: `Daniel Austin`

## Purpose

Ship/no-ship gate for `v0.89`. Check items only when evidence exists.

## Planning
- [x] Milestone goal defined (`README.md`, `VISION_v0.89.md`)
- [x] Scope + non-goals documented (`DESIGN_v0.89.md`)
- [x] WBS created and ready for the implementation issue wave (`WBS_v0.89.md`)
- [x] Decision log initialized (`DECISIONS_v0.89.md`)
- [x] Sprint plan created (`SPRINT_v0.89.md`)
- [x] Feature index created with explicit carry-forward mapping (`FEATURE_DOCS_v0.89.md`)
- [x] Core implementation issue wave opened

## Execution Discipline
- [ ] Each issue has truthful input/output cards under `.adl/`
- [ ] Draft PR opened for each tracked issue before merge where applicable
- [ ] Queue and closeout discipline followed consistently
- [x] Carry-forward to `v0.89.1` remains explicit rather than implicit
- [ ] Green-only merge policy followed

## Quality Gates
- [ ] `cargo fmt` passes
- [ ] `cargo clippy --all-targets -- -D warnings` passes
- [ ] `cargo test` passes
- [ ] CI is green on the merge target
- [ ] Coverage signal is not red (or exception documented)
- [ ] No unresolved high-priority blockers remain at release time

## Release Packaging
- [ ] Release notes finalized (`RELEASE_NOTES_v0.89.md`)
- [ ] Tag verified: `v0.89`
- [ ] GitHub Release drafted
- [ ] Links validated in release body
- [ ] Release published

## Post-Release
- [ ] Milestone issues closed with release links
- [ ] Deferred items moved to next milestone backlog / planning surface
- [ ] Follow-up bugs and debt captured explicitly
- [ ] Roadmap/status docs updated
- [ ] Retrospective summary recorded

## Exit Criteria
- all required gates are checked, or each exception has an owner and explicit rationale
- the milestone can be audited end to end via the linked docs, issues, PRs, and proof surfaces
