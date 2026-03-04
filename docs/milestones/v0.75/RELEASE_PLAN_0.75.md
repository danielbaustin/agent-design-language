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

# Release Plan — v0.75

## Metadata
- Milestone: **v0.75**
- Version: **v0.75.0**
- Target release date: **TBD (target: end of next week)**
- Release manager: **Daniel Austin**

## Purpose
v0.75 is the **interstitial stabilization milestone** between v0.7 and v0.8.

Primary intent:
- Ship **EPIC-A + EPIC-B** (Deterministic Substrate + ObsMem v1 integration boundary)
- Keep changes **reviewable** (small, auditable PRs)
- Preserve the v0.7 compatibility window (canonical `adl` naming)

Non-goals:
- Cluster / distributed execution (target v0.85+)
- Advanced adaptive policy + online learning v2 (tracked separately)

## Release Gates
A release candidate is eligible to ship only if all gates are green:

### Quality gates
- [ ] CI required checks green on `main`
- [ ] `cargo test --workspace` passes (workspace rooted at `swarm/`)
- [ ] `cargo fmt --check` passes
- [ ] `cargo clippy --workspace --all-targets -- -D warnings` passes
- [ ] Coverage floor maintained (90% target)

### Docs + demo gates
- [ ] v0.75 milestone docs complete (VISION/DESIGN/WBS/SPRINT/DECISIONS/MILESTONE_CHECKLIST/RELEASE_NOTES)
- [ ] Canonical planning docs are under `.adl/docs/v075planning/` (no duplicate sources of truth)
- [ ] Demos referenced in milestone docs are runnable and verified

### Security + determinism gates
- [ ] No regression in signing / verification behavior (remote execution envelope)
- [ ] Determinism expectations documented for any nondeterministic surfaces
- [ ] No secrets exported by default in any learning/export artifacts

### Review gates
- [ ] Internal review pass completed and findings triaged
- [ ] **3rd party review completed** and findings triaged (required)

## 0) Pre-flight
- [ ] Confirm milestone scope is up to date (issues + WBS)
- [ ] Confirm release manager and reviewers
- [ ] Ensure local dev environment is clean (toolchain, rustfmt, clippy)
- [ ] Confirm repo is fast-forwarded to `origin/main`

Links:
- Milestone checklist: TODO
- Sprint plan: TODO
- WBS: TODO

## 1) Release Readiness
- [ ] Milestone checklist complete (link: TODO)
- [ ] Release notes draft complete (link: TODO)
- [ ] Go/no-go decision recorded in DECISIONS doc (link: TODO)

## 2) Branch and Tag Preparation
- [ ] Target branch confirmed: `main`
- [ ] Working tree clean (no local changes)
- [ ] Version string(s) validated (link: TODO)
- [ ] Tag created: `v0.75.0`
- [ ] Tag pushed and verified on GitHub

## 3) GitHub Release Steps
- [ ] GitHub Release draft created from tag `v0.75.0` (link: TODO)
- [ ] Release body populated from approved v0.75 release notes
- [ ] Links to key PRs/issues included (at minimum: EPIC-A/B deliverables + major fixes)
- [ ] Release visibility confirmed (draft/prerelease/final)
- [ ] Release published

## 4) Verification
- [ ] Post-release CI status checked (link: TODO)
- [ ] Release links tested (docs, artifacts, notes)
- [ ] Immediate regressions triaged and tracked (link: TODO)

## 5) Communication
- [ ] Community announcement published (link: TODO)
- [ ] Internal update posted (link: TODO)
- [ ] Roadmap/status updated (link: TODO)

## Exit Criteria
- Tag `v0.75.0` and GitHub Release are published and accessible.
- All gates are green with no unknown critical failures.
- Review links (internal + 3rd party) and triage links are captured.

---

## Appendix A — Canonical v0.75 Docs Inventory
Canonical planning + milestone docs must live in `.adl/docs/v075planning/`:

- VISION_0.75.md
- DESIGN_0.75.md
- WBS_0.75.md
- SPRINT_0.75.md
- DECISIONS_0.75.md
- MILESTONE_CHECKLIST_0.75.md
- RELEASE_NOTES_0.75.md
- RELEASE_PLAN_0.75.md (this file)

Notes:
- `docs/milestones/` may include pointer READMEs, but must not duplicate canonical planning docs.