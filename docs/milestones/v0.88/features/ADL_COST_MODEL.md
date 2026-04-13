

# ADL COST MODEL — Time, Speed, and Economic Efficiency

## Metadata
- Owner: `adl`
- Status: `promoted milestone feature doc`; bounded runtime review surface implemented for `WP-08`
- Target milestone: `v0.88`
- Area: Runtime / Cognitive Architecture / Economics

---

## Purpose

Define a **first-class cost model for ADL** that connects:

- time (chronosense)
- execution (trace)
- cognition (agent workflows)
- economics (compute, latency, money)

This document establishes cost as a **measurable, inspectable, and optimizable property of cognition**.

---

## Core Thesis

> **Cost is the shadow of time in a computational substrate.**

In ADL:

- time is structured (chronosense)
- execution is explicit (trace)
- therefore cost can be **derived, attributed, and optimized**

This is not possible in opaque model-call systems.

---

## Why This Matters

Without a cost model:

- agents cannot reason about efficiency
- systems cannot optimize execution strategies
- users cannot predict or control spend
- "fast" vs "slow" becomes subjective

With a cost model:

- cognition becomes **economically grounded**
- optimization becomes **systematic, not heuristic**
- ADL gains a **defensible enterprise advantage**

---

## Relationship to Chronosense

Chronosense defines:

- ordering of events
- duration of execution
- lifetime of agents


The cost model builds directly on this:

> **Cost = f(duration, compute, resources, policy)**

Every cost must be anchored in:

- when it occurred
- how long it took
- what produced it
- what execution posture was requested

---

## Relationship to Cognitive Spacetime

In the Cognitive Spacetime Model:

- events occur in ordered time
- state evolves causally
- agents follow worldlines


The cost model adds:

> **Every event also carries economic weight.**

This makes cognition:

- measurable
- comparable
- optimizable

---

## Cost Dimensions

### 1. Time Cost

Measured directly from chronosense:

- wall time (UTC)
- monotonic duration
- agent lifetime delta

```
time_cost:
  started_at
  completed_at
  duration_ms
```

---

### 2. Compute Cost

Represents actual resource consumption:

- model tokens (input/output)
- CPU/GPU time
- memory footprint (optional in v1)

```
compute_cost:
  tokens_in
  tokens_out
  model
  provider
```

---

### 3. Monetary Cost

Derived from provider pricing:

```
monetary_cost:
  usd
  pricing_model_ref
```

---

### 4. Cognitive Cost

A uniquely ADL concept.

Represents **how much cognitive work was performed**.

Examples:

- number of reasoning steps
- number of branches explored
- depth of refinement loops

```
cognitive_cost:
  steps
  branches
  refinement_cycles
```

---

### 5. Opportunity Cost

Represents alternatives not taken.

Examples:

- faster model not used
- cached result bypassed
- parallelism not exploited

This is estimated, not directly measured.

---

## Execution Mode / Compute Policy

Cost should always be interpreted in the context of requested execution policy.

Canonical execution modes:

- `efficient`
  - minimal compute
  - strict constraints
  - conservative branching and tool use

- `fast`
  - higher compute if needed for faster turnaround
  - latency-biased execution

- `deterministic`
  - strict replay expectations
  - bounded variability
  - constrained entropy and branching

- `exploratory`
  - higher branching
  - higher tolerated entropy
  - wider search and refinement behavior

ADL should record both:

- the requested execution mode / compute policy
- the realized cost and execution behavior

Without that split, a reviewer can see spend but not understand why the system spent it.

For the current bounded `v0.88` implementation, the proof surface is:

- `adl::chronosense::ExecutionPolicyCostModelContract`
- `adl identity cost --out .adl/state/execution_policy_cost_model_v1.json`
- `.adl/state/execution_policy_cost_model_v1.json`

---

## Cost Anchoring (Mandatory)

> **Every cost must be attached to a trace event.**

```
cost_anchor:
  trace_event_id
  run_id
  agent_id
  observed_at_utc
  execution_policy
  duration_ms
  cost_vector
```

This ensures:

- full attribution
- replay consistency
- auditability
- policy-aware reviewability

The current bounded contract keeps these anchor fields explicit as reviewer-facing requirements rather than hidden runtime metadata.

---

## Cost Vector

All costs are expressed as a unified structure:

```
cost_vector:
  time_ms
  tokens
  usd
  cognitive_units
```

This allows:

- comparison across runs
- optimization strategies
- policy enforcement

Recommended companion structures:

```
execution_policy:
  requested_mode: efficient | fast | deterministic | exploratory
  max_tokens: <optional>
  max_duration_ms: <optional>
  max_branches: <optional>
  max_tool_calls: <optional>
  replay_strictness: strict | bounded | relaxed
  latency_target_ms: <optional>

execution_realization:
  branch_count
  tool_calls
  refinement_cycles
  replay_variance: strict | bounded | high
```

The current bounded runtime surface owns these reviewable structures directly instead of scattering them across unrelated docs or output fields.

---

## Fast vs Slow (Capability Layer)

We now define "fast" precisely.

> **Fast = minimizing cost vector under constraints**

Not just latency.

A "fast" agent may:

- use smaller models
- reduce steps
- reuse memory (ObsMem)
- parallelize execution

A "slow" agent may:

- explore more branches
- use larger models
- perform deeper reasoning

These tradeoffs are better represented as explicit execution modes than as loose
adjectives.

---

## Optimization Surfaces

ADL enables explicit optimization:

### 1. Model Selection

- tradeoff: cost vs capability

### 2. Workflow Design

- reduce unnecessary steps
- collapse redundant branches

### 3. Memory Reuse (ObsMem)

- avoid recomputation


### 4. Parallel Execution

- reduce wall time without increasing compute

### 5. Adaptive Execution (AEE)

- early stopping
- bounded refinement

---

## Cost-Aware Agents

Agents can now reason about cost:

Examples:

- "Use a cheaper model unless confidence is low"
- "Stop after 2 refinement cycles"
- "Reuse prior result if within tolerance"

This is a major step toward:

> **economic intelligence**

---

## Determinism Requirements

Cost must be:

- reproducible from trace
- stable under replay
- explainable to a reviewer

Not allowed:

- hidden costs
- untracked compute
- non-attributed spend

---

## Reviewability

A reviewer must be able to answer:

- what did this run cost?
- where was cost incurred?
- why was this path chosen?

Cost is part of truth, not metadata.

For `WP-08`, the required reviewer comparison rule is:

> reviewers must be able to compare requested execution posture against realized cost and execution behavior

The current required trace hooks are:

- `run_state.v1.duration_ms`
- `run_state.v1.scheduler_max_concurrency`
- `run_summary.v1.policy`
- `run_summary.v1.counts.provider_call_count`

---

## Demo Surface

For v0.87+:

- trace includes cost fields
- output card includes total cost vector
- demo compares:
  - "fast path"
  - "deep reasoning path"

---

## Strategic Insight

This is subtle but important.

Most AI systems optimize for:

- accuracy
- capability

ADL can optimize for:

- **cost-efficient cognition**

This is what enterprises actually need.

---

## Connection to Time Perception Research

Human and biological systems also trade time vs effort:

- more events → longer perceived duration
- more processing → richer experience


ADL mirrors this:

- more steps → higher cognitive cost
- more computation → richer reasoning

---

---

## Cost Policies (Governance Layer)

Measurement and optimization are not sufficient on their own.

ADL requires **explicit cost constraints** that govern execution.

> **Cost policies define the acceptable economic bounds of cognition.**

These policies operate alongside:

- Freedom Gate (governance / ethical constraints)
- AEE (adaptive execution limits)
- enterprise SLAs

---

### Policy Structure

```

The current `v0.88` contract records these policy fields as bounded review surfaces. It does not yet implement a full adaptive enforcement engine.
cost_policy:
  requested_mode: efficient | fast | deterministic | exploratory
  max_usd_per_run
  max_tokens
  max_duration_ms
  max_cognitive_units
  max_branches
  max_tool_calls
  preferred_models
  disallowed_models
  allow_parallel
  priority: cost | latency | quality
  replay_strictness: strict | bounded | relaxed
```

---

### Policy Semantics

Cost policies enable:

- hard limits (e.g. "never exceed $0.10 per run")
- soft preferences (e.g. "prefer cheaper models unless confidence drops")
- execution shaping (e.g. "disable deep refinement under budget pressure")

---

### Interaction with Execution

During runtime:

- policies are evaluated continuously
- cost accumulation is tracked against constraints
- agents may adapt behavior:
  - switch models
  - terminate early
  - reduce branching
  - reuse memory (ObsMem)

---

### Policy Enforcement

Violations must be:

- detected deterministically
- recorded in trace
- reviewable post-run

Possible enforcement actions:

- terminate execution
- fallback to cheaper strategy
- escalate to higher-level agent
- log as policy violation event

Trace should record:

- the requested policy
- any policy adjustments or violations
- the realized execution envelope
- the realized cost vector

## Runtime Surface

The current owned surface is:

- `adl::chronosense::ExecutionPolicyCostModelContract`
- `adl::chronosense::ExecutionPolicySchema`
- `adl::chronosense::ExecutionRealizationSchema`
- `adl::chronosense::CostVectorSchema`
- `adl::chronosense::CostPolicyContract`
- `adl::chronosense::CostAnchorContract`
- `adl identity cost`

This bounded surface is intentionally limited to:

- requested execution posture
- realized execution envelope
- realized cost vector
- trace-anchored attribution
- reviewer-facing comparison rules

It does not yet implement:

- dynamic runtime policy enforcement
- enterprise pricing catalogs
- instinct/governance integration

---

### Strategic Importance

This completes the model:

- cost measurement → visibility
- cost optimization → efficiency
- cost policy → **control**

Without policies, cost-aware systems still drift.

With policies, ADL enables:

> **bounded, predictable, and governable cognition**

---

## Future Work

- cost-based planning
- budget-constrained agents
- market-based multi-agent optimization
- pricing-aware scheduling
- SLA-driven cognition

---

## Summary

The ADL cost model turns:

- time → measurable
- execution → traceable
- cognition → optimizable

Into:

> **a system where intelligence has a price, and that price can be reasoned about.**

And where that price is interpretable in light of the execution policy that produced it.

---

## Status

- Milestone: v0.87+ (initial grounding)
- Future expansion: v0.9x (optimization + economics layer)
