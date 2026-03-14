# Adaptive Execution Engine (AEE)

**Status:** Implemented as bounded v0.8 runtime surfaces (full adaptive autonomy deferred)
**Milestone:** v0.8 bounded AEE v1; v0.85/v0.9+ advanced autonomy

## Purpose

This document defines the **repository-truth AEE surface** currently present in ADL v0.8.

In v0.8, AEE is a bounded deterministic execution/retry surface built from existing runtime controls. It is **not** a full autonomous strategy-learning engine.

Canonical scope boundary: `BOUNDED_AEE_V1_SCOPE_V0.8.md`.

## Implemented in v0.8

The following runtime surfaces exist today:

1. **Deterministic step execution loop**
   - Sequential and bounded-concurrent step scheduling with stable step ordering semantics.
   - Runtime surfaces: `swarm/src/execute/mod.rs`, `swarm/src/execute/runner.rs`.

2. **Bounded retry surface (`step.retry.max_attempts`)**
   - Per-step bounded retry count with deterministic attempt progression.
   - Runtime surfaces:
     - `swarm/src/adl.rs` (schema/validation for retry fields)
     - `swarm/src/execute/mod.rs` and `swarm/src/execute/runner.rs` (retry loop and attempt handling)

3. **Failure handling policy (`on_error: continue`)**
   - Explicit policy-controlled continuation behavior after failures.
   - Runtime surfaces: `swarm/src/execute/mod.rs`, `swarm/src/execute/runner.rs`.

4. **Retryability classification hook**
   - Retry path selection depends on deterministic retryable vs non-retryable classification.
   - Runtime surface: `swarm/src/provider.rs` (`is_retryable_error`).

5. **Attempt/evidence accounting surfaces**
   - Attempt counts and outcome surfaces emitted via run-status/evidence artifacts.
   - Runtime/tests surfaces:
     - `swarm/src/execute/mod.rs`
     - `swarm/tests/execute_tests.rs`

## Strategy-loop hook boundary (v0.8)

v0.8 exposes **hook points**, not a full strategy engine.

Implemented hook boundary:
- classify outcome
- decide retry continuation under explicit bounds/policy
- record attempt lifecycle and deterministic attempt counts

Deferred beyond v0.8:
- autonomous long-horizon strategy search
- online policy learning
- self-modifying strategy systems

## Determinism contract

For identical workflow inputs, policy, and provider responses:

- retry attempt ordering is deterministic,
- max-attempt bounds are deterministic,
- continuation behavior (`on_error`) is deterministic,
- retryability branch selection uses explicit, inspectable rules.

AEE v1 does **not** rely on hidden adaptive state.

## Canonical runtime entry points

AEE behavior is exercised through standard workflow execution surfaces:

- `adl --run <workflow>`
- workflow step fields:
  - `retry.max_attempts`
  - `on_error: continue`

Primary code entrypoints:

- `swarm/src/execute/mod.rs`
- `swarm/src/execute/runner.rs`
- `swarm/src/adl.rs`
- `swarm/src/provider.rs`

Validation coverage surface:

- `swarm/tests/execute_tests.rs`

## Security and policy boundaries

AEE v1 must remain policy-bounded:

- no implicit permission escalation,
- no hidden capability expansion,
- no secret/prompt/tool-arg leakage through AEE docs/artifacts,
- no absolute host path references in persisted milestone artifacts.

## Relationship to Gödel and ObsMem

In v0.8, AEE supplies bounded execution/retry primitives that Gödel/ObsMem docs can reference as deterministic runtime substrate.

It does **not** imply a fully autonomous Gödel runtime in this milestone.

## Future work (explicitly deferred)

Deferred to future milestones (v0.85/v0.9+):

- adaptive policy learning loops,
- richer strategy synthesis,
- autonomous multi-run optimization,
- advanced memory-driven strategy selection.

## Relationship to Sticktoittiveness Decomposition

`STICKTOITTIVENESS.md` is the implementation-slice breakdown for future retry
expansion work. It should be used when opening or prioritizing bounded follow-on
issues rather than treating "Sticktoittiveness" as one monolithic feature.

## Summary

ADL v0.8 includes a real, bounded AEE surface centered on deterministic retries, explicit policy hooks, and inspectable runtime behavior. Claims of full adaptive autonomy are deferred.
