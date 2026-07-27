# Structured Intent Prompt

Template: 1.0.0

Issue: 5663

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Replace receipt-only local Runtime v3 production adapter behavior with durable bounded local behavior for the six local services.

## Required Outcome

Production assembly uses real bounded local adapter behavior for Agent, Shepherd, Scheduler, Chronosense, CheckpointStore, and Lifelog while retaining fail-closed readiness and excluding external transport scope.

## Scope

- Runtime v3 production assembly local adapter implementation
- Agent and Shepherd admission/execution boundaries
- Scheduler and Chronosense bounded stateful behavior
- CheckpointStore atomic local persistence and restore
- Lifelog redacted append-only non-authoritative events
- Focused tests, strict Clippy, exact review, and measured net physical LoC reduction

## Authority

- Issue 5663 owns only its issue-local records plus adl-runtime-kernel/src/assembly.rs and adl-runtime-kernel/tests/assembly.rs
- Merged PR #5659 ancestry is accepted as the dependency gate; #5657 typed closeout is parallel post-merge work
- Provider, ACIP, A2A, Cloud Bridge, AWS, and WP-12 cutover are out of scope
- Existing active typed claims on operations.rs, governed_operations.rs, launch binary, config, control, Observatory, and adl-runtime paths remain untouched

## Assumptions

- none

## Operator Constraints

- Typed C-SDLC v2 lifecycle only
- No main checkout edits
- No AWS and no Python wrappers
- No degraded or receipt-only production success path for the six local adapters
- Require before/after physical LoC measurement and net reduction
