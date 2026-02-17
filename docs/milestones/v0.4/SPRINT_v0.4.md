# Sprint Template

## Metadata
- Sprint: `{{sprint_id}}`
- Milestone: `{{milestone}}`
- Start date: `{{start_date}}`
- End date: `{{end_date}}`
- Owner: `{{owner}}`

## How To Use
- Keep scope small enough to finish with green CI and merged PRs.
- List work items in planned execution order.
- Track blockers here (not scattered chat notes).

## Sprint Goal
{{sprint_goal}}

## Planned Scope
- {{scope_item_1}}
- {{scope_item_2}}
- {{scope_item_3}}

## Work Plan
| Order | Item | Issue | Owner | Status |
|---|---|---|---|---|
| 1 | {{work_item_1}} | {{issue_1}} | {{owner_1}} | {{status_1}} |
| 2 | {{work_item_2}} | {{issue_2}} | {{owner_2}} | {{status_2}} |
| 3 | {{work_item_3}} | {{issue_3}} | {{owner_3}} | {{status_3}} |

## Cadence Expectations
- Use issue cards (`input`/`output`) for each item.
- Keep changes scoped per issue; use draft PRs until checks pass.
- Run required quality gates (fmt/clippy/test) for code changes.

## Risks / Dependencies
- Dependency: {{dependency_1}}
  - Risk: {{risk_1}}
  - Mitigation: {{mitigation_1}}

## Demo / Review Plan
- Demo artifact: {{demo_artifact}}
- Review date: {{review_date}}
- Sign-off owners: {{signoff_owners}}

## Exit Criteria
- All planned scope items completed or explicitly deferred with rationale.
- Linked issues/PRs updated and traceable.
- CI is green for merged work.
- Sprint summary captured in milestone docs.

# ADL v0.4 – Sprint 1 (Runtime Concurrency Scaffold)

## Metadata
- Sprint: `v0.4-S1`
- Milestone: `v0.4`
- Start date: {{start_date}}
- End date: {{end_date}}
- Owner: Daniel Austin

---

## Sprint Goal
Ship a minimal, deterministic fork/join runtime scaffold that executes branches concurrently while preserving traceability and CI stability.

This sprint focuses on correctness and determinism — not performance optimization.

---

## Planned Scope
- Implement bounded executor for fork branches.
- Implement deterministic join (wait-for-all + ordered merge).
- Preserve sequential execution mode for fallback/debug.
- Add deterministic concurrency tests.
- Keep CI fully green.

---

## Work Plan

| Order | Item | Issue | Owner | Status |
|---|---|---|---|---|
| 1 | Define `ExecutionPlan` + DAG validation scaffold | #291 | Daniel | planned |
| 2 | Implement bounded executor for fork branches | #291 | Daniel | planned |
| 3 | Implement deterministic join barrier (ordered merge by branch ID) | #291 | Daniel | planned |
| 4 | Preserve and test sequential execution mode | #291 | Daniel | planned |
| 5 | Add determinism test (multi-run artifact equality) | #291 | Daniel | planned |
| 6 | CI validation + artifact review | #291 | Daniel | planned |

---

## Cadence Expectations
- Each logical unit of work may be split into sub-issues if needed.
- All changes go through draft PR first.
- Required quality gates:
  - `cargo fmt`
  - `cargo clippy --all-targets -- -D warnings`
  - `cargo test`
- No merge unless CI is fully green.

---

## Risks / Dependencies

- Dependency: Existing v0.3 sequential execution engine.
  - Risk: Refactor introduces regression in sequential mode.
  - Mitigation: Maintain explicit sequential path and regression tests.

- Dependency: Deterministic artifact ordering.
  - Risk: Concurrency introduces nondeterministic ordering.
  - Mitigation: Explicit branch ID ordering at join stage.

---

## Demo / Review Plan
- Demo artifact: Minimal fork/join workflow demonstrating parallel branches.
- Validation: Run workflow multiple times and compare artifacts.
- Review focus:
  - Determinism guarantees
  - Failure semantics
  - Code clarity over cleverness

---

## Exit Criteria
- Fork branches execute concurrently under bounded executor.
- Join waits for all branches and merges deterministically.
- Sequential mode still passes all existing tests.
- Determinism test passes across multiple runs.
- CI is green.
- No untracked regressions in existing demos.