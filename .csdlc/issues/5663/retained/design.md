# Issue 5663 Design: Durable Local Runtime v3 Adapters

## Boundary

Implement durable local Runtime v3 adapter behavior for Agent, Shepherd,
Scheduler, Chronosense, CheckpointStore, and Lifelog inside the local
production assembly surface. This lane does not add Provider, ACIP, A2A, Cloud
Bridge, external transports, AWS, or WP-12 cutover behavior.

## Current Problem

The production assembly currently constructs generic in-process adapter
receipts for every required operational adapter. That proves an identity-bearing
binding exists, but it does not give the local Runtime v3 services bounded
stateful behavior for admission, scheduling, time sampling, checkpoint
persistence, append-only lifelogging, or agent execution.

## Implementation Shape

Replace the generic receipt-only executor path for the six local adapters with
bounded in-process executors that use existing Tokio/Rust primitives and the
existing `OperationExecutor` contract. Keep the production binding fail-closed
when any required adapter is missing.

The local adapters should provide:

- Agent: accept bounded local work, reject malformed requests, preserve
  idempotent replay behavior, and return a typed execution result.
- Shepherd: enforce admission boundaries and return explicit rejection rather
  than production success when admission is invalid.
- Scheduler: maintain bounded scheduling state, enforce saturation and
  cancellation behavior, and keep results deterministic.
- Chronosense: return a real bounded local time sample rather than a bare
  receipt.
- CheckpointStore: persist and restore checkpoint state atomically under a
  bounded local directory.
- Lifelog: append redacted, non-authoritative JSONL events without allowing
  secret-like payloads to leak.

## Validation Plan

Use focused FastWork Rust validation for `adl-runtime-kernel` assembly behavior,
then strict Clippy for the same crate. The review must inspect the exact source
revision and retained validation evidence before publication.

## Non-Goals

No external Provider, ACIP, A2A, Cloud Bridge transport scope. No AWS. No Python
or shell lifecycle wrappers. No WP-12 soak or cutover.
