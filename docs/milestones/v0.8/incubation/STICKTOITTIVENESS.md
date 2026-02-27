# [future][EPIC] Adaptive Execution Engine (Deterministic Auto-Retry + Strategy Loop)

**Milestone:** v0.8+ (likely v0.9)  
**Area:** runtime  
**Status:** Incubation / architectural planning  

---

## Summary

Introduce an **Adaptive Execution Engine (AEE)** for ADL:

A deterministic, policy-gated, traceable auto-retry system that enables bounded self-healing execution without weakening security, determinism, or audit guarantees.

This is ADL’s enterprise-grade version of “no-fail” execution — similar in spirit to codex.app’s retry behavior — but:

- Deterministic  
- Auditable  
- Replayable  
- Policy-governed  
- Security-preserving  

This document captures the architectural intent for v0.8+ and explicitly does **not** modify v0.7 behavior.

---

## Motivation

Today, ADL steps either:

- succeed
- fail
- retry up to a static `max_attempts`

What is missing:

> When a step fails, intelligently try another deterministic strategy until success or budget exhaustion.

Codex-like systems feel powerful because they:
- detect failure,
- adjust approach,
- try again,
- converge.

ADL can implement this in a structured and enterprise-safe way.

---

## Core Concept

### Runtime Control Flow

```
execute_step()
  → run_with_retry_controller()
      → attempt N
          → classify failure (stable error code)
          → evaluate retry policy
          → compute deterministic strategy mutation
          → emit trace events
          → attempt N+1
```

---

## Design Constraints

The Adaptive Execution Engine must be:

### 1) Deterministic

- No wall-clock randomness.
- No nondeterministic UUID generation.
- Stable `delegation_id` / `attempt_id`.
- Replay-safe event ordering.
- Stable serialization.

All retry decisions must be reproducible under identical inputs.

---

### 2) Policy-Governed

All retry mutations must pass through:

- Delegation Policy Surface (v0.7 foundation).
- Security envelope.
- Sandbox invariants.

No automatic permission escalation.

Retry may never bypass #490 guardrail principles.

---

### 3) Security-Preserving

Never mutate:

- Sandbox roots
- Filesystem permissions
- Network access rules
- Delegation allow/deny policies
- Security envelope invariants

Retry may only modify:

- Provider selection (if allowed)
- Provider profile (e.g., strict → escalated)
- Prompt profile
- Tool safety flags
- Retry metadata

Security posture must be monotonic or invariant — never weakened.

---

### 4) Auditable

Trace model must emit attempt lifecycle events:

- `StepAttemptStarted`
- `StepAttemptFailed`
- `RetryPlanned`
- `RetryDeniedByPolicy`
- `StepAttemptSucceeded`
- `StepRetryExhausted`

Events must not log secrets or raw prompt/tool arguments.

All retry decisions must be explainable after the fact.

---

## Scope (v1 of AEE)

### Retry Controller v1

Add a centralized runtime wrapper:

- `run_with_retry(step, context)`
- bounded by `max_attempts`
- deterministic strategy ordering
- explicit failure taxonomy
- policy-gated mutation surface

This replaces ad-hoc retry loops with a structured controller.

---

### Strategy Surface v1

Support a small number of named strategies:

- `conservative_escalation_v1`
- `fix_and_verify_v1`

Each strategy:

- Is deterministic.
- Has documented state transitions.
- Is testable.
- Is bounded.

No ML-driven adaptation in v1.

---

### Failure Taxonomy Expansion

Introduce stable error codes such as:

- `tool_exit_nonzero`
- `compile_error`
- `test_failure`
- `provider_timeout`
- `provider_rate_limited`
- `sandbox_denied`
- `policy_denied`
- `unknown_failure`

Retry decisions must be based on structured codes — never string matching.

---

## Configuration Surface (Draft)

Example YAML:

```
retry:
  max_attempts: 4
  retryable:
    - compile_error
    - test_failure
    - provider_timeout
  strategy: conservative_escalation_v1
  allow_provider_switch: true
```

Defaults must remain conservative and secure.

Overlay integration (future) must remain opt-in and audit-safe.

---

## Out of Scope (v1)

- ML-driven strategy adaptation
- Cross-run learning mutation
- Dynamic sandbox reconfiguration
- Complex multi-party approval workflows
- Distributed tracing integration
- Autonomous policy mutation

---

## Relationship to v0.7 Foundations

Builds on:

- Delegation Policy Surface v1 (#487)
- Delegation Trace Model v1 (#488)
- Resilience Surfaces (#491)
- Sandbox taxonomy (#502)
- Learning surfaces (#481–#486)

Must not weaken #490 guardrail invariants.

v0.7 provides the audit spine.
v0.8+ builds adaptive behavior on top of it.

---

## Acceptance Criteria (EPIC Completion)

- Centralized Retry Controller implemented
- Deterministic strategy engine
- Trace events integrated
- Policy gating enforced
- Deterministic unit + integration tests:
  - retry success path
  - retry exhaustion
  - policy-denied mutation
  - deterministic replay
- Documentation finalized under `docs/milestones/v0.8/`

---

## Strategic Value

This capability enables:

- Self-healing workflows
- Reliable demos
- Enterprise-grade autonomy
- Competitive differentiation vs CrewAI / AutoGen
- Foundation for controlled operational intelligence

This is a major runtime capability and must not be rushed.

---

## Positioning

v0.7 ships:

- Determinism
- Policy
- Trace
- Learning artifacts

v0.8+ adds:

- Controlled adaptive behavior

The architecture must preserve:

> Deterministic core. Adaptive layer on top.

---

## Guiding Principle

Codex feels powerful because it retries invisibly.

ADL will be stronger because it retries visibly, deterministically, and safely.

Finish v0.7 clean.
Build adaptation deliberately.
