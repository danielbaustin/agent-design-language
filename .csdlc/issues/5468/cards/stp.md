# Structured Task Prompt

Template: 1.0.0

Issue: 5468

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Repair only terminal SRP status normalization and its focused lifecycle regression.

## Deliverables

- Typed terminal SRP status normalization
- Focused retained-receipt regression
- Regenerated truthful #5452 terminal projection

## Acceptance

1. AC1: A closed-out receipt with completed passing review projects SRP status complete
2. AC2: Projection and retained receipt carry the same normalized status and digest truth
3. AC3: Reconciliation remains atomic, deterministic, and fail closed
4. AC4: #5452 terminal projection validates without stale SRP status

## Dependencies

- Existing typed reconcile-terminal transaction from #5438

## Inputs

- csdlc-v2/src/store.rs
- csdlc-v2/tests/gate7_lifecycle.rs
- GitHub issue #5468
- Terminal review finding from #5452

## Non Goals

- Arbitrary post-closeout editing
- SOR follow-up reconciliation changes
- Runtime, AWS, CI, or workflow changes
