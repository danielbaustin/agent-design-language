# Sprint Plan — v0.75 (Sprint 01)

## Metadata
- Sprint: 0.75-S1
- Milestone: v0.75
- Start date: 2026-03-03
- End date: 2026-03-09 (target)
- Owner: Daniel / Agent Logic team

## Sprint Goal
Freeze the deterministic substrate contracts (activation log + replay + trace bundle v2) and establish the structural foundation for ObsMem v1 without expanding scope beyond EPIC-A.

This sprint focuses on stability and contract clarity, not feature expansion.

---

## Planned Scope
- Finalize and freeze Activation Log schema (contract definition + tests).
- Harden Replay runner and prove deterministic equivalence on representative workflows.
- Specify and implement Trace Bundle v2 export (versioned, canonical).
- Begin Trace Bundle import path and replay-from-bundle validation.

Out of scope for this sprint:
- ObsMem ingestion/query implementation (next sprint).
- Cluster execution.
- Gödel layer.
- Authoring surfaces.

---

## Work Plan

| Order | Item | Issue | Owner | Status |
|------|------|-------|--------|--------|
| 1 | Activation log schema freeze + invariants | TBD (WP-02) | Daniel/Codex | Planned |
| 2 | Replay determinism hardening + regression tests | TBD (WP-03) | Daniel/Codex | Planned |
| 3 | Trace bundle v2 spec + export implementation | TBD (WP-05) | Daniel/Codex | Planned |
| 4 | Trace bundle import + replay-from-bundle proof (initial) | TBD (WP-06) | Daniel/Codex | Planned |
| 5 | Determinism audit (ordering + host-path + secrets check) | TBD | Daniel | Planned |

Execution order is sequential; do not begin import before export spec is stable.

---

## Milestone Phasing (v0.75)

v0.75 is executed in three controlled sprints to reduce scope creep and preserve determinism guarantees.

### Sprint 01 — Substrate Freeze (this document)
Covers:
- WP-02 Activation log schema freeze
- WP-03 Replay determinism hardening
- WP-04 Failure taxonomy stabilization
- WP-05 Trace bundle v2 export
- WP-06 Trace bundle import + replay-from-bundle proof

Goal: Freeze execution + replay contracts before layering memory.

### Sprint 02 — ObsMem v1 Implementation
Covers:
- WP-07 Index schema
- WP-08 Ingestion pipeline
- WP-09 Structured query
- WP-10 Hybrid retrieval (optional) + ranking
- WP-11 Citations + evidence rendering
- WP-12 Operational reports

Goal: Implement deterministic memory layer on top of frozen substrate.

### Sprint 03 — Convergence & Release
Covers:
- WP-13 Demo matrix
- WP-14 Coverage / quality gate ratchet
- WP-15 Docs + review pass
- WP-16 Release ceremony

Goal: Converge, audit, demo, and ship v0.75 cleanly.

---

## Cadence Expectations
- Each item has input/output cards under `.adl/cards/<issue>/`.
- Draft PR opened before merge; CI must be green before marking DONE.
- Determinism tests must include structure or byte-level validation where appropriate.
- No widening of scope mid-sprint.

---

## Risks / Dependencies

### Risk: Hidden nondeterminism in replay
- Example: ordering differences, timestamp leakage, temp path variation.
- Mitigation: enforce stable ordering; strip volatile fields; add regression tests.

### Risk: Bundle schema churn
- Mitigation: version manifest explicitly as v2; do not revise after freeze without recorded decision.

### Dependency: v0.7 quality gate stability
- CI and coverage must remain green during substrate freeze.

---

## Demo / Review Plan
- Internal demo at sprint end:
  - Run representative workflow.
  - Export trace bundle v2.
  - Import bundle.
  - Replay from bundle.
  - Confirm artifact equivalence (excluding run-id/timestamps).
- Review: sprint-end architecture check against DESIGN_0.75.md.

---

## Exit Criteria
- Activation log schema documented and tested.
- Replay determinism proven on representative workflows.
- Trace bundle v2 export implemented and versioned.
- Replay-from-bundle works for at least one non-trivial workflow.
- CI remains green.

If these are complete, Sprint 01 is DONE and Sprint 02 (ObsMem v1 implementation) may begin.
