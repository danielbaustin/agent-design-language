# Structured Task Prompt

Template: 1.0.0

Issue: 5527

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Add and prove one atomic terminal SOR artifact-reference repair operation, then use it on #5390.

## Deliverables

- Typed repair request and store operation
- Focused fail-closed and rollback tests
- Corrected #5390 SOR artifact reference and receipt
- Reviewed terminal reconciliation

## Acceptance

1. AC1: only exact existing SOR artifact references may be replaced
2. AC2: replacement must be a receipt-authenticated retained authored artifact with matching bytes
3. AC3: stale authority, target, artifact, or receipt identity fails closed
4. AC4: invalid phase, claim, path, duplicate, or no-op replacement fails closed
5. AC5: receipt refresh failure rolls target and receipt back
6. AC6: #5390 contains no generated reference to the deleted diagram path
7. AC7: focused tests, doctor, and exact review pass

## Dependencies

- Closed #5390
- Typed C-SDLC v2 terminal repair transaction pattern

## Inputs

- #5527 review finding
- .git/csdlc-v2/closeout/5390.json
- csdlc-v2/src/store.rs
- csdlc-v2/src/bin/csdlc-closeout.rs

## Non Goals

- General terminal card editing
- Audit rewriting
- Runtime changes
- Runtime v2 changes
- AWS execution
