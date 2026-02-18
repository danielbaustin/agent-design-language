# ADL v0.4 Design - Shipped Runtime Concurrency

## Metadata
- Milestone: `v0.4`
- Version: `0.4`
- Date: `2026-02-18`
- Owner: Daniel Austin
- Related issues: #290, #296, #297, #298, #302, #304, #306

## Problem Statement
v0.3 had deterministic fork/join modeling but runtime behavior needed stronger, plan-driven concurrency wiring and release-grade demos proving the behavior end-to-end.

## Goals (Shipped)
- Execute workflows through validated `ExecutionPlan` dependencies.
- Run fork-stage work with bounded concurrency.
- Enforce deterministic join barrier behavior.
- Preserve deterministic output/trace behavior and v0.3 compatibility.
- Provide no-network, copy/paste demos that prove runtime behavior.

## Non-Goals (v0.4)
- Distributed scheduling.
- Persistent recovery/checkpoint engine.
- Configurable runtime parallelism knobs.
- New schema version beyond currently accepted ADL document versions.

## Shipped Design

### Planner and Graph
- `ExecutionPlan` + DAG validation is built before execution.
- Structural concurrent dependencies are encoded for fork/join shape:
  - `fork.branch.*` depends on `fork.plan` (when present)
  - `fork.join` depends on all `fork.branch.*`

### Runtime Execution
- Concurrent workflows execute through plan-ready sets.
- Ready-node ordering is deterministic (stable sort by step id).
- Fork work executes via bounded executor.
- Join executes only after all required branch dependencies complete.

### Failure and Retry Semantics
- Existing deterministic retry semantics remain in place.
- Fail-fast behavior remains default for unrecoverable failures.
- v0.3 behavior remains intact.

### Artifacts and Trace
- Stable run artifacts and step outputs are preserved.
- Trace event ordering remains deterministic for repeated runs with same inputs.

## Validation Evidence
- WP-01 planner/DAG scaffold: [#299](https://github.com/danielbaustin/agent-design-language/pull/299)
- WP-02 bounded executor: [#300](https://github.com/danielbaustin/agent-design-language/pull/300)
- WP-03 deterministic join: [#301](https://github.com/danielbaustin/agent-design-language/pull/301)
- Runtime wiring burst 2: [#303](https://github.com/danielbaustin/agent-design-language/pull/303)
- Runtime wiring burst 3: [#305](https://github.com/danielbaustin/agent-design-language/pull/305)
- Demo pass: [#307](https://github.com/danielbaustin/agent-design-language/pull/307)

## Current Limitations
- Runtime concurrency limit is fixed at `MAX_PARALLEL=4`.
- Configurable parallelism and advanced scheduling are deferred.

## Exit
v0.4 ships real, observable runtime concurrency with deterministic join behavior and reproducible demos, while preserving existing v0.3 stability guarantees.
