<!--
Generated Planning Draft
planning_template_set: 1.0.0
template: release_plan
template_path: docs/templates/planning/1.0.0/release_plan.md
generation_status: generated_draft
claim_boundary: generated draft only; not reviewed or approved
-->

> Generated planning draft. This file proves only template filling;
> it is not reviewed, approved, released, merged, or lifecycle-true.
# v0.91.4 Release Plan

## Metadata
- Milestone: `v0.91.4`
- Version: `v0.91.4`
- Release date: `TBD`
- Release manager: `TBD`

## How To Use
- Execute sections in order and capture links for each completed step.
- Keep this doc focused on shipping mechanics; use release notes for narrative.
- Mark blockers immediately; do not publish until gates pass.
- Ceremony is a confirmation and publication phase, not a hidden implementation
  phase.

## 0. Release-Tail Convergence
- [ ] Live trackers refreshed and reflected honestly:
  - coverage/test tracker
  - Rust module watch / refactoring tracker
  - any active milestone-specific gap/risk tracker
- [ ] Gap analysis refreshed or explicitly confirmed still current
- [ ] Closed-issue closeout pass refreshed so issue/card truth is not stale
  - capture a merged-needs-closeout visibility report before apply mode when useful
- [ ] Release-truth docs aligned:
  - `README.md`
  - `CHANGELOG.md`
  - `Cargo.toml`
  - `REVIEW.md`
  - feature list
  - milestone checklist
  - release plan
  - release notes
- [ ] Review artifacts collected and review disposition reflected truthfully
- [ ] End-of-milestone report written or refreshed (`TBD`)
- [ ] Next-milestone handoff prepared before ceremony starts
- [ ] Any remaining work is either landed, explicitly deferred, or routed

## 1. Release Readiness
- [ ] Milestone checklist complete (`TBD`)
- [ ] Release notes approved (`TBD`)
- [ ] Go/no-go decision recorded (`Use generated planning drafts only after review.`)

## 2. Branch And Tag Preparation
- [ ] Target branch confirmed (`main`)
- [ ] Working tree clean
- [ ] Version string(s) validated (`TBD`)
- [ ] Tag created: `v0.91.4`
- [ ] Tag pushed and verified

## 3. GitHub Release Steps
- [ ] GitHub Release draft created from `v0.91.4` (`TBD`)
- [ ] Release body populated from approved notes
- [ ] Links to key PRs/issues included
- [ ] Release visibility confirmed (draft/prerelease/final)
- [ ] Release published

## 4. Verification
- [ ] Post-release CI status checked (`TBD`)
- [ ] Release links tested (docs, artifacts, notes)
- [ ] Immediate regressions triaged and tracked (`TBD`)

## 5. Communication
- [ ] Community announcement published (`TBD`)
- [ ] Internal update posted (`TBD`)
- [ ] Roadmap/status updated (`TBD`)

## Exit Criteria
- No hidden implementation or unresolved truth-maintenance work remains in the ceremony phase.
- Tag and GitHub Release are published and accessible.
- Verification completed with no unknown critical failures.
- Communication links captured.
