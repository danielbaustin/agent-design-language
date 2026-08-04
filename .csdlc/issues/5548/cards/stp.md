# Structured Task Prompt

Template: 1.0.0

Issue: 5548

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Prepare typed issue state and a dedicated FastWork worktree only; do not change product, source, or test behavior in this session.

## Deliverables

- typed C-SDLC v2 issue state for #5548
- dedicated /Volumes/FastWork issue worktree bound to the #5548 branch
- preparation commit and pushed branch

## Acceptance

1. AC-1: All Gate 2 tests reach their intended assertions and pass
2. AC-2: cargo test --locked passes for csdlc-v2
3. AC-3: Terminal receipt recovery still resolves the real repository common directory and remains fail-closed
4. AC-4: A regression proves non-Git fixture behavior explicitly
5. AC-5: No AWS dependency or validation

## Dependencies

- issue #5527 validation discovery
- current origin/main baseline

## Inputs

- GitHub issue #5548
- csdlc-v2/tests/gate2.rs
- csdlc-v2/src/store.rs
- csdlc-v2/src/lifecycle.rs

## Non Goals

- implementation in this preparation session
- broad tests during preparation
- PR publication before review
- AWS
- raw gh
- touching issue #5558
