# Structured Task Prompt

Template: 1.0.0

Issue: 5470

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Add a bounded journaled terminal projection/receipt transaction with deterministic recovery and fault-injection proof.

## Deliverables

- Crash-consistent terminal transaction or recovery journal
- Explicit file and parent-directory synchronization
- Interruption recovery and fault-injection tests
- Design and diagram evidence

## Acceptance

1. Projection and receipt converge to one generation after success or recovery
2. Receipt bytes and parent directory are synchronized before success
3. Interruption before and after each durable write/rename recovers deterministically
4. Identity, rollback, idempotence, and typed reconciliation contracts remain green

## Dependencies

- Existing Store::reconcile_terminal implementation
- Retained terminal receipt format
- Rust filesystem primitives and focused test harness

## Inputs

- csdlc-v2/src/store.rs
- csdlc-v2/src/bin/csdlc-closeout.rs
- csdlc-v2/src/readiness.rs
- csdlc-v2/tests/gate7_lifecycle.rs

## Non Goals

- Runtime or AWS changes
- Arbitrary post-closeout card mutation
- Blocking #5468 SRP truth normalization
