# ADL v0.7 Release Plan

## Metadata
- Milestone: `v0.7`
- Version: `v0.7.x (release train)`
- Target release: `v0.7.0` (foundation)
- Target branch: `main`
- Release manager: Daniel Austin
- Timezone: America/Los_Angeles

---

## Purpose

This plan defines the mechanics to ship ADL v0.7, including:
- a stable **v0.7.0 foundation release**, and
- an incremental **v0.7.x learning train**.

It is intentionally procedural. Narrative belongs in `RELEASE_NOTES_v0.7.md`.

---

## Release Taxonomy

- **EPIC-\***: umbrella / grouping only.
- **WP-\***: executable work packages (drive PR/worktree flow via `pr.sh`).
- **Task**: smaller issues pulled into WPs as dependencies/sub-issues.

---

## Release Gates (Go/No-Go)

Hard gates:
- No placeholder tokens under `docs/milestones/v0.7/`.
- CI green on merge target.
- `cargo fmt`, `cargo clippy -D warnings`, `cargo test` all pass.
- Security envelope + signing gates complete before enabling any learning features.

Evidence commands:

```bash
rg -n "\{\{.*\}\}" docs/milestones/v0.7
cargo fmt --all
cargo clippy --all-targets -- -D warnings
cargo test
```

---

## Phase A — Foundation Release (v0.7.0)

### A1) Complete Foundation Work

Target completion state:
- EPIC-E Security envelope + trust hardening: #429
  - Tasks: #472, #370, #371, #386
- EPIC-B Delegation runtime: #413
- Scheduler policy surface task: #369
- EPIC-F Resilience surfaces: #430
- Canonical execution path cleanup task: #383
- EPIC-D Cleanup: #415

### A2) Foundation Demo + Quality Convergence

Run tail work in parallel, converge before tagging:
- WP-13 Demos: #474 (EPIC-G #478)
- WP-14 Coverage/quality gate: #475 (EPIC-G #478)
- WP-15 Docs/review: #476 (EPIC-G #478)

### A3) Tag and Publish v0.7.0

1) Confirm working tree clean and on `main`.
2) Ensure milestone checklist items relevant to v0.7.0 are checked:
   - `docs/milestones/v0.7/MILESTONE_CHECKLIST_v0.7.md`
3) Finalize v0.7.0 release notes section in `RELEASE_NOTES_v0.7.md`.
4) Tag and publish (performed in WP-16): #477 (EPIC-G #478).

---

## Phase B — Learning Train (v0.7.1+)

v0.7.x minors deliver learning incrementally, overlay-based and opt-in:

- EPIC-C Learning surfaces (ObsMem deferred to v0.8): #414
- EPIC-A Dynamic learning train: #412

Constraints (must remain true for every v0.7.x minor):
- No workflow YAML mutation.
- No silent auto-promotion.
- Versioned, schema-validated artifacts (`deny_unknown_fields`).
- Learning surfaces must remain independent of ObsMem (v0.8).

Each minor release repeats:
- update `RELEASE_NOTES_v0.7.md` (new sub-section)
- run demos relevant to the new learning feature(s)
- ensure security/trust constraints still hold

---

## Phase C — High-Churn Rename (late v0.7)

- WP-12 rename runs late and is expected to cause broad churn:
  - WP-12: #336 (EPIC-H #479)

Decision lock (see `DECISIONS_v0.7.md` D-06):
- Rename crate/package + binaries to `adl`.
- Keep the `swarm/` directory path stable in v0.7.
- Provide one-release compatibility shims (`swarm`/`swarm-remote`) with deprecation warnings.

After WP-12 merges, re-run:
- WP-13 demos (sanity)
- WP-14 quality gate (re-check coverage and test harness)
- WP-15 docs/review (migration notes, command updates)

---

## Final Release Ceremony (WP-16)

WP-16 owns the release ceremony steps:
- #477 (EPIC-G #478)

Checklist:
- [ ] All required gates in `MILESTONE_CHECKLIST_v0.7.md` are checked (or exceptions recorded)
- [ ] `RELEASE_NOTES_v0.7.md` finalized
- [ ] Create tag for the target release
- [ ] Create GitHub Release from tag and publish
- [ ] Verify docs links and demo commands
- [ ] Close milestone and prune completed issues

---

## Reference Documents

- Design: `docs/milestones/v0.7/DESIGN_v0.7.md`
- WBS: `docs/milestones/v0.7/WBS_v0.7.md`
- Sprint plan: `docs/milestones/v0.7/SPRINT_v0.7.md`
- Decisions: `docs/milestones/v0.7/DECISIONS_v0.7.md`
- Milestone checklist: `docs/milestones/v0.7/MILESTONE_CHECKLIST_v0.7.md`
- Release notes: `docs/milestones/v0.7/RELEASE_NOTES_v0.7.md`
