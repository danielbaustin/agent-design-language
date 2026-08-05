# Structured Task Prompt

Template: 1.0.0

Issue: 5663

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Implement the durable local adapter behavior in the assembly surface without widening into external transports or claimed launch/Observatory paths.

## Deliverables

- Durable local adapter implementation in production assembly
- Focused assembly tests for success, failure, timeout, cancellation, idempotency, restart, shutdown, checkpoint restore, and lifelog redaction where applicable to the owned surface
- Before/after physical LoC measurement showing net reduction
- Strict Clippy evidence and exact pre-PR review evidence

## Acceptance

1. AC-1: Agent, Shepherd, Scheduler, Chronosense, CheckpointStore, and Lifelog no longer return only generic accepted receipts in production
2. AC-2: malformed, timed-out, cancelled, saturated, duplicate, restart, and shutdown paths are explicit and tested in the owned assembly surface
3. AC-3: CheckpointStore writes atomically and restore rejects missing or corrupt checkpoint state
4. AC-4: Lifelog appends redacted non-authoritative JSONL entries and cannot leak raw secret-like payloads
5. AC-5: Provider, ACIP, A2A, Cloud Bridge, AWS, and WP-12 cutover remain untouched
6. AC-6: before/after physical LoC measurement records a net reduction
7. AC-7: focused tests and strict Clippy pass under FastWork target storage
8. AC-8: exact pre-PR review passes before publication

## Dependencies

- PR #5659 merge commit d751ecea0fa6c638d7897087913ab3968b772874 is ancestral to current origin/main
- Existing Runtime v3 assembly and operation executor contracts

## Inputs

- GitHub issue #5663
- adl-runtime-kernel/src/assembly.rs
- adl-runtime-kernel/tests/assembly.rs
- adl-runtime-kernel/src/operations.rs as read-only contract context
- infra/runtime-v3/runtime-init.toml as read-only launch context

## Non Goals

- No Provider, ACIP, A2A, or Cloud Bridge transport implementation
- No AWS
- No WP-12 soak or cutover
- No launch binary, config, control, Observatory, governed_operations.rs, or operations.rs edits under this initial protected path set
