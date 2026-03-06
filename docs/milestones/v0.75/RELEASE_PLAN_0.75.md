# Release Plan — v0.75

## Metadata
- Milestone: **v0.75**
- Version: **v0.75.0**
- Target release date: **TBD (target: end of next week)**
- Release manager: **Daniel Austin**

## Purpose
v0.75 is the interstitial stabilization milestone between v0.7 and v0.8.

Primary intent:
- Ship **EPIC-A + EPIC-B** (Deterministic Substrate + ObsMem v1 integration boundary)
- Keep changes reviewable (small, auditable PRs)
- Preserve the v0.7 compatibility window (canonical `adl` naming)

Non-goals:
- Cluster / distributed execution (target v0.85+)
- Advanced adaptive policy + online learning v2 (tracked separately)

## Release Gates
A release candidate is eligible to ship only if all gates are green.

### Quality gates
- [ ] CI required checks green on `main`
- [ ] `cargo test --workspace` passes (run from `./swarm/`)
- [ ] `cargo fmt --all -- --check` passes
- [ ] `cargo clippy --workspace --all-targets -- -D warnings` passes
- [ ] Coverage policy gate passes (workspace >= 90%, per-file runtime >= 80% with documented exclusions)

### Docs + demo gates
- [ ] v0.75 milestone docs complete and cross-linked
- [ ] Demo matrix commands are runnable from docs (`docs/milestones/v0.75/DEMO_MATRIX.md`)
- [ ] Coverage policy doc aligned with CI/nightly configuration (`docs/milestones/v0.75/COVERAGE_POLICY_0.75.md`)

### Security + determinism gates
- [ ] No regression in signing / verification behavior (remote execution envelope)
- [ ] Determinism expectations documented for any nondeterministic surfaces
- [ ] No secrets exported by default in learning/export artifacts

### Review gates
- [ ] Internal review pass completed and findings triaged
- [ ] 3rd-party review completed and findings triaged

## 0) Pre-flight
- [ ] Confirm milestone scope is up to date (issues + WBS)
- [ ] Confirm release manager and reviewers
- [ ] Ensure local dev environment is clean (toolchain, rustfmt, clippy)
- [ ] Confirm branch is fast-forwarded to `origin/main`

Canonical references:
- Milestone checklist: `docs/milestones/v0.75/MILESTONE_CHECKLIST_0.75.md`
- Sprint plan: `docs/milestones/v0.75/SPRINT_0.75.md`
- WBS: `docs/milestones/v0.75/WBS_0.75.md`
- Release notes: `docs/milestones/v0.75/RELEASE_NOTES_0.75.md`
- Design: `docs/milestones/v0.75/DESIGN_0.75.md`

## 1) Release Readiness
- [ ] Milestone checklist complete
- [ ] Release notes draft complete
- [ ] Go/no-go decision recorded in DECISIONS doc

## 2) Branch and Tag Preparation
- [ ] Target branch confirmed: `main`
- [ ] Working tree clean (no local changes)
- [ ] Version string(s) validated
- [ ] Tag created: `v0.75.0`
- [ ] Tag pushed and verified on GitHub

## 3) GitHub Release Steps
- [ ] GitHub Release draft created from tag `v0.75.0`
- [ ] Release body populated from approved v0.75 release notes
- [ ] Links to key PRs/issues included (at minimum: EPIC-A/B deliverables + major fixes)
- [ ] Release visibility confirmed (draft/prerelease/final)
- [ ] Release published

## 4) Verification
- [ ] Post-release CI status checked
- [ ] Release links tested (docs, artifacts, notes)
- [ ] Immediate regressions triaged and tracked

## 5) Communication
- [ ] Community announcement published
- [ ] Internal update posted
- [ ] Roadmap/status updated

## Exit Criteria
- Tag `v0.75.0` and GitHub Release are published and accessible.
- All gates are green with no unknown critical failures.
- Review and triage links are captured in release notes/decision records.

---

## Appendix A — Canonical v0.75 Docs Inventory

- `docs/milestones/v0.75/VISION_0.75.md`
- `docs/milestones/v0.75/DESIGN_0.75.md`
- `docs/milestones/v0.75/WBS_0.75.md`
- `docs/milestones/v0.75/SPRINT_0.75.md`
- `docs/milestones/v0.75/DECISIONS_0.75.md`
- `docs/milestones/v0.75/MILESTONE_CHECKLIST_0.75.md`
- `docs/milestones/v0.75/RELEASE_NOTES_0.75.md`
- `docs/milestones/v0.75/RELEASE_PLAN_0.75.md`
- `docs/milestones/v0.75/DEMO_MATRIX.md`
- `docs/milestones/v0.75/COVERAGE_POLICY_0.75.md`
