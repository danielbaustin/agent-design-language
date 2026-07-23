# Structured Task Prompt

Template: 1.0.0

Issue: 5627

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Delete or combine routine lifecycle ceremony while preserving exact scope, atomic failure, publication identity, and terminal receipts.

## Deliverables

- Atomic csdlc-validate finalize operation
- Assignment-free exact csdlc-review record operation that advances Reviewed
- Direct ready-PR publication with active-draft compatibility
- Untracked shared-Git one-shot request convention
- Measured four-command and at-most-two-artifact regression

## Acceptance

1. AC-1: #5624-equivalent routine lifecycle requires four typed commands including unchanged closeout
2. AC-2: post-product durable request/evidence artifacts are two or fewer
3. AC-3: failed validation, stale review, claim mismatch, and wrong PR identity perform zero state writes
4. AC-4: exact scoped review and terminal receipt integrity are unchanged
5. AC-5: existing active draft publications remain reconcilable
6. AC-6: focused Gate 4-7 and complete four-command lifecycle regressions pass under /Volumes/FastWork

## Dependencies

- Current origin/main including merged issue 5624 state
- Existing typed C-SDLC v2 store locking, claim, review, publication, and closeout contracts

## Inputs

- csdlc-v2/src/pvf.rs
- csdlc-v2/src/store.rs
- csdlc-v2/src/review.rs
- csdlc-v2/src/publication.rs
- csdlc-v2/src/doctor.rs
- csdlc-v2/src/cards.rs
- csdlc-v2/src/bin
- csdlc-v2/tests/gate4.rs
- csdlc-v2/tests/gate5.rs
- csdlc-v2/tests/gate6.rs
- csdlc-v2/tests/gate7_lifecycle.rs
- Rust standard library and existing workspace dependencies

## Non Goals

- No Runtime or AWS work
- No broad workflow redesign or new dependency
- No weakening of claims, review scope, validation, publication identity, or closeout
- No compatibility retention beyond active draft reconciliation
