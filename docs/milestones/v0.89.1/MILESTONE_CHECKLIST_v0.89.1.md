# Milestone Checklist - v0.89.1

## Metadata
- Milestone: `v0.89.1`
- Version: `v0.89.1`
- Target release date: `TBD`
- Owner: `Daniel Austin`

## Purpose

Ship/no-ship gate for `v0.89.1`. Check items only when evidence exists.

## Planning
- [x] Milestone goal defined (`README.md`, `VISION_v0.89.1.md`)
- [x] Scope + non-goals documented (`DESIGN_v0.89.1.md`)
- [x] WBS created and ready for later issue-wave seeding (`WBS_v0.89.1.md`)
- [x] Decision log initialized (`DECISIONS_v0.89.1.md`)
- [x] Sprint plan created (`SPRINT_v0.89.1.md`)
- [x] Feature index created with explicit carry-forward mapping (`FEATURE_DOCS_v0.89.1.md`)
- [x] Official issue-wave plan drafted and review-ready (`WP_ISSUE_WAVE_v0.89.1.yaml`)
- [x] Official `v0.89.1` issue wave opened

## Execution Discipline
- [ ] Each issue has truthful local input/output cards in the ADL control plane
- [ ] Draft PR opened for each tracked issue before merge where applicable
- [ ] Queue and closeout discipline followed consistently
- [ ] The `v0.89` / `v0.89.1` boundary remains explicit rather than blurred by carry-back scope
- [ ] Green-only merge policy followed

## Quality Gates
- [ ] `cargo fmt` passes
- [ ] `cargo clippy --all-targets -- -D warnings` passes
- [ ] `cargo test` passes
- [ ] CI is green on the merge target
- [ ] Coverage signal is not red (or exception documented)
- [ ] No unresolved high-priority blockers remain at release time

## Release Packaging
- [ ] Release notes finalized (`RELEASE_NOTES_v0.89.1.md`)
- [ ] Tag verified: `v0.89.1`
- [ ] GitHub Release drafted
- [ ] Links validated in release body
- [ ] Release published

## Post-Release
- [ ] Milestone issues closed with release links
- [ ] Deferred items moved to next milestone backlog / planning surface
- [ ] Follow-up bugs and debt captured explicitly
- [ ] Roadmap/status docs updated
- [ ] Retrospective summary recorded

## Current Planning Truth

At the moment this checklist should be read as:
- planning-complete and review-ready
- issue wave opened; `WP-02` - `WP-11` landed and `WP-12` is the active convergence gate
- integration demos, manuscript packet completion, quality, docs/review, internal review, 3rd-party review, remediation, next-milestone planning, and release ceremony are still unchecked release-tail work
- execution, review, and release gates still future work

## Exit Criteria
- all required gates are checked, or each exception has an owner and explicit rationale
- the milestone can be audited end to end via the linked docs, issues, PRs, and proof surfaces
