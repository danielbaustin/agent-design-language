# Sprint Plan — v0.6

## Metadata
- Milestone: v0.6
- Version: v0.6
- Owner: ADL core (Daniel + Codex-assisted implementation)
- Duration model: 3 execution sprints + release sprint
- Governing WPs: #401–#411
- Quality gate: #409 (coverage audit >80% per file)

---

# Sprint 1 — Deterministic Foundations

## Goal
Lock down determinism invariants and formalize the pattern registry
before adding new runtime surfaces.

This sprint establishes architectural safety so later work does not
destabilize ordering guarantees.

## Scope
- WP-A: Pattern registry + compiler expansion (#401)
- WP-F: Determinism + scheduler hardening (#406)
- Initial design clarifications for scheduler policy surface (historical context only; not a v0.6 gate)

## Planned Work (Execution Order)

| Order | Item | Issue | Owner | Status |
|-------|------|-------|-------|--------|
| 1 | Formalize registry abstraction boundary | #401 | Daniel + Codex | Planned |
| 2 | Ensure byte-stable compile transforms | #401 | Codex | Planned |
| 3 | Add regression tests for compile stability | #401 | Codex | Planned |
| 4 | Clarify max_concurrency override semantics | #406 | Daniel | Planned |
| 5 | Harden lexicographic batching guarantees | #406 | Codex | Planned |
| 6 | Add determinism regression tests | #406 | Codex | Planned |

## Definition of Done
- Pattern compilation is byte-stable.
- Scheduler invariants explicitly tested.
- No regression in existing deterministic tests.
- CI fully green.

---

# Sprint 2 — Runtime Surface Extensions

## Goal
Add minimal, explicitly defined runtime extensions without compromising
the deterministic execution model.

## Scope
- WP-B: HITL pause/resume (#402)
- WP-C: Streaming output semantics (#403)
- WP-E: Delegation metadata (log-only) (#405)
- Early alignment with signing trust policy (#371, read-only coordination)

## Planned Work (Execution Order)

| Order | Item | Issue | Owner | Status |
|-------|------|-------|-------|--------|
| 1 | Extend execution state machine (Paused state) | #402 | Codex | Planned |
| 2 | Implement resume entrypoint + tests | #402 | Codex | Planned |
| 3 | Define streaming lifecycle boundaries | #403 | Daniel | Planned |
| 4 | Implement streaming trace events | #403 | Codex | Planned |
| 5 | Extend schema with delegation metadata block | #405 | Codex | Planned |
| 6 | Log delegation metadata in trace (no enforcement) | #405 | Codex | Planned |
| 7 | Add regression + integration tests | #402/#403/#405 | Codex | Planned |

## Definition of Done
- Pause/resume visible in trace and test-covered.
- Streaming does not alter final artifact determinism.
- Delegation metadata appears in trace.
- Determinism tests still pass across repeated runs.

---

# Sprint 3 — Tooling, Profiles, and Validation

## Goal
Make v0.6 demonstrable, observable, and measurable.

## Scope
- WP-D: Provider profiles (#404)
- WP-G: Instrumentation + replay diff + graph export (#407)
- WP-H: Demo matrix (#408)
- WP-H2: Coverage audit (#409)

## Planned Work (Execution Order)

| Order | Item | Issue | Owner | Status |
|-------|------|-------|-------|--------|
| 1 | Define provider profile documentation surface | #404 | Daniel | Planned |
| 2 | Implement structured trace export format | #407 | Codex | Planned |
| 3 | Implement replay diff utility | #407 | Codex | Planned |
| 4 | Implement graph export (Mermaid or DOT) | #407 | Codex | Planned |
| 5 | Define demo matrix coverage | #408 | Daniel | Planned |
| 6 | Implement demo scenarios | #408 | Codex | Planned |
| 7 | Run per-file coverage audit | #409 | Codex | Planned |
| 8 | Raise coverage to >80% where practical | #409 | Codex | Planned |
| 9 | Document justified coverage exclusions | #409 | Daniel | Planned |

## Definition of Done
- Replay + graph export produce usable artifacts.
- Demo matrix runs deterministically in CI.
- Coverage >80% per file (or documented exception).
- No regression in runtime invariants.

---

# Sprint 4 — Documentation and Release

## Goal
Ship a coherent, documented, stable release.

## Scope
- WP-I: Docs + review pass (#410)
- WP-J: Release ceremony (#411)
- Alignment with remote security envelope (#370)
- Alignment with signing trust policy (#371)

## Planned Work (Execution Order)

| Order | Item | Issue | Owner | Status |
|-------|------|-------|-------|--------|
| 1 | Align README + milestone docs | #410 | Daniel | Planned |
| 2 | Final regression review | #410 | Daniel | Planned |
| 3 | Validate coverage + demo artifacts | #409/#408 | Daniel | Planned |
| 4 | Tag release | #411 | Daniel | Planned |
| 5 | Publish release notes | #411 | Daniel | Planned |

## Definition of Done
- All WPs #401–#411 closed.
- CI green on main.
- Coverage audit complete.
- Release notes accurate and published.
- v0.6.0 tag created.

---

# Cadence Expectations

- Use issue cards (`input` / `output`) for each WP or subtask.
- No multi-WP PRs.
- Determinism invariants are reviewed before merge.
- CI must be green before merge to main.
- Avoid parallel edits in the same file across WPs.

---

# Risks and Mitigation

Risk: Streaming conflicts with determinism.
Mitigation: Enforce lifecycle boundary ordering tests.

Risk: Pause/resume introduces hidden state.
Mitigation: Explicit state transitions + trace assertions.

Risk: Coverage audit consumes disproportionate time.
Mitigation: Timebox + document exclusions instead of perfection.

Risk: Scope creep into v0.7 (learning/adaptive runtime).
Mitigation: Delegation remains log-only; no adaptive scheduler.

---

# Exit Criteria (Milestone-Level)

- All sprint definitions of done satisfied.
- Deterministic behavior preserved across multiple runs.
- Replay + graph export produce consistent artifacts.
- Coverage audit complete.
- Documentation reflects shipped runtime.
- No template placeholders remain in milestone docs.
