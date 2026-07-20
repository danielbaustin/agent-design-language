# Structured Task Prompt

Template: 1.0.0

Issue: 5569

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Repair only #5547 SPP step status using retained proof.

## Deliverables

- Four typed repair requests
- Corrected #5547 receipt and projection
- Repair authority record

## Acceptance

1. AC-1: Four exact typed terminal plan repairs succeed under #5569 authority
2. AC-2: #5547 remains closed_out claim-free and receipt/projection equivalent
3. AC-3: Doctor and diff hygiene pass without new execution claims

## Dependencies

- #5547
- PR #5554

## Inputs

- #5547
- PR #5554
- .git/csdlc-v2/closeout/5547.json

## Non Goals

- No #5547 source changes
- No terminal disposition change
- No historical evidence rewrite
