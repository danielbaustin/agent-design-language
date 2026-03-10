# Adaptive Execution Engine (AEE)

**Status:** Planning (bounded v1 scope in v0.8; broader autonomy deferred)
**Milestone:** v0.8 bounded AEE v1; v0.9+ advanced autonomy

## Overview

The **Adaptive Execution Engine (AEE)** is a planned runtime subsystem in ADL for persistent, policy‑gated task execution. Its purpose is to allow agents to **continue working toward a goal despite failures**, using bounded strategies, deterministic replay, and explicit artifacts.

This capability provides the “sticktoitiveness” behavior commonly observed in advanced coding agents: when a task fails, the system attempts alternative strategies rather than terminating immediately.

Unlike most agent frameworks, the Adaptive Execution Engine is:

- **Provider‑agnostic** (works with any model or tool provider)
- **Deterministic by design** (replayable execution when strict mode is enabled)
- **Policy‑bounded** (no silent capability escalation)
- **Artifact‑driven** (all adaptation decisions are captured as trace artifacts)

Conceptually, the AEE serves as a deterministic control system that manages attempts, retries, and strategy changes. For the explicit v0.8 scope boundary, see `BOUNDED_AEE_V1_SCOPE_V0.8.md`.

---

# Core Principle

Most AI agents today operate using a single attempt model:

Task → Attempt → Failure → Abort

The Adaptive Execution Engine instead implements a persistent execution loop:

Task → Attempt → Failure → Strategy Selection → Retry → Convergence or Exhaustion

This model treats problem solving as a **bounded search process**, similar to how human experts approach complex tasks.

---

# Architectural Role in ADL

The Adaptive Execution Engine sits between:

- **Workflow execution** (ADL runtime)
- **Trace / replay infrastructure**
- **ObsMem operational memory**
- **Policy / trust envelopes**

It is responsible for coordinating retries, recording attempt histories, and ensuring all behavior remains deterministic and auditable.

High‑level architecture:

Model / Tool Provider
        ↓
Step Execution
        ↓
Adaptive Execution Engine
        ↓
Retry Controller + Strategy Engine
        ↓
Trace / Replay / ObsMem

---

# Key Capabilities

## 1. Retry Controller

A centralized controller wraps step execution.

```
run_with_retry_controller(step, context) → StepResult
```

Responsibilities:

- classify failures using stable error codes
- enforce retry policies
- coordinate strategy transitions
- emit attempt lifecycle trace events

The controller ensures retries are **bounded and policy‑compliant**.

---

## 2. Strategy Engine

When a step fails, the engine selects a retry strategy.

Initial v1 strategies include:

- `conservative_escalation_v1`
- `fix_and_verify_v1`

Strategies may modify:

- prompt profiles
- verification steps
- provider profiles (policy permitting)

All mutations must remain **deterministic under identical inputs**.

---

## 3. Determinism Modes

The engine supports two determinism modes.

### Strict Mode

Guarantees full replay reproducibility:

- deterministic retry planning
- deterministic artifact generation
- stable trace ordering

Strict mode is used for:

- debugging
- reproducibility
- CI validation

### Best‑Effort Mode

Allows limited adaptive behavior such as:

- provider escalation
- adaptive backoff

All decisions are still logged and policy‑gated.

---

## 4. Attempt Lifecycle Events

Every retry attempt generates structured trace events.

Examples:

- `AttemptStarted`
- `AttemptFailed`
- `RetryPlanned`
- `AttemptSucceeded`
- `RetryExhausted`

These events enable:

- deterministic replay
- debugging
- operational analytics

Trace events must never expose:

- secrets
- raw prompts
- absolute paths
- sensitive tool inputs

---

## 5. Verification Hooks

Strategies may invoke verification hooks before declaring success.

Examples:

- test suite execution
- schema validation
- linting
- artifact verification

This enables patterns such as:

Fix → Verify → Retry if verification fails

---

# Relationship to Replay and Trace Bundles

The Adaptive Execution Engine relies on the deterministic runtime infrastructure introduced in earlier ADL milestones:

- activation logs
- trace bundles
- replay‑sufficient execution artifacts

These components allow the system to reproduce entire retry sequences exactly.

Replay capability is essential for:

- debugging failed workflows
- validating retry strategies
- evaluating improvements proposed by Gödel agents

---

# Relationship to ObsMem

ObsMem provides long‑term operational memory built from execution traces.

The Adaptive Execution Engine consumes ObsMem in two ways:

1. **Failure pattern recall** – retrieving similar historical failures
2. **Strategy selection hints** – choosing strategies that succeeded previously

All memory usage remains provenance‑aware and deterministic.

---

# Relationship to Gödel Self‑Improvement

The Gödel subsystem builds on top of the Adaptive Execution Engine.

Execution failures become inputs to hypothesis generation:

Failure → Hypothesis → Evaluation → Commit

The Adaptive Execution Engine provides the **experimental substrate** required for these improvement loops:

- deterministic replay
- evaluation workflows
- bounded mutations
- artifact‑based results

This structure closely mirrors the problem‑solving process described by mathematician **Jacques Hadamard** in studies of mathematical invention:

Attempt → Impasse → Reorganization → Insight → Verification

In ADL terms:

Attempt → Failure → Strategy Mutation → Retry → Verification

## Connection to Hadamard and Bayesian reasoning

The Adaptive Execution Engine can also be understood through two historical lenses: the **Hadamard model of invention** and **Bayesian learning**.

### Hadamard: structured cycles of discovery

Mathematician Jacques Hadamard described mathematical invention as a recurring process:

Attempt → Impasse → Reorganization → Insight → Verification

This maps almost directly to the AEE loop:

Attempt → Failure → Strategy Mutation → Retry → Verification

In ADL the stages are made explicit and observable:

- **Attempt** → step execution
- **Impasse** → classified failure code
- **Reorganization** → strategy engine mutation
- **Insight** → successful retry path
- **Verification** → tests / schema / artifact validation

What Hadamard described psychologically becomes, in ADL, a **deterministic engineering structure**.

### Bayesian interpretation

The retry loop can also be interpreted as a Bayesian updating process.

Each attempt produces evidence:

```
prior strategy belief
        ↓
execute attempt
        ↓
failure / success evidence
        ↓
update strategy choice
```

ObsMem supplies the historical evidence base:

- past failures
- successful strategies
- provider/tool performance

The Strategy Engine then approximates Bayesian updating by selecting strategies that historically produced higher success rates for similar conditions.

While v1 uses rule‑based strategies, future versions may incorporate statistical priors derived from ObsMem.

### Relationship to Gödel loops

The Gödel subsystem extends this reasoning one level higher.

AEE operates at the **execution level**:

```
attempt → failure → retry
```

Gödel operates at the **system improvement level**:

```
failure patterns → hypothesis → experiment → evaluation → commit
```

In this sense:

- **AEE** provides the experimental substrate
- **ObsMem** provides the empirical evidence
- **Gödel** provides the hypothesis generator

Together they form a layered learning system:

```
execution retries (AEE)
        ↓
operational memory (ObsMem)
        ↓
hypothesis generation (Gödel)
```

This layered structure allows ADL to evolve behavior **without sacrificing determinism, replayability, or auditability**.

---

# Safety and Policy Boundaries

Adaptive behavior must **never violate the ADL security envelope**.

Constraints include:

- no automatic permission escalation
- no widening of sandbox boundaries
- provider switches must be policy‑allowed
- all mutations must be recorded as artifacts

If a strategy attempts a prohibited mutation, the engine must produce a deterministic failure.

---

# Why This Matters

Traditional agent frameworks rely heavily on model intelligence.

ADL instead emphasizes **execution architecture**.

The Adaptive Execution Engine demonstrates that many capabilities associated with advanced agents arise from:

- persistent execution loops
- deterministic replay
- structured retries
- artifact‑based learning

This architectural approach enables provider‑agnostic agents to exhibit robust problem‑solving behavior without requiring larger models.

---

# Future Extensions

Later milestones may extend the Adaptive Execution Engine with:

- ML‑assisted strategy selection
- adaptive budgeting
- distributed retry orchestration
- deeper ObsMem integration

All such extensions must preserve the fundamental ADL guarantees:

- determinism (where declared)
- auditability
- security boundaries
- artifact‑driven learning

---

# Summary

The Adaptive Execution Engine transforms ADL from a simple workflow runner into a **persistent problem‑solving system**.

It introduces:

- bounded retry loops
- strategy‑driven execution
- deterministic traceability
- policy‑safe adaptation

Together with Replay, ObsMem, and the Gödel subsystem, it forms the foundation for **long‑running autonomous agent workflows** in ADL.
