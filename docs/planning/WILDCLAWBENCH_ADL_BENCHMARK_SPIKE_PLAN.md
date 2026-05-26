# WildClawBench ADL Benchmark Spike Plan

## Status

Tracked v0.91.4 planning source for the WildClawBench benchmark spike sidecar.

This document promotes the relevant execution plan from the local TBD note into
tracked repo truth so the v0.91.4 issue wave does not depend on local-only
`.adl` planning state.

## Purpose

WildClawBench is an external pressure test for long-horizon agents in native
runtimes. ADL should use it to evaluate substrate behavior: runtime harness,
tool contracts, traceability, recovery, governance, and diagnosability.

The immediate goal is not to claim benchmark victory. The immediate goal is to
run a small representative spike, learn where ADL succeeds or fails, and record
evidence about whether ADL improves long-horizon execution as a substrate.

## Claim Boundary

This sidecar may support future evaluation work, but it is not a v0.91.4
release gate and must not be framed as a benchmark-win claim.

Allowed claims:

- ADL is testing WildClawBench as an external long-horizon substrate pressure
  test.
- Small-subset failures are useful if they produce better trace and failure
  classification.
- UTS and ACC should be evaluated separately.

Disallowed claims:

- ADL wins WildClawBench.
- ADL proves general intelligence.
- A small spike establishes model or runtime superiority.
- WildClawBench completion is required for C-SDLC v0.91.4 release.

## Hypotheses

- ADL may improve long-horizon completion through structured planning, retry,
  tracing, and state management.
- AEE may improve recovery behavior after failed commands, missing dependencies,
  timeouts, or repeated ineffective actions.
- Trace/replay may improve diagnosability even when task score does not
  improve.
- UTS may improve tool interoperability while ACC governs authority, policy,
  auditability, and runtime expectations.

## Spike Shape

The spike uses one umbrella plus four ordered child issues:

1. `WC-PRE-01`: setup and upstream smoke baseline.
2. `WC-PRE-02`: ADL wrapper trace comparison.
3. `WC-PRE-03`: UTS-only and UTS+ACC comparison setup.
4. `WC-PRE-04`: results taxonomy and handoff.

## Initial Task Subset

Do not start with the full benchmark.

The initial subset should be small and representative:

- one pure text task
- one multimodal task, if local environment supports it
- one coding or file-manipulation task
- one task likely to involve environment setup
- one safety-sensitive or policy-sensitive task
- one task with enough steps to exercise recovery behavior

Selection should be representative, reproducible, and explicitly not
cherry-picked for ADL.

## Execution Stages

### Stage 0: Upstream Reconnaissance

- Clone or update WildClawBench.
- Review task format and harness invocation model.
- Identify how existing agents are integrated.
- Record Docker/runtime prerequisites.

### Stage 1: Upstream Smoke Baseline

- Run one upstream harness on a small subset.
- Capture logs and grading output.
- Confirm Docker task isolation.
- Record setup friction and environment mismatches.

### Stage 2: ADL Wrapper Run

- Run at least one selected task through an ADL-controlled wrapper.
- Preserve upstream grading where possible.
- Capture ADL trace artifacts.
- Do not optimize prompts or tune ADL for score.

### Stage 3: Trace Comparison

Compare upstream logs with ADL traces.

Questions:

- Does ADL explain failures better?
- Are tool actions reconstructable?
- Are environment mutations visible?
- Can a reviewer tell what happened without rerunning the task?

### Stage 4: UTS-Only And UTS+ACC Comparison

For the same subset, document or run:

- UTS-only tool descriptions and adapter path.
- UTS+ACC capability and policy boundaries.
- Tool-call validity, capability denials, policy blocks, task success, runtime
  cost, wall-clock time, and trace quality.

Keep UTS and ACC separate:

- UTS standardizes tool shape.
- ACC governs tool authority.

### Stage 5: Scale Decision

Scale only if:

- smoke runs are reproducible
- task logs are captured consistently
- ADL traces are complete enough to analyze
- UTS-only and UTS+ACC configurations are clearly separated
- failures can be classified

## Metrics

Capture upstream-compatible metrics where available:

- task success score
- category score
- wall-clock time
- token cost
- tool-call count
- final grading result
- task-level logs

Capture ADL-specific metrics where available:

- workflow steps
- retries
- repeated command detection
- policy checkpoints
- objections raised and resolved
- capability denials
- trace completeness
- replayability status
- failure classification
- signed artifact availability, when supported

## Failure Taxonomy

Classify failures into stable categories:

- model reasoning failure
- tool schema failure
- tool adapter failure
- dependency/environment failure
- timeout
- context drift
- repeated ineffective action
- policy block
- incorrect policy block
- insufficient capability grant
- excessive capability grant
- grading mismatch
- benchmark incompatibility
- ADL runtime bug

## Deliverables

- `WILDCLAW_SETUP_NOTES.md`
- `WILDCLAW_ADL_ADAPTER_DESIGN.md`
- `WILDCLAW_EXPERIMENT_MATRIX.md`
- `WILDCLAW_RESULTS.md`
- `WILDCLAW_FAILURE_TAXONOMY.md`
- `WILDCLAW_UTS_ACC_ANALYSIS.md`

These deliverables may be produced by the sidecar issues or by later follow-on
work if the spike records a truthful blocker.

## Exit Bar

The sidecar is complete when it records either:

- a small benchmark spike result with upstream baseline, ADL wrapper evidence,
  UTS/ACC comparison notes, and failure taxonomy; or
- a truthful blocked handoff with enough setup evidence to route the next
  attempt.

In both cases, the sidecar must state whether to scale, defer, or open a
dedicated future evaluation lane.

