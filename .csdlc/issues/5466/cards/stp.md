# Structured Task Prompt

Template: 1.0.0

Issue: 5466

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Implement and prove only the narrow merged-head publication reconciliation needed for truthful closeout.

## Deliverables

- Typed reconcile-merged publication command
- Focused regression tests
- Truthful #5412 closeout proof

## Acceptance

1. AC-1: Reconciliation requires an explicit merged PR and exact final reviewed head
2. AC-2: Repository, base, head, issue linkage, title, body, SHA, and merged state fail closed on drift
3. AC-3: Normal draft publication behavior is unchanged
4. AC-4: #5412 can proceed through normal readiness and closeout using truthful final-head evidence

## Dependencies

- #5412 merged PR #5459 final-head review

## Inputs

- csdlc-v2/src/bin/csdlc-publish.rs
- csdlc-v2/src/publication.rs
- csdlc-v2/src/readiness.rs
- csdlc-v2/tests/gate6.rs

## Non Goals

- No automatic acceptance of arbitrary late commits
- No weakening of review guards
- No AWS execution
