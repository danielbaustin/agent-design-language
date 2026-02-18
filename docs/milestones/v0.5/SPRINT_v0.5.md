# ADL v0.5 Sprint Plan

## Metadata
- Sprint: `v0.5-S1`
- Milestone: `v0.5`
- Start date: `2026-02-19`
- End date: `TBD`
- Owner: `Daniel Austin`

---

## Sprint Goal
Establish the structural foundation for v0.5 by locking the language surface (6 primitives), defining composition rules, and scaffolding configurable scheduler controls—without introducing runtime instability.

This sprint is architecture-first and discipline-first.

---

## Planned Scope (Unit 0 + Early Work Units)

### Unit 0 — Milestone Initialization (#330)
- Populate and freeze all v0.5 milestone docs
- Define work units clearly in WBS
- Define demo matrix
- Freeze v0.5 scope

### WP-01 — Explicit Primitive Schemas
- Define explicit schemas for:
  - Agents
  - Runs
  - Providers
  - Tasks
  - Tools
  - Workflows
- Validate schema consistency and naming
- Add validation tests (schema load + negative cases)

### WP-02 — Composition Rules + Validation
- Define how primitives reference each other
- Enforce composition constraints in validator
- Add deterministic ordering guarantees for composed graphs

### WP-03 — Configurable Scheduler Controls (#309)
- Add concurrency limit configuration surface
- Wire config → runtime executor
- Add integration tests for parallelism levels (1, 2, N)
- Preserve deterministic replay behavior

---

## Work Plan

| Order | Work Unit | Issue | Status |
|-------|----------|-------|--------|
| 1 | Unit 0 — Milestone Init | #330 | In Progress |
| 2 | WP-01 — Explicit primitive schemas | (TBD) | Planned |
| 3 | WP-02 — Composition rules + validation | (TBD) | Planned |
| 4 | WP-03 — Configurable scheduler controls | #309 | Planned |
| 5 | Demo generation pass | (TBD) | Planned |
| 6 | Documentation pass | (TBD) | Planned |
| 7 | Review pass | (TBD) | Planned |
| 8 | Closing ceremony (tag + release docs) | (TBD) | Planned |

---

## Demo Matrix (v0.5 Target)

### Primitive-Alone Demos
- Agent-only demo
- Task-only demo
- Tool-only demo
- Provider-only demo
- Workflow-only demo

### Composition Demos
- Linear workflow
- Multi-step workflow
- Hierarchical workflow
- Parallel fork/join workflow
- Local + remote mixed placement workflow
- Deterministic replay workflow

All demos must be:
- One-command runnable
- No-network reproducible (mock provider available)
- Documented in README

---

## Discipline Rules
- All implementation behind issue cards (`input` / `output`)
- Draft PR required before merge
- CI must be green before merge
- Each work unit must reference a WBS item
- No silent scope expansion

---

## Risks

### Tooling Stability
- `pr.sh start` behavior inconsistent
- Mitigation: tracked separately; avoid coupling runtime work to tooling fixes

### Schema Drift
- Risk: primitives evolve inconsistently
- Mitigation: freeze definitions before runtime wiring

### Determinism Regression
- Risk: configurable concurrency breaks replay guarantees
- Mitigation: deterministic ordering remains default; tests required for all parallel levels

---

## Exit Criteria
- Unit 0 complete and milestone structure frozen
- Primitive schemas explicitly defined and validated
- Composition rules documented and enforced
- Scheduler configurability scaffolded with tests
- Demo matrix implemented and runnable
- Documentation pass complete
- Review pass complete
- CI green
- Ready for `v0.5.0` tag