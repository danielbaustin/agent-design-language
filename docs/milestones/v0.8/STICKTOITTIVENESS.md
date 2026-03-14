# Sticktoittiveness Decomposition (v0.8)

**Status:** Decomposed into bounded implementation slices  
**Milestone:** v0.8 clarification artifact; future implementation work remains sliced  
**Area:** runtime / retry / adaptive execution

---

## Purpose

This document replaces the earlier "one big future subsystem" framing of
`Sticktoittiveness`.

Repository truth today is narrower:

- ADL already has bounded deterministic retry primitives.
- ADL does **not** have a full retry-strategy engine, persistent adaptive state,
  or autonomous self-healing subsystem.

The goal of this document is to break the concept into concrete slices that can
be implemented, reviewed, and prioritized independently.

---

## Current Repository Truth

The following retry/adaptive surfaces already exist:

1. **Retry attempt bound**
   - Workflow surface: `retry.max_attempts`
   - Runtime surfaces:
     - `swarm/src/adl.rs`
     - `swarm/src/execute/mod.rs`
     - `swarm/src/execute/runner.rs`

2. **Continuation policy**
   - Workflow surface: `on_error: continue`
   - Runtime surfaces:
     - `swarm/src/adl.rs`
     - `swarm/src/execute/mod.rs`
     - `swarm/src/execute/runner.rs`

3. **Retryable vs non-retryable classification**
   - Runtime surface:
     - `swarm/src/provider.rs`

4. **Deterministic retry accounting and review surfaces**
   - Runtime/tests surfaces:
     - `swarm/tests/execute_tests.rs`
     - `swarm/src/cli/mod.rs`
     - `swarm/src/learning_export.rs`

5. **Deterministic overlay application for retry budget changes**
   - Runtime/tests surfaces:
     - `swarm/src/overlay.rs`
     - `swarm/tests/execute_tests.rs`

So the repository already implements a **bounded retry substrate**. What is
missing is the broader multi-slice system that the old "Sticktoittiveness"
language implied.

---

## What Is Not Implemented

The following are **not** implemented as current v0.8 runtime behavior:

- deterministic backoff schedules beyond repeated bounded attempts
- named retry strategies with explicit state transitions
- persistent strategy state carried across runs
- runtime mutation of retry strategy based on prior outcomes
- closed-loop failure-classification feedback that rewrites execution policy
- autonomous "fix and retry" or patch-application behavior

Those belong in smaller slices rather than a single umbrella feature.

---

## Bounded Implementation Slices

### Slice 1: Retry Policy Semantics Baseline

**Purpose**

Make the current retry substrate explicit and reviewable as the baseline slice.

**Current status**

Partially implemented now.

**What it includes**

- `retry.max_attempts`
- `on_error: continue`
- deterministic attempt progression
- retry exhaustion behavior
- fail-fast default when retry is absent

**Runtime surfaces**

- `swarm/src/adl.rs`
- `swarm/src/execute/mod.rs`
- `swarm/src/execute/runner.rs`
- `swarm/tests/execute_tests.rs`
- `swarm/examples/v0-3-on-error-retry.adl.yaml`

**Why this is its own slice**

This is the stable base that later slices must build on rather than redefine.

---

### Slice 2: Failure Taxonomy and Retry Eligibility

**Purpose**

Separate "how many times may we retry?" from "what kinds of failures are even
retryable?"

**Current status**

Partially implemented now through provider-level retryable classification.

**What exists**

- deterministic `is_retryable_error` hook
- clear 4xx vs 5xx vs timeout behavior in provider tests

**What remains**

- stronger stable failure taxonomy across provider/runtime/delegation paths
- documentation that maps error classes to retry eligibility explicitly
- reviewable bounded list of retryable categories rather than ad-hoc expansion

**Primary runtime surfaces**

- `swarm/src/provider.rs`
- `swarm/tests/provider_tests.rs`
- `swarm/tests/execute_tests.rs`

**Why this is its own slice**

Eligibility rules can be tightened and tested without changing strategy
selection or persistence.

---

### Slice 3: Deterministic Retry Ordering and Schedule

**Purpose**

Define what happens between attempts in a deterministic way.

**Current status**

Mostly unimplemented beyond immediate repeated attempts.

**What exists**

- deterministic attempt count progression
- deterministic attempt ordering

**What remains**

- explicit statement of whether retries are immediate, zero-delay, or follow a
  bounded static schedule
- optional bounded schedule representation that is replay-friendly
- tests proving identical inputs choose identical retry schedule/order

**Likely runtime surfaces**

- `swarm/src/execute/runner.rs`
- `swarm/src/adl.rs`
- `swarm/tests/execute_tests.rs`

**Why this is its own slice**

Scheduling rules can be introduced without yet adding strategy mutation or
persistent state.

---

### Slice 4: Attempt and Strategy-State Persistence

**Purpose**

Persist enough structured data to explain retry behavior after the run.

**Current status**

Partially present through attempt counts and learning/export surfaces, but not
as a dedicated strategy-state subsystem.

**What exists**

- retry counts in score/learning artifacts
- run-status/evidence surfaces
- exported learning summaries

**What remains**

- explicit persisted record of selected retry strategy or retry-plan reason
- stable artifact field(s) for why attempt `N+1` was chosen
- deterministic trace/evidence linkage for retry planning

**Primary surfaces**

- `swarm/src/cli/mod.rs`
- `swarm/src/learning_export.rs`
- trace/run artifact surfaces

**Why this is its own slice**

Persistence can be improved independently of runtime strategy generation.

---

### Slice 5: Failure-Classification Feedback Loop

**Purpose**

Turn observed failures into bounded future recommendations rather than
immediately into autonomous runtime mutation.

**Current status**

Partially present as reporting/suggestion surfaces, not as closed-loop runtime
adaptation.

**What exists**

- retry counts in score summaries
- suggestion surfaces that can point to safer retry policy changes

**What remains**

- deterministic feedback from observed failure classes to proposed retry policy
  updates
- explicit issue/workflow-level artifact describing recommended retry-policy
  changes
- separation between recommendation and automatic application

**Primary surfaces**

- `swarm/src/cli/mod.rs`
- `swarm/src/learning_export.rs`
- `docs/milestones/v0.8/ADAPTIVE_EXECUTION_ENGINE.md`

**Why this is its own slice**

This keeps the first adaptive loop reviewable and non-autonomous.

---

## Recommended Ordering

Implement in this order:

1. **Retry Policy Semantics Baseline**
   - primarily documentation/truth-alignment and minor hardening
2. **Failure Taxonomy and Retry Eligibility**
   - safest runtime tightening with strong tests
3. **Deterministic Retry Ordering and Schedule**
   - bounded runtime behavior change
4. **Attempt and Strategy-State Persistence**
   - artifact/trace clarity
5. **Failure-Classification Feedback Loop**
   - recommendation surfaces before any automation

This ordering keeps the work incremental:

- semantics first
- classification second
- scheduling third
- persistence fourth
- adaptive feedback last

---

## Relationship to Existing v0.8 Docs

- `ADAPTIVE_EXECUTION_ENGINE.md` describes current bounded runtime truth.
- `BOUNDED_AEE_V1_SCOPE_V0.8.md` remains the canonical v0.8 scope boundary.
- This document decomposes the larger `Sticktoittiveness` idea into bounded
  follow-on slices.

This means:

- `ADAPTIVE_EXECUTION_ENGINE.md` answers "what exists now?"
- `BOUNDED_AEE_V1_SCOPE_V0.8.md` answers "what is in scope for v0.8?"
- `STICKTOITTIVENESS.md` now answers "how should the larger future idea be
  broken down?"

---

## Review Guidance

Reviewers should **not** read this document as a claim that ADL already ships a
full self-healing runtime.

Reviewers **should** read it as:

- a truthful decomposition of the larger concept,
- tied to current retry/runtime surfaces,
- with bounded next implementation slices.

---

## Bottom Line

`Sticktoittiveness` should no longer function as one giant future feature.

For current repository truth, it decomposes into:

1. retry policy semantics baseline
2. failure taxonomy and retry eligibility
3. deterministic retry ordering and schedule
4. attempt and strategy-state persistence
5. failure-classification feedback loop

That is a reviewable implementation plan. It is smaller, clearer, and better
aligned with the runtime surfaces ADL already has.
