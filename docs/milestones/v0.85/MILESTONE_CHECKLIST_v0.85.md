# Milestone Checklist — v0.85

## Metadata
- Milestone: `v0.85`
- Version: `0.85`
- Target release date: `TBD`
- Owner: `Daniel Austin / Agent Logic`

## Purpose
Ship / no‑ship gate for the v0.85 milestone.

This checklist verifies that planning, implementation discipline, quality gates, and release packaging steps have all been satisfied before the release ceremony.

Evidence should exist for every checked item.

---

## Planning

- [x] Milestone goal defined (`DESIGN_v0.85.md`)
- [x] Scope + non‑goals documented (`DESIGN_v0.85.md`)
- [x] WBS created and mapped to milestone work (`WBS_v0.85.md`)
- [x] Decision log initialized (`DECISIONS_v0.85.md`)
- [x] Sprint plan created (`SPRINT_v0.85.md`)

---

## Execution Discipline

- [ ] Each issue has input/output cards under `.adl/cards/<issue>/`
- [ ] Each burst writes artifacts under `.adl/reports/burst/<timestamp>/`
- [ ] Draft PR opened for each issue before merge
- [ ] Transient failures retried and documented
- [ ] "Green‑only merge" policy followed

---

## Quality Gates

- [ ] `cargo fmt` passes
- [ ] `cargo clippy --workspace --all-targets -- -D warnings` passes
- [ ] `cargo test --workspace` passes
- [ ] CI is green on the merge target
- [ ] Coverage signal is not red (or exception documented)
- [ ] No unresolved high‑priority blockers remain

---

## Review Sequence

The milestone requires both internal and external review before the release ceremony.

- [ ] Milestone docs made internally consistent
- [ ] Internal review completed
- [ ] External review completed
- [ ] Review findings resolved or explicitly deferred

---

## Release Packaging

- [ ] Release notes finalized (`RELEASE_NOTES_v0.85.md`)
- [ ] Tag verified: `v0.85.0`
- [ ] GitHub Release drafted
- [ ] Links validated in release body
- [ ] Release published

---

## Post‑Release

- [ ] Milestone / epic issues closed with release links
- [ ] Deferred items moved to next milestone backlog
- [ ] Follow‑up bugs / tech debt captured as issues
- [ ] Roadmap / status docs updated
- [ ] Retrospective summary recorded

---

## Exit Criteria

The milestone is considered successfully shipped when:

- All required gates are checked, **or**
- Any remaining exceptions have a documented rationale, owner, and follow‑up issue.

The milestone should be auditable end‑to‑end using the documents and artifacts referenced above.
