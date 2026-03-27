# Milestone Checklist — v0.85

## Metadata
- Milestone: `v0.85`
- Version: `0.85`
- Target release date: `TBD`
- Owner: `Daniel Austin / Agent Logic`

## Purpose
Ship / no-ship gate for the v0.85 milestone.

This checklist verifies that planning, implementation discipline, quality gates, demo proof surfaces, and release packaging steps have all been satisfied before the release ceremony.

Canonical quality-gate reference: `QUALITY_GATE_v0.85.md`

Evidence should exist for every checked item.

---

## Planning

- [x] Milestone goal defined (`DESIGN_v0.85.md`)
- [x] Scope + non-goals documented (`DESIGN_v0.85.md`)
- [x] WBS created and mapped to milestone work (`WBS_v0.85.md`)
- [x] Decision log initialized (`DECISIONS_v0.85.md`)
- [x] Sprint plan created (`SPRINT_v0.85.md`)
- [x] Four-sprint, twenty-five-work-package structure reflected consistently across canonical milestone docs
- [x] Each work package mapped to one canonical issue
- [x] Each canonical issue mapped to one work package
- [x] `#886` milestone-reorganization work completed or explicitly narrowed
- [x] `#674` established as the canonical queue/checkpoint/steering issue and duplicate placeholder `#867` resolved
- [x] Gödel issues `#748` through `#752` explicitly mapped into the milestone work-package structure

---

## Execution Discipline

- [x] Each canonical issue has input/output cards under `.adl/cards/<issue>/` where required by the workflow
- [x] Each burst writes artifacts under `.adl/reports/burst/<timestamp>/`
- [x] Draft PR opened for each issue before merge
- [x] Transient failures retried and documented
- [x] "Green-only merge" policy followed

---

## Demo Proof Surfaces

- [x] Steering/queueing/checkpoint demo exists
- [x] HITL/editor/review workflow demo exists
- [x] Five-command editing lifecycle proof surface exists
- [x] Gödel hypothesis-engine demo exists
- [x] Affect-engine demo exists
- [x] Affect-plus-Gödel/reasoning demo exists
- [x] Demo matrix or playbook ties major new features to bounded runnable demos

---

## Quality Gates

- [x] Canonical quality gate documented (`QUALITY_GATE_v0.85.md`)
- [x] `cargo fmt` passes
- [x] `cargo clippy --workspace --all-targets -- -D warnings` passes
- [x] `cargo test --workspace` passes
- [x] CI is green on the merge target
- [x] Coverage signal is not red (or exception documented)
- [x] No unresolved high-priority blockers remain

---

## Review Sequence

The milestone requires a clean review sequence before the release ceremony.

- [x] Documentation consistency pass completed
- [x] Internal review completed
- [x] External review completed
- [x] Review findings remediation completed or explicit deferrals recorded
- [x] Final `swarm` -> `adl` cutover timing confirmed as end-of-milestone work (`SWARM_REMOVAL_PLANNING.md`)

---

## Release Packaging

- [x] Release notes finalized (`RELEASE_NOTES_v0.85.md`)
- [x] Final `swarm` -> `adl` active-surface cutover completed or explicitly deferred with rationale
- [x] Tag verified: `v0.85.0`
- [x] GitHub Release drafted
- [x] Links validated in release body
- [x] Release published

---

## Post-Release

- [x] Milestone / epic issues closed with release links
- [x] Deferred items moved to next milestone backlog
- [x] Follow-up bugs / tech debt captured as issues
- [x] Roadmap / status docs updated
- [x] Next milestone planning materials prepared before final milestone closure
- [x] Retrospective summary recorded

---

## Exit Criteria

The milestone is considered successfully shipped when:

- All required gates are checked, or
- Any remaining exceptions have a documented rationale, owner, and follow-up issue.

The milestone should be auditable end-to-end using the documents and artifacts referenced above.
