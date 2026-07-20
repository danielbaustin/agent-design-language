# Structured Task Prompt

Template: 1.0.0

Issue: 5521

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Invoke the existing typed repair once and validate exact parity.

## Deliverables

- Corrected #5518 SPP S4
- Refreshed #5518 terminal receipt
- Typed doctor and exact-diff proof

## Acceptance

1. AC1: exact authority, target, and receipt CAS succeeds
2. AC2: #5518 S4 is completed and no other semantic field changes
3. AC3: #5518 card, index, and receipt parity is exact
4. AC4: typed doctor passes for #5518 and #5521

## Dependencies

- Merged PR #5519 terminal plan repair
- Merged PR #5520 #5518 terminal closeout

## Inputs

- .csdlc/issues/5518
- csdlc-v2/closeout/5518.json

## Non Goals

- Source changes
- General terminal editing
- Runtime changes
- AWS execution
