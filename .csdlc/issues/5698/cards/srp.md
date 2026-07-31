# Structured Review Prompt

Template: 1.0.0

Issue: 5698

Repository: danielbaustin/agent-design-language

Card: srp

Status: complete

## Scope

adl-runtime-kernel/Cargo.lock
adl-runtime-kernel/Cargo.toml
adl-runtime-kernel/src/assembly.rs
adl-runtime-kernel/src/durable_state.rs
adl-runtime-kernel/src/governed_operations.rs
adl-runtime-kernel/src/lib.rs
adl-runtime-kernel/tests/assembly.rs
adl-runtime-kernel/tests/durable_state.rs
adl-runtime-kernel/tests/governed_operations.rs
adl-runtime-kernel/tests/guardian_soak.rs
adl-v2/Cargo.lock

## Prompts

- Verify Runtime v3 checkpoint and lifelog production adapters use redb as the single durable state authority.
- Verify restart restores exact committed bytes and rejects identity, schema, generation, and hash mismatches.
- Verify writer locking, corruption, and interrupted-write behavior fail closed without fallback JSON/JSONL storage.
- Verify scope coordination with #5344 and no tracked main or /private/tmp edits.

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Gemini exact-head CI repair review for 89e162f79392a520678913cac7af47b6695a8d1a returned PASS / no findings. It confirmed the guardian_soak IPv4 loopback repair preserves real socket proof and matches Runtime v3's IPv4-only control-plane contract.

## Review Result

Revision: Some("git-blake3:89e162f79392a520678913cac7af47b6695a8d1a:e4f836330b00072bb8eb33d195876dcaf18298a1ff9b5d31be82123373a83eb5")

Reviewer: Some("provider:gemini-3.1-pro-preview")

Result: pass
