# Structured Task Prompt

Template: 1.0.0

Issue: 5831

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Deliver working evaluation bindings, durable adaptation deltas, governed graph mutation, and replay-safe Adaptive Learning DAG execution.

## Deliverables

- working evaluation bindings, durable adaptation deltas, governed graph mutation, and replay-safe Adaptive Learning DAG execution
- real adaptive execution, deterministic replay, durable state deltas, accepted and rejected mutation paths, and required negative cases

## Acceptance

1. The Adaptive Learning DAG executes real Runtime v3 behavior at the exact reviewed revision
2. Declared dependencies are verified from current evidence
3. Every accepted state or graph change is policy-authorized, durable, and deterministically replayable
4. Rejected mutation, forged history, discontinuous resume, invalid binding, unbounded recurrence, and missing evidence are proven negative cases
5. No fixture, receipt, demo mode, synthetic result, or planning document substitutes for runtime behavior
6. One bounded pre-PR review has no unresolved actionable findings

## Dependencies

- WP-01
- WP-13
- issue-5104-merge-evidence
- current Runtime v3 loop qualification

## Inputs

- docs/milestones/v0.92/WP_ISSUE_WAVE_v0.92.yaml
- docs/milestones/v0.92/WBS_v0.92.md
- docs/milestones/v0.92/features/ADAPTIVE_LEARNING_DAG_v0.92.md
- .csdlc/evidence/5817/prerequisite-and-loop-runtime-requalification.md
- adl-runtime-kernel/src/reasoning.rs
- adl-runtime-kernel/tests/reasoning.rs

## Non Goals

- Adjacent work packages
- Historical evidence rewriting
- Ungoverned or unconstrained self-modification
- Treating a planning packet as implementation
- Unsupported downstream milestone claims
