# Structured Task Prompt

Template: 1.0.0

Issue: 5566

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Implement only exact existing-worktree recognition for typed bind and focused regressions.

## Deliverables

- Existing-worktree bind recognition
- Focused Gate 2 regressions

## Acceptance

1. AC-1: Existing matching branch and canonical worktree path activate the exact reserved claim
2. AC-2: Mismatched claimed branch or worktree path fails closed
3. AC-3: Exact claim, collision, path-safety, and readiness guards remain enforced
4. AC-4: The #5306 recovery can bind without editing main or lifecycle JSON by hand

## Dependencies

- none

## Inputs

- csdlc-v2/src/lifecycle.rs
- csdlc-v2/tests/gate2.rs
- .csdlc/issues/5306

## Non Goals

- No expired-claim recovery changes
- No v1 restoration
- No AWS or Spot validation
