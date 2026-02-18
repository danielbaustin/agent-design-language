# Release Process Template

## Metadata
- Milestone: `{{milestone}}`
- Version: `{{version}}`
- Release date: `{{release_date}}`
- Release manager: `{{release_manager}}`

## How To Use
- Execute sections in order and capture links for each completed step.
- Keep this doc focused on shipping mechanics; use release notes for narrative.
- Mark blockers immediately; do not publish until gates pass.

## 1) Release Readiness
- [ ] Milestone checklist complete (`{{milestone_checklist_link}}`)
- [ ] Release notes approved (`{{release_notes_link}}`)
- [ ] Go/no-go decision recorded (`{{decision_link}}`)

## 2) Branch And Tag Preparation
- [ ] Target branch confirmed (`{{target_branch}}`)
- [ ] Working tree clean
- [ ] Version string(s) validated (`{{version_validation_link}}`)
- [ ] Tag created: `{{tag_name}}`
- [ ] Tag pushed and verified

## 3) GitHub Release Steps
- [ ] GitHub Release draft created from `{{tag_name}}` (`{{release_draft_link}}`)
- [ ] Release body populated from approved notes
- [ ] Links to key PRs/issues included
- [ ] Release visibility confirmed (draft/prerelease/final)
- [ ] Release published

## 4) Verification
- [ ] Post-release CI status checked (`{{ci_run_link}}`)
- [ ] Release links tested (docs, artifacts, notes)
- [ ] Immediate regressions triaged and tracked (`{{triage_link}}`)

## 5) Communication
- [ ] Community announcement published (`{{announcement_link}}`)
- [ ] Internal update posted (`{{internal_update_link}}`)
- [ ] Roadmap/status updated (`{{roadmap_update_link}}`)

## Exit Criteria
- Tag and GitHub Release are published and accessible.
- Verification completed with no unknown critical failures.
- Communication links captured.
