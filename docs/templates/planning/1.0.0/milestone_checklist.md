# Milestone Checklist Template

## Metadata
- Milestone: `<milestone>`
- Version: `<version>`
- Target release date: `<target_release_date>`
- Owner: `<owner>`

## Purpose
Ship/no-ship gate for the milestone. Check items only when evidence exists.

## Planning
- [ ] Milestone goal defined (`<goal_doc_link>`)
- [ ] Scope + non-goals documented (`<scope_doc_link>`)
- [ ] WBS created and mapped to issues (`<wbs_link>`)
- [ ] Decision log initialized (`<decisions_link>`)
- [ ] Sprint plan created (`<sprint_plan_link>`)

## Execution Discipline
- [ ] New issue bundles use the target lifecycle
  `SIP -> STP -> SPP -> SRP -> SOR`, or legacy compatibility exceptions are
  explicitly documented
- [ ] Required proof artifacts are recorded in issue-local SORs or review/evidence docs
- [ ] Draft PR opened for each issue before merge unless an explicit no-PR closeout path is recorded
- [ ] Transient failures retried and documented when they affect proof truth
- [ ] Merge policy followed for the target branch

## Quality Gates
- [ ] Focused validation commands recorded for each touched surface
- [ ] Code-format/lint/test gates run where relevant to touched code
- [ ] CI is green on the merge target or exceptions are documented
- [ ] Coverage signal is not red where coverage applies, or exception documented (`<coverage_link_or_note>`)
- [ ] No unresolved high-priority blockers (`<blocker_report_link>`)

## Release Packaging
- [ ] Release notes finalized (`<release_notes_link>`)
- [ ] Tag verified: `<tag_name>`
- [ ] GitHub Release drafted (`<release_draft_link>`)
- [ ] Links validated in release body
- [ ] Release published

## Post-Release
- [ ] Milestone/epic issues closed with release links
- [ ] Deferred items moved to next milestone backlog
- [ ] Follow-up bugs/tech debt captured as issues
- [ ] Roadmap/status docs updated (`<roadmap_update_link>`)
- [ ] Retrospective summary recorded (`<retro_link>`)

## Exit Criteria
- All required gates are checked, or each exception has an owner + due date.
- Milestone can be audited end-to-end via the links captured above.
