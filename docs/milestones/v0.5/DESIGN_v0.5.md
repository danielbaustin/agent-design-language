# ADL v0.5 Design — Deterministic Multi-Agent Patterns + Configurable Scheduler

## Metadata
- Milestone: `v0.5`
- Version: `0.5`
- Date: `2026-02-18`
- Owner: Daniel Austin
- Related issues: #308 (epic), #342, #343, #344, #345, #357, #346, #347, #361, #362, #363, #364

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
2. Scheduler behavior needs explicit policy documentation and guardrails for deterministic replay.
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

## v0.5 Language Overview

v0.5 formalizes six first-class primitives in a single ADL document:

- `providers`: AI model-call backends (for example `local_ollama`, `http_remote`)
- `tools`: MCP-style external capabilities (distinct from providers)
- `agents`: execution identities that reference a provider and optional tools
- `tasks`: reusable task contracts that reference an agent and optional tool allowlist
- `workflows`: step graphs (currently sequential or concurrent)
- `run`: execution instance that references a workflow and provides input/placement hooks

Minimal complete example (full file: `swarm/examples/v0-5-primitives-minimal.adl.yaml`):

```yaml
version: "0.5"
providers: { ... }
tools: { ... }
agents: { ... }
tasks: { ... }
workflows:
  wf_main:
    steps:
      - id: "summarize.topic"
        task: "summarize"
run:
  workflow_ref: "wf_main"
```

Validation expectations in v0.5:
- run uses `workflow_ref` or inline `workflow` (legacy), but never both; if multiple providers exist, provider selection must be explicit.
- ids may be declared explicitly (`id`) and must match collection keys when present
- refs must resolve (`provider`, `agent_ref`, `task`, `workflow_ref`, tool refs)
- provider/tool kinds must be supported with required config fields

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
- `run.defaults.max_concurrency` controls bounded execution (`>= 1`, default `4`)
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
- `RunDefaultsSpec { max_concurrency }`

PatternSchema compiles into deterministic ExecutionPlan nodes with stable IDs.

---

### Execution Semantics

1. Pattern expands to structured DAG.
2. Stable node IDs derived from canonicalized pattern + seed.
3. Scheduler respects:
   - `run.defaults.max_concurrency`
   - deterministic lexicographic tie-breaking by full step id
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
  - Set `run.defaults.max_concurrency: 1` for fully sequential behavior
  - Restrict to sequential workflows where needed

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

---

## WP-04 PatternSchema v0.1 (Implemented)

WP-04 ships a minimal PatternSchema compiler surface:

- `linear` pattern: ordered node chain (`A -> B -> C`)
- `fork_join` pattern: deterministic branch chains with join-node dependencies on each branch tail

Compilation outputs deterministic runtime-ready `ExecutionPlan` nodes with reserved IDs:

- `p::<pattern_id>::<node>`
- `p::<pattern_id>::<branch_id>::<node>`

Join semantics are encoded only as explicit dependencies in the plan; runtime scheduler behavior remains deterministic lexicographic ready-step ordering.
