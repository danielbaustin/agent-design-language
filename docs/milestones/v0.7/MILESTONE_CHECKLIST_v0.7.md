# ADL v0.7 Milestone Checklist

## Metadata
- Milestone: `v0.7`
- Version: `v0.7.x (release train)`
- Target release date: TBD (set during WP-16)
- Owner: Daniel Austin

## Purpose
Ship/no-ship gate for the milestone. Check items only when evidence exists (commands, links, PRs, CI runs).

---

## Planning (WP-01)

- [ ] Milestone goal defined (Design): `docs/milestones/v0.7/DESIGN_v0.7.md`
- [ ] Scope + non-goals documented (Design): `docs/milestones/v0.7/DESIGN_v0.7.md`
- [ ] WBS created and mapped to issues: `docs/milestones/v0.7/WBS_v0.7.md`
- [ ] Decision log initialized: `docs/milestones/v0.7/DECISIONS_v0.7.md`
- [ ] Sprint plan created: `docs/milestones/v0.7/SPRINT_v0.7.md`
- [ ] Rename planning doc recorded (for WP-12): `docs/milestones/v0.7/SWARM_NAME_CHANGE_PLANNING_v0.7.md`
- [ ] No placeholder tokens remain in v0.7 docs:
  - Evidence: `rg -n "\{\{.*\}\}" docs/milestones/v0.7` returns no matches

---

## Execution Discipline

- [ ] Each executed issue has input/output cards under `.adl/cards/<issue>/`
- [ ] Each burst writes artifacts under `.adl/reports/burst/<timestamp>/` (if used)
- [ ] Draft PR opened for each issue before merge
- [ ] Transient failures retried and documented in the output card
- [ ] "Green-only merge" policy followed (CI green on merge target)

---

## Foundation Release Gates (v0.7.0)

Security / trust / sandbox
- [ ] Security envelope hardening complete (EPIC-E): #429
- [ ] Sandbox symlink escape prevention complete: #472
- [ ] Remote signing enforcement implemented: #370
- [ ] Trust policy requirements implemented: #371
- [ ] Remote request signing task complete: #386

Runtime semantics
- [ ] Delegation runtime complete (EPIC-B): #413
- [ ] Scheduler policy surface complete: #369
- [ ] Runtime resilience surfaces complete (EPIC-F): #430
- [ ] Canonical execution path cleanup complete: #383
- [ ] Cleanup / deferred systems work complete (EPIC-D): #415

---

## Learning Train Gates (v0.7.x)

- [ ] Learning surfaces implemented and stable (EPIC-C): #414
- [ ] Dynamic learning observe/score/suggest delivered (EPIC-A): #412
- [ ] Overlay apply/export delivered (EPIC-A): #412

Hard constraints (must hold for all learning features)
- [ ] Overlay-based only; no workflow YAML mutation
- [ ] Opt-in only; no silent auto-promotion
- [ ] Artifacts are versioned and schema-validated (`deny_unknown_fields`)
- [ ] Learning surfaces independent of ObsMem (ObsMem deferred to v0.8)

---

## Rename / Identity Migration (late v0.7)

- [ ] Runtime identity rename executed late (EPIC-H / WP-12): #479 / #336
- [ ] `adl` is the canonical crate/package + binary name
- [ ] Compatibility window present (legacy `swarm`/`swarm-remote` shims) with deprecation warning
- [ ] `swarm/` directory path remains stable in v0.7 (no directory rename)
- [ ] Migration notes included in docs / release notes

---

## Release Tail (parallelizable; converge before release)

- [ ] Demo matrix + integration demos complete (EPIC-G / WP-13): #478 / #474
- [ ] Coverage / quality gate complete (EPIC-G / WP-14): #478 / #475
- [ ] Docs + review pass complete (EPIC-G / WP-15): #478 / #476
- [ ] Release ceremony complete (EPIC-G / WP-16): #478 / #477

---

## Quality Gates

- [ ] `cargo fmt --all` passes
- [ ] `cargo clippy --all-targets -- -D warnings` passes
- [ ] `cargo test` passes
- [ ] CI is green on the merge target
- [ ] Coverage signal is not red (or exception documented) — see WP-14 (#475)
- [ ] No unresolved high-priority blockers (P0/P1) remain for v0.7

---

## Release Packaging

- [ ] Release plan finalized: `docs/milestones/v0.7/RELEASE_PLAN_v0.7.md`
- [ ] Release notes finalized: `docs/milestones/v0.7/RELEASE_NOTES_v0.7.md`
- [ ] Tag verified (set during WP-16)
- [ ] GitHub Release drafted (set during WP-16)
- [ ] Links validated in release body
- [ ] Release published

---

## Post-Release

- [ ] Milestone/epic issues closed with release links
- [ ] Deferred items moved to next milestone backlog (notably v0.8: ObsMem, cluster execution, durable checkpoint engine)
- [ ] Follow-up bugs/tech debt captured as issues
- [ ] Roadmap/status docs updated (as part of WP-16)
- [ ] Retrospective summary recorded (as part of WP-16)

---

## Exit Criteria

- All required gates are checked, or each exception has an owner + due date.
- Milestone can be audited end-to-end via the links captured above.
