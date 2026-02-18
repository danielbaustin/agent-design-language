# ADL v0.5 Design — Deterministic Multi-Agent Patterns + Configurable Scheduler

## Metadata
- Milestone: `v0.5`
- Version: `0.5`
- Date: `2026-02-18`
- Owner: Daniel Austin
- Related issues: #308 (epic), #309 (Burst 1), TBD pattern issues

## Purpose
Define how ADL evolves from deterministic workflow execution (v0.4) into a deterministic multi-agent orchestration platform with configurable runtime scheduling and a clean boundary for an external Observable Memory (ObsMem) module.

This document locks architectural intent before implementation proceeds.

---

## Problem Statement

v0.4 shipped:
- ExecutionPlan-driven runtime
- Bounded fork execution
- Deterministic join barrier
- Deterministic replayable demos

However:

1. Multi-agent patterns (debate, planner-executor, referee, hierarchical) are not first-class language constructs.
2. Scheduler behavior is fixed (MAX_PARALLEL=4).
3. Observable memory is conceptual but not productized or modularized.
4. The six ADL primitives are not yet explicitly schema-bound with composition guarantees.

v0.5 must:
- Expand the ADL language
- Strengthen runtime configurability
- Establish ADL as a deterministic alternative to AutoGen-style systems

---

## Goals

- Introduce explicit schemas for all six primitives:
  - Agents
  - Runs
  - Providers
  - Tasks
  - Tools
  - Workflows
- Define compositional model across primitives.
- Introduce first-class deterministic multi-agent patterns.
- Expose configurable concurrency controls in runtime.
- Preserve determinism guarantees.
- Keep ObsMem modular and separable as its own crate.

---

## Non-Goals

- Distributed execution across clusters.
- Full checkpoint/recovery engine.
- Persistent long-term memory store in core runtime.
- Non-deterministic scheduling policies.
- External orchestration platform (K8s integration, etc.).

---

## Scope

### In scope

- Pattern schema (v0.1)
- Pattern → ExecutionPlan compiler
- Deterministic multi-agent turn ordering
- Configurable `max_parallel`
- Scheduler policy surface (minimal: fifo | fair)
- Stable deterministic trace markers for patterns
- ObsMem crate scaffold (separate project)
- Demo pass covering each primitive alone and in composition

### Out of scope

- Distributed multi-node scheduler
- Advanced dynamic scaling
- Full RAG system embedded in runtime

---

## Requirements

### Functional

- All six primitives have explicit schema definitions.
- Workflows can embed patterns.
- Pattern compilation produces deterministic ExecutionPlan graphs.
- Runtime supports configurable concurrency limits.
- Multi-agent execution preserves stable ordering and replayability.
- Demos exist for:
  - linear
  - multi-step
  - hierarchical
  - remote
  - fork/join
  - multi-agent debate

### Non-functional

- Deterministic behavior and reproducible outputs.
- Clear failure semantics and observability.
- Stable trace schema with pattern markers.
- CI green with expanded test surface.
- Nightly coverage automation targets as high as reasonable coverage (non-blocking but reported).

---

## Proposed Design

### Overview

v0.5 introduces a three-layer architecture:

1. Language Layer  
   Explicit schemas for primitives and patterns.

2. Compilation Layer  
   Pattern compiler → ExecutionPlan DAG.

3. Runtime Layer  
   Configurable bounded scheduler executing deterministic DAGs.

ObsMem exists as a separate crate and integrates through Tools or Providers.

---

### Interfaces / Data Contracts

- `AgentSchema`
- `TaskSchema`
- `ToolSchema`
- `ProviderSchema`
- `RunSchema`
- `WorkflowSchema`
- `PatternSchema`
- `SchedulerConfig { max_parallel, policy }`

PatternSchema compiles into deterministic ExecutionPlan nodes with stable IDs.

---

### Execution Semantics

1. Pattern expands to structured DAG.
2. Stable node IDs derived from canonicalized pattern + seed.
3. Scheduler respects:
   - max_parallel
   - deterministic tie-breaking
4. Join barriers preserve stable artifact ordering.
5. Trace logs:
   - PatternStart
   - PatternTurn
   - PatternJoin
   - SchedulerDecision

Replay remains provider-free and validates ordering invariants.

---

## Risks and Mitigations

- Risk: Pattern explosion increases complexity.
  - Mitigation: Start with minimal set (debate, planner_executor).

- Risk: Configurable scheduler breaks determinism.
  - Mitigation: Deterministic tie-breaking and invariant tests.

- Risk: ObsMem coupling contaminates runtime.
  - Mitigation: Separate crate; integrate via Tool interface only.

---

## Alternatives Considered

- Option: Hardcode patterns in runtime.
  - Tradeoff: Faster to implement, but not extensible.

- Option: Fully dynamic non-deterministic scheduler.
  - Tradeoff: Performance flexibility but breaks replay guarantees.

---

## Validation Plan

- Unit tests for:
  - Pattern compilation
  - Scheduler determinism
  - Configurable concurrency
- Integration tests:
  - Debate demo
  - Hierarchical planner demo
  - Remote multi-agent demo
- Success metrics:
  - Stable replay across 100 runs
  - CI green
  - Demo pass automated
- Rollback:
  - Disable configurable scheduler
  - Revert to v0.4 fixed concurrency

---

## Exit Criteria

- All six primitives have schemas and validation tests.
- Pattern compiler produces deterministic DAGs.
- Configurable scheduler works and remains deterministic.
- ObsMem crate scaffolded separately.
- Demo pass complete.
- Documentation pass complete.
- Review pass complete.
- Milestone checklist gates satisfied.
