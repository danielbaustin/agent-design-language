# Structured Review Prompt

Template: 1.0.0

Issue: 5663

Repository: danielbaustin/agent-design-language

Card: srp

Status: draft

## Scope

adl-runtime-kernel/src/assembly.rs
adl-runtime-kernel/src/bin/adl-runtime-kernel.rs
adl-runtime-kernel/src/governed_operations.rs
adl-runtime-kernel/src/operations.rs
adl-runtime-kernel/tests/assembly.rs
adl-runtime-kernel/tests/operations.rs

## Prompts

- Can any of the six local adapters still earn production success with only a generic receipt?
- Are timeout, cancellation, saturation, duplicate, restart, shutdown, checkpoint restore, and lifelog redaction behaviors real and tested?
- Does the implementation stay out of Provider, ACIP, A2A, Cloud Bridge, AWS, and WP-12 scope?
- Is the before/after physical LoC measurement truthful and net-negative?
- Do focused tests and strict Clippy prove the owned surface without relying on fixture-only credit?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: Some("git-blake3:9e388b86b2c731381f7a195185652db87db3746d:98a12f92d966aca16fdc5d21ffec6ad7b578ceb3995c7d109cd5703bc5108f4b")

Reviewer: Some("external:consolidated-opus-5")

Result: pass
