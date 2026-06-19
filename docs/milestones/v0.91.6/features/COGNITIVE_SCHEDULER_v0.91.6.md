# Cognitive Scheduler v1

## Status

Planned implementation for `v0.91.6`.

## Feature Role

Cognitive Scheduler v1 is the first dedicated ADL scheduler substrate planned for scarce cognitive resources.

It turns the cognitive economics and scheduler concept drafts into a bounded implementation path for deciding which lane should handle a unit of work:

- local cognition
- cheap remote cognition
- premium cognition
- governor cognition
- delayed execution

The feature exists because Current ADL execution evidence suggests C-SDLC is moving from issue-centric execution toward sprint-centric execution. Once issue creation, card generation, review support, and workflow paperwork become faster, the bottleneck shifts to premium model capacity, validation-tail latency, and human governor attention.

## Source Inputs

- Operator-local input consumed into the tracked plan: `.adl/docs/TBD/ADL_COGNITIVE_ECONOMICS.md`
- Operator-local input consumed into the tracked plan: `.adl/docs/TBD/ADL_COGNITIVE_SCHEDULER_v1.md`
- Tracked TBD plan: `docs/TBD/cognitive_scheduler/COGNITIVE_SCHEDULER_V1_PLAN.md`
- `#4105` scheduler umbrella
- `#4106` scheduler economics inputs
- `#4107` scheduler implementation

## Existing Scheduler-Adjacent Surfaces

ADL already has scheduler-adjacent code, but not a dedicated Cognitive Scheduler v1 implementation. The implementation issue should coordinate with:

- `adl/src/execute/runner.rs` for execution max-concurrency policy.
- `adl/src/trace/mod.rs` and `adl/src/trace/store.rs` for scheduler policy trace events.
- `adl/src/runtime_v2/governed_episode/` for existing bounded scheduling-decision artifacts.
- `adl/src/plan.rs` and `adl/src/execution_plan.rs` for planning/execution-plan concepts.

The preferred v1 home is a dedicated `adl/src/scheduler/` module, unless implementation review finds that extending an existing module is simpler and cleaner.

## Problem Statement

ADL can now generate more useful work than a single premium cognition lane should execute.

That is a good problem. It means the lifecycle is compressing. But it also means scheduling must become explicit.

Without a scheduler, ADL risks:

- wasting premium model windows on low-risk paperwork
- hiding validation-tail cost until too late
- overloading the human governor with avoidable decisions
- treating cheap generation as cheap even when review cost is high
- leaving provider and local-model capacity invisible

Scheduler v1 addresses this by turning work allocation into an auditable decision artifact.

## v1 Scope

In scope:

- schedulable work-item input model
- economics input model
- deterministic lane selection
- scheduling decision artifact
- explanation fields
- fixture-driven proof
- focused tests
- runbook/documentation

Out of scope:

- autonomous sprint conductor
- live GitHub mutation
- live provider/model calls
- automatic purchasing or overage
- market/bidding behavior
- full PVF execution
- measured speedup or cost-reduction claims

## Execution Lanes

| Lane | Purpose | Typical Work |
| --- | --- | --- |
| `LOCAL` | Cheapest available cognition, usually quota-independent | card checks, status summaries, routine docs, checklist support |
| `CHEAP_REMOTE` | Inexpensive cloud or commodity model work | first-pass review, decomposition, docs, test generation |
| `PREMIUM` | Scarce frontier/code execution | implementation, hard debugging, PR repair, security-sensitive code |
| `GOVERNOR` | Human/founder/architect judgment | irreversible choices, release authority, strategic direction |
| `DELAYED` | Wait for quota/capacity instead of wasting premium resources | low-urgency work during constrained windows |

## Decision Inputs

Scheduler v1 should account for:

- task type
- dependencies
- urgency
- risk
- expected value
- estimated effort
- validation burden
- coordination cost
- parallelism potential
- premium capacity pressure
- governor attention pressure
- manual override

## Decision Output

Scheduler v1 should emit a first-class artifact that records:

- selected lane
- alternatives considered
- rejection reasons
- score/explanation breakdown
- dependency status
- manual override status
- confidence

The output must be deterministic for identical inputs.

## Relationship To PVF

PVF is part of scheduling because validation-tail latency can dominate the actual cost of a task.

Scheduler v1 should not only ask whether a task is easy to implement. It should ask whether the task is cheap to prove, cheap to review, and safe to schedule at the current point in the sprint.

## Relationship To Provider Work

Scheduler v1 should not directly choose live providers by hidden heuristics.

Provider/profile and model-suitability work can supply future inputs, but v1 should remain fixture-driven and deterministic unless provider state is explicitly passed in.

## Dependency Note: `schedules` Crate

The `schedules` crate is a candidate dependency for timed execution, recurring watchers, delayed work, or SEP cadence. It is not a substitute for Cognitive Scheduler v1's lane-selection and economics decision model.

The v1 implementation should evaluate `schedules` only if timed execution is part of the accepted `#4107` scope. The default should remain a deterministic, fixture-driven ADL scheduler with no live timing dependency.

## Relationship To SEP And Sprint Conduction

Scheduler v1 is a planning primitive for future SEP and sprint-conductor work.

It does not conduct the sprint by itself. It helps explain how a sprint should allocate work across lanes.

## Issue Plan

| Issue | Role |
| --- | --- |
| `#4105` | Umbrella and planning surface |
| `#4106` | Scheduler economics input model |
| `#4107` | Scheduler v1 implementation and proof |

Recommended order:

1. Execute `#4106` to define the economics input model.
2. Execute `#4107` to build Scheduler v1 and prove deterministic scheduling.
3. Close `#4105` only after both children land and this feature doc reflects implementation truth.

## Acceptance Bar

The feature is ready when:

- Scheduler v1 can load explicit work-item inputs.
- Scheduler v1 can produce deterministic lane decisions.
- Decisions include explanation fields.
- Fixtures cover local, cheap remote, premium, governor, delayed, and blocked/dependency cases.
- Focused tests prove ordering and failure behavior.
- Documentation states how to use the scheduler and what it does not claim.

## Non-Claims

This feature does not claim:

- measured sprint acceleration
- measured cost reduction
- automatic agent orchestration
- replacement of human review
- live provider capacity visibility
- complete cognitive economics

## Review Notes

The design deliberately starts small. The point of v1 is to make scheduling explicit and inspectable, not to invent a hidden optimizer.
