# Structured Task Prompt

Template: 1.0.0

Issue: 5440

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Change only the C-SDLC v2 design approval authorization and focused tests.

## Deliverables

- Bound and implemented design reapproval
- Later-phase rejection
- Focused tests

## Acceptance

1. Bound and implemented reapproval refreshes both design-bearing cards
2. Generation and audit history advance without phase or transition rewrite
3. Normal typed card mutation works after implemented reapproval
4. Reviewed and later phases reject design reapproval
5. Focused C-SDLC v2 tests pass

## Dependencies

- Existing typed approve-design operation
- Issue #5411 reproduction

## Inputs

- csdlc-v2/src/store.rs
- csdlc-v2/tests/gate2.rs

## Non Goals

- Manual canonical-state mutation
- Review recovery redesign
- Runtime changes
