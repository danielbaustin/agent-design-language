# Structured Task Prompt

Template: 1.0.0

Issue: 5518

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Add and prove one atomic terminal plan-step repair operation, then use it on #5516.

## Deliverables

- Typed repair request and store operation
- Focused fail-closed and rollback tests
- Corrected #5516 SPP S3 and receipt
- Reviewed terminal reconciliation

## Acceptance

1. AC1: only pending or in-progress terminal SPP steps may advance to completed
2. AC2: stale authority, target, artifact, or receipt identity fails closed
3. AC3: invalid phase, claim, step, or transition fails closed
4. AC4: receipt refresh failure rolls target and receipt back
5. AC5: #5516 SPP S3 and terminal receipt agree at completed
6. AC6: focused tests and exact review pass

## Dependencies

- Closed #5516
- Typed C-SDLC v2 terminal design repair pattern

## Inputs

- #5516 terminal review finding
- csdlc-v2/src/store.rs
- csdlc-v2/src/bin/csdlc-closeout.rs

## Non Goals

- General terminal card editing
- Audit rewriting
- Runtime changes
- Runtime v2 changes
- AWS execution
