# Milestone Checklist Template

## Metadata
- Milestone: `{{milestone}}`
- Version: `{{version}}`
- Target release date: `{{target_release_date}}`
- Owner: `{{owner}}`

## How To Use
- Check items only when evidence exists (issue, PR, report, or run link).
- Keep links beside each completed checklist item.
- Treat this as the ship/no-ship gate document.

## 1) Planning Prerequisites
- [ ] Milestone goal defined and approved (`{{goal_doc_link}}`)
- [ ] Scope and non-goals documented (`{{scope_doc_link}}`)
- [ ] WBS created with issue mapping (`{{wbs_link}}`)
- [ ] Decision log initialized (`{{decisions_link}}`)
- [ ] Sprint plan and sequencing agreed (`{{sprint_plan_link}}`)

## 2) Development Cadence (Per Burst)
- [ ] Every issue has input/output cards under `.adl/cards/<issue>/`
- [ ] Burst artifacts written under `.adl/reports/burst/<timestamp>/`
- [ ] Work is executed in deterministic order per issue plan
- [ ] Draft PR opened for each issue before merge
- [ ] Transient failures retried and documented

## 3) Pre-Release Hardening Gates
- [ ] Formatting gate passes (`cargo fmt` where applicable)
- [ ] Lint gate passes (`cargo clippy --all-targets -- -D warnings` where applicable)
- [ ] Test gate passes (`cargo test` where applicable)
- [ ] CI checks green on release branch/PR
- [ ] Coverage signal not red (coverage workflow green or documented exception)
- [ ] No unresolved high-priority blockers (`{{blocker_report_link}}`)

## 4) Release Packaging
- [ ] Release notes finalized (`{{release_notes_link}}`)
- [ ] Version/tag verified (`{{tag_name}}`)
- [ ] GitHub Release draft created (`{{release_draft_link}}`)
- [ ] Artifacts/links validated in release body
- [ ] Final release published

## 5) Post-Release Cleanup
- [ ] Milestone issue closed with release links
- [ ] Deferred items moved to next milestone backlog
- [ ] Follow-up bugs/tech debt captured as issues
- [ ] Documentation index/roadmap updated
- [ ] Retrospective summary recorded (`{{retro_link}}`)

## Exit Criteria
- All required planning, hardening, and release packaging gates are checked.
- Any unchecked item has an explicit, approved exception with owner and due date.
- Milestone can be audited end-to-end from checklist links.
