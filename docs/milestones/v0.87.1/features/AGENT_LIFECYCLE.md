# Agent Lifecycle in the ADL Runtime

## Status
Active milestone feature for `v0.87.1` runtime completion

## Purpose

This document defines the bounded runtime lifecycle that `v0.87.1` must make explicit in code, trace output, and tests.

The lifecycle in scope for this milestone is a runtime control model:
- explicit phases
- explicit boundaries
- deterministic ordering
- trace visibility

This document does not claim:
- persistent identity across restarts
- chronosense
- full agency continuity

Those remain future-milestone work. `v0.87.1` only establishes the local runtime lifecycle substrate they may later build on.

## Authoritative Lifecycle Phases

The runtime lifecycle for `v0.87.1` has four explicit phases:

1. `init`
2. `execute`
3. `complete`
4. `teardown`

These phases are runtime phases, not philosophical states of agent being.

## Phase Semantics

### `init`

The runtime establishes the execution context before step work begins.

In scope:
- runtime root selection
- run-artifact root availability
- resume-vs-fresh-start classification
- execution-plan bring-up

Observable proof:
- `LifecyclePhaseEntered phase=init`
- `ExecutionBoundaryCrossed boundary=runtime_init ...`

### `execute`

The runtime performs bounded workflow execution.

In scope:
- sequential or concurrent step scheduling
- provider delegation
- workflow calls
- step-level trace emission
- pause eligibility

Observable proof:
- `LifecyclePhaseEntered phase=execute`
- existing step, delegation, and call trace events

### `complete`

The runtime has reached a successful bounded completion state.

In scope:
- all required execution work for the current run finished successfully
- success is explicit rather than inferred from process exit

Observable proof:
- `LifecyclePhaseEntered phase=complete`
- `ExecutionBoundaryCrossed boundary=run_completion state=success`

### `teardown`

The runtime exits the active execution phase and hands off to artifact/reporting surfaces.

In scope:
- post-execution closeout of the active run phase
- teardown after success
- teardown after failure
- teardown after pause

Observable proof:
- `LifecyclePhaseEntered phase=teardown`

## Explicit Runtime Boundaries

`v0.87.1` makes the following boundaries explicit:

### Runtime-init boundary

The runtime moves from unresolved entry state into an initialized execution context.

Observable proof:
- `ExecutionBoundaryCrossed boundary=runtime_init state=fresh_start`
- or `ExecutionBoundaryCrossed boundary=runtime_init state=resume`

### Resume boundary

Resumed work is explicitly marked as resumed rather than silently treated as a fresh run.

Observable proof:
- `ExecutionBoundaryCrossed boundary=resume state=entered`

### Workflow-call boundary

Cross-workflow control transfer is explicit.

Observable proof:
- `ExecutionBoundaryCrossed boundary=workflow_call state=entered`
- `CallEntered ...`
- `ExecutionBoundaryCrossed boundary=workflow_call state=success|failure`
- `CallExited ...`

### Pause boundary

Pause is a boundary, not an implicit partial success.

Observable proof:
- `ExecutionBoundaryCrossed boundary=pause state=entered`

### Run-completion boundary

Final success or failure is explicit.

Observable proof:
- `ExecutionBoundaryCrossed boundary=run_completion state=success`
- or `ExecutionBoundaryCrossed boundary=run_completion state=failure`

## Invariants

The bounded lifecycle must preserve:

- deterministic phase ordering for identical inputs
- no hidden phase transitions
- no hidden boundary crossings
- explicit trace visibility for lifecycle entry and completion boundaries
- replay-safe artifact behavior
- no secrets or absolute host-path leakage in trace output

## Out of Scope

This lifecycle document does not define:
- distributed lifecycle semantics
- identity persistence beyond the local bounded runtime
- temporal self-modeling or chronosense
- Shepherd preservation and repair policy
- richer agency-continuity semantics

Those belong to later `v0.87.1` and future milestone work.

## Code Surfaces

The authoritative implementation surfaces for this milestone are:
- `adl/src/execute/state/contracts.rs`
- `adl/src/execute/mod.rs`
- `adl/src/execute/runner.rs`
- `adl/src/trace.rs`

## Validation Expectations

Lifecycle correctness for `v0.87.1` means:
- the phases exist explicitly in code
- the major runtime boundaries emit trace events
- integration tests observe the bounded lifecycle ordering
- docs and runtime trace vocabulary use the same lifecycle terms

## Runtime State Consequence

The lifecycle surface must also make runtime persistence discipline inspectable.

In `v0.87.1`, the authoritative lifecycle-state proof surfaces are:

- `run_status.json`
  - `continuity_status`
  - `persistence_mode`
  - `cleanup_disposition`
  - `resume_guard`
- `pause_state.json` for recoverable interruption cases

This remains a bounded local runtime contract. It does not claim persistent identity, chronosense, or full agency continuity.
