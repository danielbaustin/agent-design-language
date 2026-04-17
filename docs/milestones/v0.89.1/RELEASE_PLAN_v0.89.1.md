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

`v0.89.1` is currently an active release-tail milestone with the adversarial/runtime proof band, integration-demo package, quality gate, docs-review convergence pass, internal review record, internal-review remediation issues, third-party review, and next-milestone planning handoff landed or in final promotion. There is no release candidate yet.

That means:
- the canonical planning package exists and is reviewable
- the official issue wave is open and visible through `#1922` - `#1941`
- `WP-02` - `WP-16` are closed or represented by tracked release-tail proof surfaces
- `WP-16` owns the internal review surface
- internal-review remediation is closed through `#1992`
- `#1999` prepared the third-party review handoff before `WP-17`
- `WP-17` third-party review is closed with no additional P0/P1/P2 findings
- `WP-18` review remediation is closed with internal review fixes recorded and F8 deferred to v0.90 maintainability work
- `WP-19` owns the v0.90 planning package promotion before ceremony
- no release candidate exists yet
- release ceremony remains the final release-tail step

## 1) Release Readiness
- [ ] Milestone checklist complete (`MILESTONE_CHECKLIST_v0.89.1.md`)
- [ ] Release notes approved (`RELEASE_NOTES_v0.89.1.md`)
- [ ] Go/no-go decision recorded (`DECISIONS_v0.89.1.md` or final release issue)
- [x] Core implementation issue wave opened
- [x] Core adversarial/runtime and proof-entry wave landed through `WP-11`
- [x] Integration demos and three-paper manuscript packet complete or explicitly deferred in release truth
- [x] Quality gate complete (`QUALITY_GATE_v0.89.1.md` and `bash adl/tools/demo_v0891_quality_gate.sh`)
- [x] Docs-review convergence complete (`DOCS_REVIEW_v0.89.1.md`)
- [x] Internal review complete (`INTERNAL_REVIEW_v0.89.1.md`)
- [x] Internal-review remediation closed through `#1992`
- [x] Third-party review handoff prepared in the local review store
- [x] Third-party review completed with no additional P0/P1/P2 findings
- [x] Review remediation closed with internal review fixes recorded and F8 deferred
- [x] Next-milestone planning package promoted by `WP-19`
- [ ] Operator fast-forwarded root main after the handoff merge and confirmed no local tracked drift

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
