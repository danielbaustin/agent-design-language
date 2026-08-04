# Structured Task Prompt

Template: 1.0.0

Issue: 5664

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Implement only the bounded protocol adapter surface and focused tests for #5664.

## Deliverables

- Runtime v3 protocol adapter module
- Provider, ACIP, A2A, and Cloud Bridge executor builders
- Local deterministic Tokio black-box tests
- LoC before/after measurement
- Focused test and strict Clippy evidence
- Exact pre-PR review evidence

## Acceptance

1. AC-1: Provider dispatch performs real authenticated exchange with bounded timeout, cancellation, retry, idempotency, and replay rejection
2. AC-2: ACIP performs real bidirectional exchange and fails closed for malformed, unauthorized, replay, timeout, and shutdown cases
3. AC-3: A2A performs real authenticated message exchange and rejects replay and unauthorized messages
4. AC-4: Cloud Bridge forwards only declared capabilities and classifies unsupported, unavailable, unauthorized, malformed, timeout, retry exhaustion, and shutdown outcomes explicitly
5. AC-5: Networked transport exposes Rustls configuration and avoids plaintext credential material in tracked artifacts
6. AC-6: Deterministic local black-box tests, focused Rust tests, strict Clippy, LoC measurement, and exact pre-PR review pass

## Dependencies

- PR #5659 merged into current main from #5657
- #5663 disjoint durable local adapter scope
- #5665 disjoint Runtime API/WSS/Observatory scope

## Inputs

- adl-runtime-kernel/src/operations.rs
- adl-runtime-kernel/src/assembly.rs
- adl-runtime-kernel/Cargo.toml
- adl-runtime-kernel/tests/operations.rs
- adl-runtime-kernel/tests/assembly.rs

## Non Goals

- AWS provisioning or cloud account experiments
- Runtime API/WSS/Observatory feature proof implementation
- Guardian launch, runtime config, or runtime-init edits
- Durable local checkpoint/lifelog/scheduler/Chronosense adapter work
- Runtime v2 edits
- Cutover or default switch
