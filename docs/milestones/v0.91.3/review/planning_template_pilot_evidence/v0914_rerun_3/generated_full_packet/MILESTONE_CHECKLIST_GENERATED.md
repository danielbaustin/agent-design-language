<!--
Generated Planning Draft
planning_template_set: 1.0.0
template: milestone_checklist
template_path: docs/templates/planning/1.0.0/milestone_checklist.md
generation_status: generated_draft
claim_boundary: generated draft only; not reviewed or approved
-->

> Generated planning draft. This file proves only template filling;
> it is not reviewed, approved, released, merged, or lifecycle-true.
# v0.91.4 Milestone Checklist

## Metadata
- Milestone: `v0.91.4`
- Version: `v0.91.4`
- Target release date: `TBD`
- Owner: `ADL planning-template pilot`

## Purpose
Ship/no-ship gate for the milestone. Check items only when evidence exists.

## Planning
- [ ] Milestone goal defined (`C-SDLC lifecycle proof and repeatability`)
- [ ] Scope + non-goals documented (`TBD`)
- [ ] WBS created and mapped to issues (`TBD`)
- [ ] Decision log initialized (`TBD`)
- [ ] Sprint plan created (`TBD`)

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
- [ ] Coverage signal is not red where coverage applies, or exception documented (`TBD`)
- [ ] No unresolved high-priority blockers (`TBD`)

## Release Packaging
- [ ] Release notes finalized (`TBD`)
- [ ] Tag verified: `v0.91.4`
- [ ] GitHub Release drafted (`TBD`)
- [ ] Links validated in release body
- [ ] Release published

## Post-Release
- [ ] Milestone/epic issues closed with release links
- [ ] Deferred items moved to next milestone backlog
- [ ] Follow-up bugs/tech debt captured as issues
- [ ] Roadmap/status docs updated (`TBD`)
- [ ] Retrospective summary recorded (`TBD`)

## Exit Criteria
- All required gates are checked, or each exception has an owner + due date.
- Milestone can be audited end-to-end via the links captured above.
