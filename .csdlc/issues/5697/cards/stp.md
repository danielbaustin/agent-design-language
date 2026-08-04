# Structured Task Prompt

Template: 1.0.0

Issue: 5697

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Apply the already prepared five-file Chronosense trusted-time correction to a new #5697 branch from current origin/main and validate it.

## Deliverables

- Five-file Runtime v3 Chronosense trusted-time correction
- Focused assembly and governed-operation test evidence
- Strict all-target Clippy evidence
- Exact-head review evidence
- Ready PR body with Closes #5697

## Acceptance

1. AC-1: Production Chronosense consumes RecorderTrustedTime backed by the live RuntimeRecorder and never reads SystemTime directly
2. AC-2: Chronosense fails closed until trusted_time is qualified and returns monotonic qualified samples afterward
3. AC-3: Topology expresses a control/readiness dependency on trusted_time without fabricating an OperationResult data input
4. AC-4: Startup proof requires trusted_time Running immediately followed by Chronosense Running, then Scheduler and all remaining live services
5. AC-5: Focused assembly and governed-operation tests pass
6. AC-6: Strict all-target Clippy passes
7. AC-7: Exact-head review passes before ready publication
8. AC-8: Ready PR closes #5697

## Dependencies

- Current origin/main at e7cab4ab61e9e2db56dea3d3a1d2cd0adba343d4
- Source-only commit 059cd5a48abf65d61870b78d5d9146d06025a37d from the terminal #5663 branch

## Inputs

- GitHub issue #5697
- adl-runtime-kernel/src/assembly.rs
- adl-runtime-kernel/src/bin/adl-runtime-kernel.rs
- adl-runtime-kernel/src/governed_operations.rs
- adl-runtime-kernel/src/operations.rs
- adl-runtime-kernel/tests/assembly.rs

## Non Goals

- No #5663 lifecycle mutation
- No Provider, ACIP, A2A, or Cloud Bridge behavior changes
- No selector cutover, Runtime v1 deletion, or WP-12 evidence mutation
