# Release Plan - v0.89.1

## Metadata
- Milestone: `v0.89.1`
- Version: `v0.89.1`
- Release date: `2026-04-17`
- Release manager: `Daniel Austin`

## How To Use
- execute sections in order and capture links for each completed step
- keep this document focused on shipping mechanics; use release notes for narrative
- do not publish until the adversarial/runtime core, publication skill, demo program, and review surfaces are truthful

## Current Release Truth

`v0.89.1` is complete. The adversarial/runtime proof band, integration-demo package, quality gate, docs-review convergence pass, internal review record, internal-review remediation issues, third-party review, next-milestone planning handoff, and release ceremony have landed.

That means:
- the canonical planning package exists and is reviewable
- the official issue wave is open and visible through `#1922` - `#1941`
- `WP-02` - `WP-16` are closed or represented by tracked release-tail proof surfaces
- `WP-16` owns the internal review surface
- internal-review remediation is closed through `#1992`
- `#1999` prepared the third-party review handoff before `WP-17`
- `WP-17` third-party review is closed with no additional P0/P1/P2 findings
- `WP-18` review remediation is closed with internal review fixes recorded and F8 deferred to v0.90 maintainability work
- `WP-19` promoted the v0.90 planning package before ceremony
- `WP-20` completed release ceremony
- release tag and GitHub release are `v0.89.1`

## 1) Release Readiness
- [x] Milestone checklist complete (`MILESTONE_CHECKLIST_v0.89.1.md`)
- [x] Release notes approved (`RELEASE_NOTES_v0.89.1.md`)
- [x] Go/no-go decision recorded in `#1941`
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
- [x] Operator fast-forwarded root main after the handoff merge and confirmed no local tracked drift

## 2) Branch And Tag Preparation
- [x] Target branch confirmed (`main`)
- [x] Working tree clean
- [x] Version string(s) validated
- [x] Tag created: `v0.89.1`
- [x] Tag pushed and verified

## 3) GitHub Release Steps
- [x] GitHub Release draft created from `v0.89.1`
- [x] Release body populated from approved notes
- [x] Links to key PRs/issues included
- [x] Release visibility confirmed
- [x] Release published

## 4) Verification
- [x] Post-release CI status checked
- [x] Release links tested (docs, artifacts, notes)
- [x] Immediate regressions triaged and tracked; none known at release

## 5) Communication
- [x] Community announcement deferred outside repository release mechanics
- [x] Internal update captured by release issue closeout
- [x] Roadmap/status updated for v0.90 handoff

## Exit Criteria
- tag and GitHub Release are published and accessible
- verification completed with no unknown critical failures
- communication and roadmap updates are captured
