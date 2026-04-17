# Milestone Checklist - v0.89.1

## Metadata
- Milestone: `v0.89.1`
- Version: `v0.89.1`
- Release date: `2026-04-17`
- Owner: `Daniel Austin`

## Purpose

Ship/no-ship gate for `v0.89.1`. Check items only when evidence exists.

## Planning
- [x] Milestone goal defined (`README.md`, `VISION_v0.89.1.md`)
- [x] Scope + non-goals documented (`DESIGN_v0.89.1.md`)
- [x] WBS created and ready for later issue-wave seeding (`WBS_v0.89.1.md`)
- [x] Decision log initialized (`DECISIONS_v0.89.1.md`)
- [x] Sprint plan created (`SPRINT_v0.89.1.md`)
- [x] Feature index created with explicit carry-forward mapping (`FEATURE_DOCS_v0.89.1.md`)
- [x] Official issue-wave plan drafted and review-ready (`WP_ISSUE_WAVE_v0.89.1.yaml`)
- [x] Official `v0.89.1` issue wave opened

## Execution Discipline
- [x] Each issue has truthful local input/output cards in the ADL control plane
- [x] Draft PR opened for each tracked issue before merge where applicable
- [x] Queue and closeout discipline followed consistently
- [x] The `v0.89` / `v0.89.1` boundary remains explicit rather than blurred by carry-back scope
- [x] Green-only merge policy followed
- [x] Docs-review convergence surface exists (`DOCS_REVIEW_v0.89.1.md`)
- [x] Internal review surface exists (`INTERNAL_REVIEW_v0.89.1.md`)
- [x] Internal-review remediation issues are closed through `#1992`
- [x] Third-party review handoff is prepared in the local review store
- [x] Third-party review completed with no additional P0/P1/P2 findings
- [x] Review remediation closed with internal review fixes recorded and F8 deferred to v0.90
- [x] Next-milestone planning package promoted by `WP-19`
- [x] Root main has been fast-forwarded after the handoff merge with no local tracked drift

## Quality Gates
- [x] `cargo fmt` passes
- [x] `cargo clippy --all-targets -- -D warnings` passes
- [x] `cargo test` passes
- [x] CI is green on the merge target
- [x] Coverage signal is not red (or exception documented)
- [x] Quality gate documented (`QUALITY_GATE_v0.89.1.md`)
- [x] D10 quality-gate walkthrough passes (`bash adl/tools/demo_v0891_quality_gate.sh`)
- [x] No unresolved high-priority blockers remain at release time

## Release Packaging
- [x] Release notes finalized (`RELEASE_NOTES_v0.89.1.md`)
- [x] Tag verified: `v0.89.1`
- [x] GitHub Release drafted
- [x] Links validated in release body
- [x] Release published

## Post-Release
- [x] Milestone issues closed with release links
- [x] Deferred items moved to next milestone backlog / planning surface
- [x] Follow-up bugs and debt captured explicitly
- [x] Roadmap/status docs updated
- [x] Retrospective summary recorded in release issue closeout

## Current Planning Truth

At the moment this checklist should be read as:
- planning-complete and review-ready
- issue wave opened; `WP-02` - `WP-19` landed, closed, or represented by tracked release-tail proof surfaces
- internal-review remediation is closed through `#1992`
- third-party review closed with no additional P0/P1/P2 findings
- review remediation closed with internal review fixes recorded and F8 deferred to v0.90
- release ceremony is complete; v0.90 planning is ready for the next issue wave

## Exit Criteria
- all required gates are checked, or each exception has an owner and explicit rationale
- the milestone can be audited end to end via the linked docs, issues, PRs, and proof surfaces
