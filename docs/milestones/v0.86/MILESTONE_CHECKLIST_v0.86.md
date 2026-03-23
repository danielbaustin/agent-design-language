# Milestone Checklist Template

## Metadata
- Milestone: `{{milestone}}`
- Version: `{{version}}`
- Target release date: `{{target_release_date}}`
- Owner: `{{owner}}`

## Purpose
Ship/no-ship gate for the milestone. Check items only when evidence exists.

## Planning
- [ ] Milestone goal defined (`{{goal_doc_link}}`)
- [ ] Scope + non-goals documented (`{{scope_doc_link}}`)
- [ ] WBS created and mapped to issues (`{{wbs_link}}`)
- [ ] Decision log initialized (`{{decisions_link}}`)
- [ ] Sprint plan created (`{{sprint_plan_link}}`)

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
- [ ] Coverage signal is not red (or exception documented) (`{{coverage_link_or_note}}`)
- [ ] No unresolved high-priority blockers (`{{blocker_report_link}}`)

## Release Packaging
- [ ] Release notes finalized (`{{release_notes_link}}`)
- [ ] Tag verified: `{{tag_name}}`
- [ ] GitHub Release drafted (`{{release_draft_link}}`)
- [ ] Links validated in release body
- [ ] Release published

## Post-Release
- [ ] Milestone/epic issues closed with release links
- [ ] Deferred items moved to next milestone backlog
- [ ] Follow-up bugs/tech debt captured as issues
- [ ] Roadmap/status docs updated (`{{roadmap_update_link}}`)
- [ ] Retrospective summary recorded (`{{retro_link}}`)

## Exit Criteria
- All required gates are checked, or each exception has an owner + due date.
- Milestone can be audited end-to-end via the links captured above.
