# Structured Task Prompt

Template: 1.0.0

Issue: 5698

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Runtime v3 durable local state only; no Provider/ACIP/A2A/Cloud Bridge transport behavior, no Runtime v2 deletion, and no WP-12 launch harness changes beyond preserving compatibility.

## Deliverables

- redb durable state module
- checkpoint/lifelog adapter wiring
- restart, corruption, identity, and writer-lock tests
- strict Clippy and diff hygiene proof
- ready PR with Closes #5698

## Acceptance

1. AC-1: A real Runtime v3 process stores checkpoint state through redb, restarts, and restores the exact persisted bytes and generation.
2. AC-2: Lifelog append uses the same redb database and a documented atomic transaction boundary.
3. AC-3: Principal, runtime identity, schema, generation, and hash mismatches fail closed.
4. AC-4: A second live writer for the same state root is rejected without corrupting the first writer.
5. AC-5: Crash/restart and interrupted-write tests recover from the last committed transaction only.
6. AC-6: Corrupt or unsupported database/schema state returns an explicit fatal error and never falls back to JSON/JSONL files.
7. AC-7: The old production checkpoint.json and lifelog.jsonl paths are deleted or restricted to non-production migration tests after parity proof.
8. AC-8: Focused macOS/Linux/native-Windows-capable tests use real filesystem state and do not use fixtures, wrappers, nested builds, or /private/tmp.
9. AC-9: One exact pre-PR review confirms one durable state authority and no hidden flat-file fallback.

## Dependencies

- WP-12 #5344 exact head ca242a5a for current Runtime v3 launch simplification
- redb COTS crate already used by adl-runtime backpressure
- Runtime v3 local adapter execution surface

## Inputs

- GitHub issue #5698 body
- adl-runtime-kernel/src/assembly.rs
- adl-runtime-kernel/src/lib.rs
- adl-runtime-kernel/Cargo.toml
- adl-runtime/src/backpressure.rs

## Non Goals

- scheduler redesign
- Runtime v2 deletion
- remote database
- SQL service
- fixture-only durability
- receipt-only storage
- degraded fallback
