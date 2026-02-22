# Milestone Checklist — v0.6

## Metadata
- Milestone: v0.6
- Version: v0.6
- Target release: TBD (set at RC freeze when WP-I enters final review)
- Owner: ADL core (Daniel + Codex-assisted implementation)

---

## Purpose

This checklist is the ship / no-ship gate for ADL v0.6.

v0.6 is a **stabilize + formalize** release.  
It must:

- Preserve determinism invariants.
- Introduce structured runtime surface (streaming, HITL, delegation metadata).
- Harden scheduler and policy surfaces.
- Establish coverage ratchet discipline.
- Ship clean documentation aligned with WPs #401–#411.

Items are checked only when objective evidence exists (PRs, CI runs, artifacts).

---

# Planning Integrity

- [x] DESIGN_v0.6.md finalized and aligned with WPs #401–#411
- [x] WBS_v0.6.md maps work to umbrella WPs (#401–#411)
- [x] SPRINT_v0.6.md defines phased execution order
- [x] DECISIONS_v0.6.md records architectural boundaries
- [ ] RELEASE_PLAN_v0.6.md finalized (entry/exit criteria + ceremony)
- [ ] RELEASE_NOTES_v0.6.md drafted (non-template, no placeholders)

---

# Scope Discipline

- [ ] No runtime behavior violates determinism guarantees (see ADR-0001)
- [ ] Delegation remains metadata + trace only (no policy enforcement in v0.6)
- [ ] No distributed execution introduced (#339 deferred)
- [ ] No checkpoint/recovery engine introduced (#340 deferred)
- [ ] ObsMem remains separate project (#337 deferred)
- [ ] Advanced adaptive scheduler policies deferred to v0.7 (#338)

---

# WP Completion Gates

## Core Runtime

- [ ] WP-A (#401) Pattern registry + compiler expansion complete
- [ ] WP-B (#402) HITL pause/resume minimal surface complete
- [ ] WP-C (#403) Streaming output implemented without altering artifact determinism
- [ ] WP-D (#404) Provider profiles documented and validated
- [ ] WP-E (#405) Delegation metadata schema + trace logging implemented
- [ ] WP-F (#406) Determinism + scheduler policy hardening complete

## Tooling & Observability

- [ ] WP-G (#407) Instrumentation + replay diff + graph export available
- [ ] WP-H (#408) Demo matrix updated and validated
- [ ] WP-H2 (#409) Coverage audit complete (>80% per file or documented exception)

## Finalization

- [ ] WP-I (#410) Docs + review pass complete
- [ ] WP-J (#411) Release ceremony executed

---

# Quality Gates (Hard Blockers)

- [ ] `cargo fmt --all` passes
- [ ] `cargo clippy --all-targets -- -D warnings` passes
- [ ] `cargo test` passes
- [ ] CI green on `main`
- [ ] No unresolved high-priority runtime bugs labeled `version:v0.6`
- [ ] Coverage gate satisfied per WP-H2 (#409)

Coverage Gate Definition:
- Target: >80% per file.
- Exceptions must:
  - Be documented.
  - Have explicit issue link.
  - Have an owner.

---

# Documentation Integrity

- [ ] All v0.6 milestone docs contain no template placeholders e.g. curly braces
- [ ] DESIGN/WBS/SPRINT/DECISIONS reflect actual shipped behavior
- [ ] README updated if public surface changes
- [ ] SECURITY.md updated if threat model changes
- [ ] Demos reflect v0.6 capabilities accurately

---

# Release Packaging

- [ ] Version bumped appropriately
- [ ] Tag created (v0.6.0)
- [ ] Release notes finalized and reviewed
- [ ] GitHub release drafted and verified
- [ ] Artifacts reproducible from clean checkout

---

# Post-Release Discipline

- [ ] All v0.6 issues closed or explicitly deferred
- [ ] Deferred items moved to v0.7 label
- [ ] v0.7 epics (#412–#415) updated if scope changed
- [ ] Retrospective summary written in docs/milestones/v0.6/
- [ ] Roadmap updated

---

# Exit Criteria

v0.6 can ship only if:

1. All core WPs (#401–#411) are complete or explicitly deferred with rationale.
2. Determinism invariants remain intact and validated.
3. Coverage gate (#409) is satisfied.
4. CI is green and reproducible.
5. Documentation accurately describes shipped behavior.

No partial ship. No “we’ll fix it in patch.”  
v0.6 is a foundation release.
