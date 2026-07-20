# Structured Task Prompt

Template: 1.0.0

Issue: 5551

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Invoke the existing typed terminal plan repair three times and validate exact parity.

## Deliverables

- Corrected #5527 SPP S2-S4
- Refreshed #5527 terminal receipt
- Typed doctor and exact-diff proof

## Acceptance

1. AC-1: exact authority, target, and receipt CAS succeeds for each repair
2. AC-2: #5527 S2, S3, and S4 are completed and S1 remains completed
3. AC-3: #5527 card, index, and receipt parity is exact
4. AC-4: typed doctor passes for #5527 and #5551

## Dependencies

- Merged PR #5550
- Typed terminal plan-step repair from #5518

## Inputs

- .csdlc/issues/5527
- csdlc-v2/closeout/5527.json

## Non Goals

- Source changes
- General terminal editing
- Runtime changes
- AWS execution
