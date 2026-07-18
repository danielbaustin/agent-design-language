# Structured Task Prompt

Template: 1.0.0

Issue: 5426

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Change only C-SDLC v2 validation evaluation and focused tests needed for deterministic supersession.

## Deliverables

- Shared latest-validation evaluation helper
- Readiness and terminal card validation integration
- Regression tests for both supersession directions
- Design and validation evidence

## Acceptance

1. AC-1: repeated observations have deterministic logical identity
2. AC-2: pass after waiting permits readiness
3. AC-3: failure after pass still fails closed
4. AC-4: append-only history is preserved
5. AC-5: focused Rust tests and formatting pass

## Dependencies

- Issue #5426
- Current Gate 10D2 typed C-SDLC v2 authority

## Inputs

- csdlc-v2/src/store.rs
- csdlc-v2/src/cards.rs
- csdlc-v2/tests/gate7_lifecycle.rs

## Non Goals

- Manual lifecycle state repair
- Removing historical validation evidence
- Runtime or product changes
