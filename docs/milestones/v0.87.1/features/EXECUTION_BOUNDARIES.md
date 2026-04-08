# ADL Execution Boundaries

## Status
Active milestone feature for `v0.87.1` runtime completion

## Purpose

Execution boundaries define where `v0.87.1` requires control transfer to be explicit, observable, and reviewable.

For this milestone, a boundary is in scope when:
- control crosses a meaningful runtime handoff
- the handoff must be visible in trace output
- later trace, resilience, operator, and review work will depend on that visibility

This is a bounded runtime-control feature. It does not attempt to finalize long-term identity, chronosense, or full agency continuity semantics.

## Core Principle

No meaningful runtime control transition should remain implicit.

For `v0.87.1`, that means:
- boundary names are explicit
- trace records boundary crossings
- success, failure, pause, and workflow-call transitions are distinguishable

## In-Scope Boundaries

### 1. Runtime-init boundary

When execution context is first bound, the runtime crosses the `runtime_init` boundary.

Requirements:
- classify startup as fresh-start vs resume
- expose the transition in trace

Trace proof:
- `ExecutionBoundaryCrossed boundary=runtime_init state=fresh_start`
- `ExecutionBoundaryCrossed boundary=runtime_init state=resume`

### 2. Resume boundary

Resumed execution must be marked explicitly.

Requirements:
- resumed work must not be silently presented as a fresh run
- resume classification must be visible in trace

Trace proof:
- `ExecutionBoundaryCrossed boundary=resume state=entered`

### 3. Workflow-call boundary

Nested workflow calls are explicit control transfers.

Requirements:
- entering the called workflow is explicit
- exiting the called workflow is explicit
- success vs failure stays visible

Trace proof:
- `ExecutionBoundaryCrossed boundary=workflow_call state=entered`
- `CallEntered ...`
- `ExecutionBoundaryCrossed boundary=workflow_call state=success|failure`
- `CallExited ...`

### 4. Pause boundary

Pause is an execution boundary, not an accidental partial completion.

Requirements:
- a pause-capable run must emit an explicit pause boundary
- downstream surfaces can distinguish paused from completed

Trace proof:
- `ExecutionBoundaryCrossed boundary=pause state=entered`

### 5. Run-completion boundary

Final run disposition must be explicit.

Requirements:
- success and failure are explicit boundary outcomes
- completion is not inferred only from process termination

Trace proof:
- `ExecutionBoundaryCrossed boundary=run_completion state=success`
- `ExecutionBoundaryCrossed boundary=run_completion state=failure`

## Relationship to Existing Step and Delegation Trace

`v0.87.1` already had explicit lower-level execution trace such as:
- `StepStarted`
- `StepFinished`
- delegation policy evaluation
- delegation dispatch/result/completion

This feature does not replace those surfaces.

Instead, it adds the missing runtime-level boundary vocabulary above them so the runtime has:
- lifecycle phases
- runtime boundaries
- step/delegation trace

all in one coherent model.

## Validation Expectations

Boundary correctness for `v0.87.1` means:
- the in-scope boundaries emit deterministic trace lines
- the same inputs preserve the same boundary ordering
- workflow-call and completion transitions are observable
- pause and failure remain distinguishable from success

## Operator Inspection Consequence

Because boundary visibility is part of the runtime contract, the operator surface for `v0.87.1` must expose the canonical trace artifact directly. For this milestone that means operator-facing demos and docs should point reviewers to:
- `runtime_environment.json` for runtime-root truth
- `run_summary.json` and `run_status.json` for run disposition
- `logs/trace_v1.json` for explicit boundary evidence

The operator surface should not require a reviewer to discover boundary proof indirectly through unrelated helper artifacts.

## Out of Scope

This milestone does not attempt to finalize:
- distributed boundary semantics
- remote multi-host runtime governance
- full capability-policy architecture for all future substrates
- persistent identity semantics across machine restarts

Those can build later on the bounded runtime boundary surface established here.
