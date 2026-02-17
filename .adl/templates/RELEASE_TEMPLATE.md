# Release Process Template

## Metadata
- Milestone: `{{milestone}}`
- Version: `{{version}}`
- Release date: `{{release_date}}`
- Release manager: `{{release_manager}}`

## How To Use
- Execute sections in order and capture links for each completed step.
- Keep this process doc focused on shipping mechanics; use release notes for narrative.
- Mark blockers immediately and do not publish until gates pass.

## 1) Release Readiness
- [ ] Milestone checklist is complete (`{{milestone_checklist_link}}`)
- [ ] Release notes draft is complete (`{{release_notes_link}}`)
- [ ] Final go/no-go decision recorded (`{{decision_link}}`)

## 2) Branch And Tag Preparation
- [ ] Target branch confirmed (`{{target_branch}}`)
- [ ] Version string(s) validated (`{{version_validation_link}}`)
- [ ] Tag prepared: `{{tag_name}}`
- [ ] Tag pushed and verified in remote

## 3) GitHub Release Steps
- [ ] Release draft created from `{{tag_name}}`
- [ ] Release body populated from approved notes
- [ ] Links to key PRs/issues included
- [ ] Release visibility confirmed (draft/prerelease/final)
- [ ] Release published

## 4) Verification
- [ ] Post-release CI status checked
- [ ] Release links tested (docs, artifacts, notes)
- [ ] Any immediate regressions triaged and tracked

## 5) Communication
- [ ] Community announcement published (`{{announcement_link}}`)
- [ ] Internal update posted (`{{internal_update_link}}`)
- [ ] Roadmap/status docs updated (`{{roadmap_update_link}}`)

## Exit Criteria
- Tag and GitHub Release are published and accessible.
- Release notes and communication links are complete.
- Post-release verification is complete with no unknown critical failures.
