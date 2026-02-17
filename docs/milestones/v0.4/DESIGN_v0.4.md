# Design Template

## Metadata
- Milestone: `{{milestone}}`
- Version: `{{version}}`
- Date: `{{date}}`
- Owner: `{{owner}}`
- Related issues: {{issues}}

## Purpose
Define what we are building, why, and how we validate it — concisely, with links to issues/PRs.

## Problem Statement
{{problem_statement}}

## Goals
- {{goal_1}}
- {{goal_2}}

## Non-Goals
- {{non_goal_1}}
- {{non_goal_2}}

## Scope
### In scope
- {{in_scope_1}}
- {{in_scope_2}}

### Out of scope
- {{out_of_scope_1}}
- {{out_of_scope_2}}

## Requirements
### Functional
- {{functional_requirement_1}}
- {{functional_requirement_2}}

### Non-functional
- Deterministic behavior and reproducible outputs.
- Clear failure semantics and observability.
- {{non_functional_requirement_1}}

## Proposed Design
### Overview
{{architecture_summary}}

### Interfaces / Data contracts
- {{interface_or_contract_1}}
- {{interface_or_contract_2}}

### Execution semantics
{{execution_semantics}}

## Risks and Mitigations
- Risk: {{risk_1}}
  - Mitigation: {{mitigation_1}}
- Risk: {{risk_2}}
  - Mitigation: {{mitigation_2}}

## Alternatives Considered
- Option: {{alternative_1}}
  - Tradeoff: {{tradeoff_1}}
- Option: {{alternative_2}}
  - Tradeoff: {{tradeoff_2}}

## Validation Plan
- Checks/tests: {{validation_checks}}
- Success metrics: {{success_metrics}}
- Rollback/fallback: {{rollback_plan}}

## Exit Criteria
- Goals/non-goals and scope boundaries are explicit.
- Validation plan is actionable and referenced by the milestone checklist.
- Major open questions are resolved or tracked in the decision log.

# ADL v0.4 Design – Runtime Concurrency + Orchestration

## Metadata
- Milestone: `v0.4`
- Version: `0.4`
- Date: `{{date}}`
- Owner: Daniel Austin
- Related issues: #290, #291

## Purpose
v0.4 moves ADL from “design + sequential execution” to a minimal but real runtime concurrency engine. The goal is to execute fork/join graphs deterministically while preserving the traceability, artifacts, and review discipline established in v0.3.

---

## Problem Statement
In v0.3, fork/join was design-only and execution remained sequential. ADL needs:

- A real execution scaffold for fork/join graphs.
- Deterministic concurrency semantics.
- Clear failure handling and retry policy boundaries.
- Observability strong enough to support future memory/indexing features (scheduled for v0.5).

Without this, ADL cannot credibly position itself as an orchestration language for multi-agent workflows.

---

## Goals
- Implement a deterministic runtime fork/join execution scaffold.
- Preserve sequential fallback for debugging and reproducibility.
- Ensure full trace artifacts for concurrent steps.
- Keep runtime simple, testable, and extensible.

## Non-Goals
- High-performance distributed scheduler.
- Persistent state store.
- Observable memory + Bayesian indexing (moved to v0.5).
- Cross-process or cluster-level execution.

---

## Scope

### In scope
- Fork node execution model (parallel branches).
- Join node semantics (wait-for-all).
- Execution graph validation prior to run.
- Deterministic scheduling strategy.
- Structured trace + artifact output per branch.

### Out of scope
- Remote cluster execution.
- Dynamic graph mutation during execution.
- Persistent run recovery.

---

## Requirements

### Functional
- Execute fork blocks concurrently using a bounded executor.
- Join blocks must wait for all upstream branches.
- Fail-fast behavior configurable at workflow level.
- Retry policy applied per node (existing policy integrated).

### Non-functional
- Deterministic ordering of emitted artifacts.
- Reproducible execution traces.
- Clear and debuggable failure semantics.
- No unsafe shared mutable state.

---

## Proposed Design

### Overview
We introduce a lightweight execution engine layer:

- GraphBuilder → validates DAG and constructs execution plan.
- Executor → runs nodes (sequential or concurrent mode).
- JoinBarrier → waits for branch completion.
- TraceRecorder → records step lifecycle events.

Concurrency model:
- Use a bounded thread pool.
- Each fork branch executes independently.
- Join waits for all branch futures.
- Results merged deterministically (sorted by branch id).

Sequential mode remains available for:
- Debugging
- Deterministic replay
- CI tests

---

### Interfaces / Data Contracts

- `ExecutionPlan` – validated DAG with explicit dependencies.
- `ExecutionContext` – immutable runtime inputs + scoped branch state.
- `ExecutionResult` – node result + metadata + trace link.
- `JoinResult` – ordered merge of branch results.

Branch state must be isolated; only explicit outputs may flow across joins.

---

### Execution Semantics

1. Validate DAG.
2. Topologically sort nodes.
3. When encountering a fork:
   - Spawn tasks for each branch.
   - Record branch IDs.
4. Join barrier waits for all branch completions.
5. Merge outputs deterministically.
6. Continue downstream execution.

Failure policy:
- Default: fail-fast on first unrecoverable error.
- Optional: complete-all then aggregate errors.

Retry policy:
- Apply existing retry logic per node.
- Retries must not violate determinism of final artifacts.

---

## Risks and Mitigations

- Risk: Non-deterministic artifact ordering.
  - Mitigation: Explicit branch ID ordering during join merge.

- Risk: Hidden shared state bugs.
  - Mitigation: Enforce immutable context and scoped branch state.

- Risk: Debug complexity.
  - Mitigation: Preserve sequential execution mode.

---

## Alternatives Considered

- Async runtime (e.g., async/await everywhere)
  - Tradeoff: Higher complexity, less transparent execution model.

- Fully distributed executor
  - Tradeoff: Premature optimization and infrastructure complexity.

---

## Validation Plan

- Unit tests for fork/join behavior.
- Determinism test: multiple runs produce identical artifacts.
- Failure tests: branch failure scenarios.
- CI must remain green under concurrent execution mode.

Success metrics:
- Concurrent demo workflow executes correctly.
- No regression in sequential mode.

Rollback:
- Feature flag to disable concurrency and revert to sequential mode.

---

## Exit Criteria

- Fork/join executes concurrently with deterministic artifacts.
- CI is green.
- Sequential mode still passes all tests.
- Design doc reflects shipped behavior.