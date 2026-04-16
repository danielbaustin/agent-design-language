# Release Plan - v0.89.1

## Metadata
- Milestone: `v0.89.1`
- Version: `v0.89.1`
- Release date: `TBD`
- Release manager: `Daniel Austin`

## How To Use
- execute sections in order and capture links for each completed step
- keep this document focused on shipping mechanics; use release notes for narrative
- do not publish until the adversarial/runtime core, publication skill, demo program, and review surfaces are truthful

## Current Release Truth

`v0.89.1` is currently an active execution milestone with the adversarial/runtime proof band and integration-demo package landed through `WP-13`, `WP-14` owning the quality gate, and no release candidate yet.

That means:
- the canonical planning package exists and is reviewable
- the official issue wave is open and visible through `#1922` - `#1941`
- `WP-02` - `WP-13` are closed on the live tracker
- `WP-14` owns the quality-gate proof surface before `WP-15` docs/review
- no release candidate exists yet
- review, remediation, next-milestone planning, and ceremony work are all future steps

## 1) Release Readiness
- [ ] Milestone checklist complete (`MILESTONE_CHECKLIST_v0.89.1.md`)
- [ ] Release notes approved (`RELEASE_NOTES_v0.89.1.md`)
- [ ] Go/no-go decision recorded (`DECISIONS_v0.89.1.md` or final release issue)
- [x] Core implementation issue wave opened
- [x] Core adversarial/runtime and proof-entry wave landed through `WP-11`
- [x] Integration demos and three-paper manuscript packet complete or explicitly deferred in release truth
- [x] Quality gate complete (`QUALITY_GATE_v0.89.1.md` and `bash adl/tools/demo_v0891_quality_gate.sh`)

## 2) Branch And Tag Preparation
- [ ] Target branch confirmed (`main` unless explicitly changed)
- [ ] Working tree clean
- [ ] Version string(s) validated
- [ ] Tag created: `v0.89.1`
- [ ] Tag pushed and verified

## 3) GitHub Release Steps
- [ ] GitHub Release draft created from `v0.89.1`
- [ ] Release body populated from approved notes
- [ ] Links to key PRs/issues included
- [ ] Release visibility confirmed
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
- tag and GitHub Release are published and accessible
- verification completed with no unknown critical failures
- communication and roadmap updates are captured
