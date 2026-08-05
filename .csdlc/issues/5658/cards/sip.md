# Structured Intent Prompt

Template: 1.0.0

Issue: 5658

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Make typed C-SDLC v2 lifecycle writes materialize and operate from the declared bound worktree instead of dirtying the primary checkout.

## Required Outcome

Binding and later doctor/validate/review/publish/closeout use the same issue worktree root, while primary-main lifecycle writes fail closed outside explicit bootstrap/read-only paths.

## Scope

- csdlc-v2/src/lifecycle.rs
- csdlc-v2/src/store.rs
- csdlc-v2/tests/gate7_lifecycle.rs

## Authority

- Issue #5658 owns only typed v2 lifecycle-root behavior and focused regressions
- Existing issue records and #5655 are preserved
- Root main cleanup is not part of implementation

## Assumptions

- none

## Operator Constraints

- Use FastWork only
- Do not write on main
- No AWS
- Rust binaries only; no Python or shell lifecycle wrapper
