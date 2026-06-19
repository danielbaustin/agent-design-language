# Cognitive Scheduler v1 Plan

## Status

Planned for `v0.91.6` implementation.

## Source Inputs

- Operator-local source: `.adl/docs/TBD/ADL_COGNITIVE_ECONOMICS.md`
- Operator-local source: `.adl/docs/TBD/ADL_COGNITIVE_SCHEDULER_v1.md`
- Tracked review surface: this document
- `#4105` scheduler umbrella
- `#4106` scheduler economics input model
- `#4107` Cognitive Scheduler v1 implementation

## Executive Summary

Cognitive Scheduler v1 is the first dedicated ADL scheduler plan for scarce cognitive resources.

The scheduler exists because C-SDLC has compressed issue creation, lifecycle paperwork, and sprint coordination enough that the bottleneck has moved. The next constraint is no longer only human coordination. It is premium cognition availability, provider quota, context-window capacity, validation burden, and governor attention.

The v1 goal is intentionally narrow:

- define schedulable work items
- define cognitive execution lanes
- define economics inputs
- rank or assign work to lanes deterministically
- record the scheduling decision as an artifact
- explain the decision
- keep premium cognition visible and conserved

It is not a market, bidding system, autonomous sprint conductor, or provider-purchasing agent.

## Existing Scheduler-Adjacent Surfaces

ADL does not currently have a dedicated `schedules.rs` module. Scheduler v1 should not pretend the repository is blank. It should coordinate with existing scheduler-adjacent surfaces:

- `adl/src/execute/runner.rs` owns execution max-concurrency policy and emits scheduler policy trace events.
- `adl/src/trace/mod.rs` and `adl/src/trace/store.rs` already represent `SchedulerPolicy` trace records.
- `adl/src/runtime_v2/governed_episode/` has bounded resource-pressure scheduling-decision artifacts for Runtime v2 demos.
- `adl/src/plan.rs` and `adl/src/execution_plan.rs` already define planning/execution-plan surfaces.

Recommended v1 code shape remains a dedicated `adl/src/scheduler/` module, but it should treat those existing surfaces as integration boundaries rather than inventing a parallel scheduling vocabulary.

## Core Thesis

C-SDLC pushes ADL up the Amdahl curve by decomposing work and parallelizing execution. That success exposes the next serial fraction: scarce cognition and scarce governor attention.

Cognitive Scheduler v1 is the mechanism that makes that new bottleneck explicit.

```text
C-SDLC = structured work demand
SCR / providers / humans = cognitive labor supply
Cognitive Scheduler = allocation mechanism
```

## v1 Completion Bar

Scheduler v1 is complete when ADL has:

1. A typed or schema-valid work-item input shape.
2. A bounded economics input model.
3. At least four execution lanes.
4. A deterministic scheduler command or library surface.
5. Fixture-driven proof covering serial blockers, parallelizable work, docs/review work, high-risk code work, and governor-only decisions.
6. A scheduling decision artifact with explanation fields.
7. Focused tests for deterministic ordering, dependency blocking, lane selection, and malformed input handling.
8. A runbook explaining how to use the scheduler and what it does not claim.

## Execution Lanes

Scheduler v1 should support these lanes first:

| Lane | Purpose | Typical work |
| --- | --- | --- |
| `LOCAL` | cheapest available cognition, quota-independent where possible | card checks, status summaries, routine docs, checklist support |
| `CHEAP_REMOTE` | inexpensive cloud or commodity model work | first-pass review, decomposition, docs, test generation |
| `PREMIUM` | scarce frontier/code execution | implementation, hard debugging, PR repair, security-sensitive code |
| `GOVERNOR` | human/founder/architect judgment | irreversible choices, release authority, strategic direction |
| `DELAYED` | wait for quota/capacity instead of wasting premium resources | low-urgency work during constrained windows |

## Scheduler Inputs

The first input shape should be boring and explicit.

```yaml
task_id: string
task_type: issue_card | planning | documentation | review | test_generation | implementation | refactor | security_review | release_gate | architecture
risk_level: low | medium | high | critical
urgency: low | normal | high | immediate
dependencies:
  - task_id: string
required_capabilities:
  - string
context_scope:
  repo_paths: []
  expected_files: []
  max_context_budget: optional
quality_bar: draft | reviewable | merge_ready | release_ready
estimated_effort: small | medium | large
estimated_validation_cost: low | medium | high
estimated_coordination_cost: low | medium | high
expected_value: low | medium | high | critical
parallelism_potential: blocked | serial | parallelizable | highly_parallelizable
human_required: false
manual_override: optional
```

## Economics Inputs

`#4106` should produce or validate the economics input model consumed by `#4107`.

Minimum v1 fields:

- estimated effort
- estimated validation cost
- estimated coordination cost
- risk
- expected value
- urgency
- dependency posture
- parallelism potential
- premium capacity pressure
- governor attention pressure

The key invariant is that cost means lifecycle cost, not just token cost.

Cheap generation that creates expensive review is not cheap.

## Scheduler Outputs

Scheduler v1 should emit an auditable decision artifact.

```yaml
scheduler_decision:
  schema: adl.scheduler.decision.v1
  task_id: string
  selected_lane: LOCAL | CHEAP_REMOTE | PREMIUM | GOVERNOR | DELAYED
  selected_profile_ref: optional
  alternatives_considered:
    - lane: string
      disposition: rejected | fallback | equivalent
      reason: string
  reason: string
  score_breakdown:
    effort: string
    validation_cost: string
    coordination_cost: string
    risk: string
    expected_value: string
    urgency: string
    premium_capacity_pressure: string
    governor_attention_pressure: string
  dependency_status: clear | blocked
  manual_override:
    present: boolean
    reason: optional
  confidence: low | medium | high
```

The artifact must answer:

- why this route was selected
- why alternatives were rejected
- what policy was applied
- whether the decision was automatic or overridden

## Determinism Rules

Scheduler v1 must be deterministic for the same inputs.

Required tie-breakers:

1. Dependency status.
2. Risk and human-required gates.
3. Urgency.
4. Expected value.
5. Validation burden.
6. Premium capacity conservation.
7. Stable task id ordering.

The scheduler should not use ambient provider state, current open PRs, live quota telemetry, or local shell state unless the input explicitly provides that state.

## PVF And Validation Awareness

PVF matters because validation-tail latency is now one of the biggest scheduling costs.

Scheduler v1 should model validation burden as a first-class input:

- docs-only proof
- focused tooling proof
- runtime/code proof
- release/full proof
- security/adversarial proof

A task with short implementation time and long validation time may be a poor candidate for late-sprint execution.

## Governor Attention

Governor attention is part of the economics model.

Scheduler v1 should distinguish:

- tasks that can auto-accept through contract proof
- tasks that require review support
- tasks that require explicit human decision
- tasks that should wait rather than interrupt the governor

This prevents the scheduler from optimizing tokens while increasing the true serial fraction.

## Issue Wave

### `#4105`: Umbrella And Planning Surface

Purpose:

- own this plan
- keep the v0.91.6 scheduler issue wave coherent
- close only after `#4106` and `#4107` have truthful closeout

### `#4106`: Scheduler Economics Inputs

Purpose:

- implement the economics input shape
- provide examples/fixtures
- document included/deferred economics concepts
- avoid claiming perfect cost prediction

### `#4107`: Scheduler v1 Implementation

Purpose:

- implement scheduler v1 command or library surface
- consume `#4106` inputs or compatible fixtures
- emit deterministic decision artifacts
- include tests and runbook

## External Crate Note: `schedules`

The Rust crate `schedules` should be evaluated, not assumed. It is a time-based callback scheduler for intervals, calendar schedules, manual execution, async execution, jitter, backoff, and persisted scheduler state. That is useful for recurring watchers, SEP cadence, delayed work, or retry-style execution loops.

Cognitive Scheduler v1 is different: it assigns work to cognitive lanes based on economics, risk, validation burden, dependencies, and governor attention. That decision layer should remain ADL-owned. If `schedules` is adopted, it should support timed execution around the scheduler, not replace the scheduler's decision model.

Initial recommendation: do not add `schedules` as a dependency in `#4106`. In `#4107`, evaluate it only if the implementation includes delayed execution, recurring watchers, or timed SEP checks. Otherwise keep v1 dependency-free and fixture-driven.

## Recommended Implementation Shape

Preferred small Rust surface:

```text
adl/src/scheduler/
  mod.rs
  economics.rs
  work_item.rs
  decision.rs
  policy.rs
  tests.rs
```

Possible command surface:

```text
adl tooling cognitive-scheduler plan --input fixtures/scheduler/work_items.yaml --policy fixtures/scheduler/policy.yaml --json
```

If the command routing is too large for v1, use a library plus focused fixture tests first, then add CLI in a follow-on.

## Fixture Set

Create fixtures for:

1. Low-risk docs task routed to `LOCAL`.
2. First-pass review task routed to `CHEAP_REMOTE`.
3. High-risk implementation routed to `PREMIUM`.
4. Architecture/release decision routed to `GOVERNOR`.
5. Low-urgency task routed to `DELAYED` under capacity pressure.
6. Blocked task held because dependency is not complete.

## Validation Plan

Focused proof is enough for `#4105`:

- Markdown path/reference check for the plan and feature doc.
- No broad Rust validation unless `#4105` touches code.

For `#4106` and `#4107`:

- `cargo fmt --manifest-path adl/Cargo.toml --all --check`
- focused scheduler tests
- fixture parse tests
- malformed input tests
- stdout/stderr contract proof if a CLI is added

## Non-Claims

Scheduler v1 does not claim:

- autonomous sprint conduction
- live provider quota introspection
- model suitability proof
- dynamic purchasing or overage decisions
- market behavior
- full PVF execution
- measured speedup
- measured cost reduction

## Open Questions

- Should the first CLI live under `adl tooling cognitive-scheduler` or a dedicated `adl scheduler` command?
- Should economics inputs be YAML-only at first, or Rust structs plus JSON/YAML fixtures?
- Should provider-profile refs be accepted in v1, or deferred until provider suitability work lands?
- Should sprint-level summary output be part of v1 or the first follow-on?

## Recommended Next Step

Execute `#4106` first, then `#4107`.

`#4105` should remain open as the umbrella until both children land and the scheduler feature doc is updated with the implemented truth.
