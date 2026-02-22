# ADL v0.6 Design

## Metadata
- Milestone: v0.6
- Version: v0.6
- Status: Planning
- Owner: ADL core (Daniel + Codex-assisted implementation)
- Related issues:
  - #401–#411 (v0.6 umbrella WPs)
  - #409 (Coverage audit >80% per file)
  - #370, #371, #386 (remote security + signing follow-ups)

---

## Purpose

v0.6 moves ADL from a “deterministic execution engine with primitives” to a
structured, instrumented, policy-aware runtime foundation.

The goal is not to dramatically expand surface area, but to:
- Formalize patterns as first-class registry artifacts.
- Introduce minimal human-in-the-loop (HITL) control points.
- Enable streaming output and richer instrumentation.
- Harden determinism and scheduler semantics.
- Establish measurable quality gates (coverage audit).
- Produce a clean, demo-ready release.

v0.6 is an architectural consolidation and capability extension release.
It is not a distributed systems release and not a memory-integrated release.

---

## Problem Statement

ADL v0.5 ships:
- Deterministic execution planning.
- Bounded concurrency.
- Signing + canonicalization.
- Remote execution MVP with documented trust limits.

However, several gaps remain:

1. Patterns exist but are not yet treated as a coherent registry surface.
2. Delegation metadata exists conceptually but is not logged/structured at runtime.
3. Output is buffered; streaming semantics are not clearly defined.
4. Instrumentation is usable but not yet exportable for replay/diff workflows.
5. Human pause/resume semantics are not formally defined.
6. Quality bar is not enforced via a measurable coverage ratchet.

v0.6 addresses these without destabilizing the deterministic core.

---

## Goals

### 1. Pattern Registry + Compiler Expansion (WP-A, #401)
- Formalize a pattern registry abstraction.
- Ensure fork/join and linear patterns are byte-stable.
- Document registry structure and constraints.
- Keep patterns deterministic and declarative.

### 2. Minimal HITL Pause/Resume (WP-B, #402)
- Introduce an explicit pause state in execution lifecycle.
- Pause must be:
  - Deterministic.
  - Explicit in trace.
  - Resume-capable without hidden state.
- No background magic or autonomous mutation.

### 3. Streaming Output Semantics (WP-C, #403)
- Define streaming boundaries:
  - stdout (step output)
  - stderr (progress)
  - trace events
- Preserve deterministic step ordering.
- Streaming must not change final artifact determinism.

### 4. Provider Profiles (WP-D, #404)
- Define documented profiles for a curated set of models.
- Profiles are configuration/documentation-level constructs.
- No runtime auto-selection heuristics in v0.6.

### 5. Delegation Metadata (Log-Only) (WP-E, #405)
- Introduce structured delegation metadata in schema.
- Record metadata in trace.
- Do not enforce policy in v0.6 (policy engine deferred to v0.7).

### 6. Determinism + Scheduler Hardening (WP-F, #406)
- Clarify concurrency override semantics.
- Preserve lexicographic batching guarantees.
- Ensure max-concurrency invariants remain test-backed.

### 7. Instrumentation + Replay Diff + Graph Export (WP-G, #407)
- Export trace in structured format.
- Enable replay comparison/diff workflows.
- Provide graph export (DOT or Mermaid) for workflow visualization.

### 8. Demo Matrix + Integration Demos (WP-H, #408)
- Define canonical demos covering:
  - Concurrency
  - Signing
  - Remote execution
  - Pattern usage
  - Delegation metadata logging
- Ensure demos are deterministic and CI-verifiable.

### 9. Coverage Audit (>80% per file) (WP-H2, #409)
- Perform per-file coverage analysis.
- Raise coverage floor to >80% where practical.
- Identify intentional exclusions explicitly.

### 10. Docs + Review Pass (WP-I, #410)
- Align README, docs, milestone planning.
- Ensure threat-model and determinism invariants are explicit.

### 11. Release Ceremony (WP-J, #411)
- Tag.
- Release notes.
- Clean milestone checklist.

---

## Non-Goals

- No distributed cluster execution (#339).
- No checkpoint/recovery engine (#340).
- No advanced adaptive scheduler (#338).
- No ObsMem integration (#337).
- No runtime policy enforcement for delegation (v0.7).
- No public-facing hardened remote server (security envelope tracked in #370/#371).

---

## Scope

### In Scope
- Deterministic runtime enhancements.
- Streaming + instrumentation improvements.
- Registry formalization.
- Logging-only delegation metadata.
- Demo standardization.
- Coverage ratchet.

### Out of Scope
- Learning systems.
- Autonomous policy engines.
- Hidden adaptive behavior.
- Cross-node orchestration.

---

## Requirements

### Functional
- Pattern registry must compile deterministically.
- Pause/resume must preserve execution state integrity.
- Streaming must not reorder step completion artifacts.
- Delegation metadata must appear in trace.
- Graph export must reflect compiled execution plan.

### Non-Functional
- Deterministic behavior and reproducible outputs.
- Clear failure semantics and observability.
- >80% per-file test coverage target (WP-H2).
- No regression in existing CLI smoke tests.

---

## Proposed Design

### Architectural Overview

v0.6 maintains the same core runtime pipeline:

Schema → Resolve → Compile → Execute → Trace → Artifacts

Enhancements occur at:
- Compile phase (pattern registry formalization).
- Execute phase (pause/streaming semantics).
- Trace layer (delegation metadata + streaming events).
- Tooling layer (graph export + replay diff).

Core invariants remain:
- Deterministic plan construction.
- Deterministic step ordering.
- Bounded concurrency guarantees.

---

### Pattern Registry (WP-A)

- Patterns are declared and referenced symbolically.
- Registry maps pattern IDs → compile transforms.
- Compile output must remain byte-stable.
- No dynamic mutation of patterns at runtime.

Risk: registry abstraction drift.
Mitigation: keep registry thin and declarative.

---

### HITL Pause/Resume (WP-B)

Execution state machine extended with:

- Running
- Paused
- Completed
- Failed

Pause:
- Emitted in trace.
- Explicit barrier before next step.
- Resume must re-enter scheduler cleanly.

No background worker resumption or implicit timeouts.

---

### Streaming Semantics (WP-C)

Streaming applies to:
- Step output (stdout).
- Progress (stderr).
- Trace events.

Rules:
- Streaming may occur during execution.
- Final artifacts remain deterministic.
- Ordering guarantees apply to step lifecycle events.

---

### Delegation Metadata (WP-E)

Schema extension:
- Optional delegation block per step.

Runtime:
- Metadata logged into trace.
- No enforcement logic in v0.6.
- Enables v0.7 policy engine.

---

### Scheduler Hardening (WP-F)

Determinism contract for concurrent workflow runs:
- Ready-step selection is sorted lexicographically by full step id.
- Batches execute with a bounded cap and preserve deterministic step lifecycle ordering.
- Effective concurrency precedence is deterministic:
  1) workflow-local override (`run.workflow.max_concurrency` or `workflows.<id>.max_concurrency`)
  2) `run.defaults.max_concurrency`
  3) runtime default (`4`)

Intentionally unspecified:
- No fairness guarantees across equally ready steps beyond deterministic lexicographic ordering.
- No wall-clock-based scheduling policy and no adaptive concurrency in v0.6.

Validation:
- Override and bounded-cap semantics are regression tested under `swarm/tests/execute_tests.rs`.

---

### Instrumentation + Graph Export (WP-G)

- Structured trace output.
- Replay diff tool compares trace artifacts.
- Graph export format (Mermaid or DOT) generated from compiled plan.

---

## Risks and Mitigations

Risk: Streaming breaks determinism.
Mitigation: enforce ordering at lifecycle boundary.

Risk: Pause introduces inconsistent state.
Mitigation: explicit state transitions + test coverage.

Risk: Scope creep into v0.7 territory.
Mitigation: delegation remains log-only.

Risk: Coverage work stalls progress.
Mitigation: timebox audit and document exclusions.

Risk: Registry abstraction overcomplicates compiler.
Mitigation: keep compile transforms simple and pure.

---

## Alternatives Considered

### Alternative: Full delegation enforcement in v0.6
Tradeoff: increases risk and destabilizes runtime. Deferred to v0.7.

### Alternative: Adaptive scheduler in v0.6
Tradeoff: conflicts with determinism guarantees. Deferred.

### Alternative: Integrated memory (ObsMem) in v0.6
Tradeoff: coupling risk; violates separation principle. Deferred.

---

## Validation Plan

- All new behaviors covered by tests.
- Determinism tests pass across multiple runs.
- Streaming tests confirm ordering guarantees.
- Coverage report confirms >80% per file where practical.
- Demo matrix runs clean in CI.

Rollback:
- Feature flags or revert branch.
- Determinism invariants must remain intact.

---

## Exit Criteria

- All WPs #401–#411 complete.
- Coverage audit complete (#409).
- Docs pass complete (#410).
- Release notes drafted and reviewed.
- Determinism invariants explicitly documented.
- No template placeholders remain in milestone docs.
