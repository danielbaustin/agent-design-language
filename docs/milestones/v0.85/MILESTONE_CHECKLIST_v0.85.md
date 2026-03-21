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
- [ ] Four-sprint, twenty-five-work-package structure reflected consistently across canonical milestone docs
- [ ] Each work package mapped to one canonical issue
- [ ] Each canonical issue mapped to one work package
- [ ] `#886` milestone-reorganization work completed or explicitly narrowed
- [ ] `#674` established as the canonical queue/checkpoint/steering issue and duplicate placeholder `#867` resolved
- [ ] Gödel issues `#748` through `#752` explicitly mapped into the milestone work-package structure

---

## Execution Discipline

- [ ] Each canonical issue has input/output cards under `.adl/cards/<issue>/` where required by the workflow
- [ ] Each burst writes artifacts under `.adl/reports/burst/<timestamp>/`
- [ ] Draft PR opened for each issue before merge
- [ ] Transient failures retried and documented
- [ ] "Green-only merge" policy followed

---

## Demo Proof Surfaces

- [ ] Steering/queueing/checkpoint demo exists
- [ ] HITL/editor/review workflow demo exists
- [ ] Gödel hypothesis-engine demo exists
- [ ] Affect-engine demo exists
- [ ] Affect-plus-Gödel/reasoning demo exists
- [ ] Demo matrix or playbook ties major new features to bounded runnable demos

---

## Quality Gates

- [ ] Canonical quality gate documented (`QUALITY_GATE_v0.85.md`)
- [ ] `cargo fmt` passes
- [ ] `cargo clippy --workspace --all-targets -- -D warnings` passes
- [ ] `cargo test --workspace` passes
- [ ] CI is green on the merge target
- [ ] Coverage signal is not red (or exception documented)
- [ ] No unresolved high-priority blockers remain

---

## Review Sequence

The milestone requires a clean review sequence before the release ceremony.

- [ ] Documentation consistency pass completed
- [ ] Internal review completed
- [ ] External review completed
- [ ] Review findings remediation completed or explicit deferrals recorded
- [ ] Final `swarm` -> `adl` cutover timing confirmed as end-of-milestone work (`SWARM_REMOVAL_PLANNING.md`)

---

## Release Packaging

- [ ] Release notes finalized (`RELEASE_NOTES_v0.85.md`)
- [ ] Final `swarm` -> `adl` active-surface cutover completed or explicitly deferred with rationale
- [ ] Tag verified: `v0.85.0`
- [ ] GitHub Release drafted
- [ ] Links validated in release body
- [ ] Release published

---

## Post-Release

- [ ] Milestone / epic issues closed with release links
- [ ] Deferred items moved to next milestone backlog
- [ ] Follow-up bugs / tech debt captured as issues
- [ ] Roadmap / status docs updated
- [ ] Next milestone planning materials prepared before final milestone closure
- [ ] Retrospective summary recorded

---

## Exit Criteria

The milestone is considered successfully shipped when:

- All required gates are checked, or
- Any remaining exceptions have a documented rationale, owner, and follow-up issue.

The milestone should be auditable end-to-end using the documents and artifacts referenced above.
