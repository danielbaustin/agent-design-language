# Decisions Template

## Metadata
- Milestone: `{{milestone}}`
- Version: `{{version}}`
- Date: `{{date}}`
- Owner: `{{owner}}`

## Purpose
Capture significant decisions (architecture, scope, process) at the time they are made.

## How To Use
- Add one row per decision.
- Prefer links to issues/PRs over long prose.
- Keep status current: `accepted`, `rejected`, `deferred`, `superseded`.

## Decision Log
| ID | Decision | Status | Rationale | Alternatives | Impact | Link |
|---|---|---|---|---|---|---|
| D-01 | {{decision_1}} | {{status_1}} | {{rationale_1}} | {{alternatives_1}} | {{impact_1}} | {{link_1}} |
| D-02 | {{decision_2}} | {{status_2}} | {{rationale_2}} | {{alternatives_2}} | {{impact_2}} | {{link_2}} |

## Open Questions
- {{open_question_1}} (Owner: {{owner_oq1}}) (Issue: {{issue_oq1}})
- {{open_question_2}} (Owner: {{owner_oq2}}) (Issue: {{issue_oq2}})

## Exit Criteria
- All milestone-critical decisions are logged with a rationale.
- Deferred/rejected/superseded options are explicitly recorded.
- Open questions have owners and tracking links.

# Decisions – ADL v0.4

## Metadata
- Milestone: `v0.4`
- Version: `0.4`
- Date: {{date}}
- Owner: Daniel Austin
- Related issues: #290, #291

## Purpose
Capture significant v0.4 decisions (architecture, scope, process) as they are made, with links to evidence (issues/PRs).

## How to use
- Keep entries short; link to issues/PRs for details.
- Update status as decisions evolve: `accepted`, `deferred`, `superseded`.
- Add an “Open Question” instead of a half-decided decision.

## Decision Log

| ID | Decision | Status | Rationale | Alternatives | Impact | Link |
|---|---|---|---|---|---|---|
| D-01 | Implement **bounded** runtime concurrency for fork branches using a small executor/threadpool. | accepted | We need real runtime concurrency in v0.4, but must keep complexity low and behavior testable. A bounded executor avoids runaway parallelism and makes failure handling simpler. | Unbounded threads; fully async runtime; distributed executor. | Enables real parallel fork execution with controlled resource use. | #291 |
| D-02 | Preserve a **sequential mode** for deterministic replay/debugging and CI simplification. | accepted | Concurrency increases debug surface area. A sequential mode provides a stable fallback and helps compare traces/artifacts for determinism tests. | Remove sequential mode; always parallel. | Improves debuggability and reduces risk while shipping concurrency. | #291 |
| D-03 | Join semantics are **wait-for-all** with deterministic merge ordering by **branch ID**. | accepted | Prevents nondeterministic artifact ordering and makes join behavior auditable. | “First-success” joins; merge by completion order. | Deterministic outputs and stable traces across runs. | #291 |
| D-04 | Default failure policy: **fail-fast** on first unrecoverable branch failure; allow optional “complete-all then aggregate errors” later. | accepted | Matches current workflow expectations and keeps orchestration simple for the first runtime-concurrency increment. | Always aggregate; always continue. | Predictable failure behavior, simpler semantics for v0.4. | #291 |
| D-05 | Retry semantics remain **deterministic**: no jitter/backoff in v0.4; retries are per-node and must not change artifact ordering guarantees. | accepted | Determinism is core; jitter/backoff can change timing and interleavings in ways that complicate trace comparison. | Exponential backoff with jitter; global retry coordination. | Stable artifacts and trace reasoning; easier tests. | #291 |
| D-06 | Concurrency-safe state: branch state is isolated; only explicit outputs flow through join (no shared mutable state). | accepted | Avoids hidden data races and makes reasoning about branch correctness straightforward. | Shared mutable state with locks; global state store. | Reduces concurrency bugs; enforces explicit dataflow. | #291 |
| D-07 | **Observable memory + Bayesian indexing** is explicitly deferred to **v0.5**. | accepted | v0.4 must focus on execution engine credibility; memory/indexing needs separate storage/retrieval semantics and evaluation. | Attempt MVP in v0.4; bolt-on indexing. | Keeps v0.4 scope tight; sets clear roadmap. | (planned) |

## Open Questions

- What is the minimal API boundary between `ExecutionPlan` construction and the executor (types, module layout)? (Owner: Daniel) (Issue: #291)
- Should “aggregate errors” mode be a workflow-level flag in v0.4 or deferred to v0.5? (Owner: Daniel) (Issue: #291)
- What is the minimum determinism test suite we require (N repeated runs; artifact diff rules)? (Owner: Daniel) (Issue: #291)

## Exit Criteria
- All v0.4 milestone-critical decisions are logged with rationale.
- Any deferred/superseded decisions are explicitly marked.
- Open questions have owners and tracking links.