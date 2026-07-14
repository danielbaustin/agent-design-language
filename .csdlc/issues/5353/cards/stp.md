# Structured Task Prompt

Template: 1.0.0

Issue: 5353

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Repair only initialization atomicity and design/diagram digest refresh behavior.

## Deliverables

- focused regressions
- typed partial-initialization handling
- design and diagram digest parity

## Acceptance

1. issue-local design paths initialize safely
2. partial initialization fails typed and recoverably
3. approval refreshes SPP and VPP design and diagram digests
4. doctor passes after init and approval

## Dependencies

- none

## Inputs

- csdlc-v2/src/lifecycle.rs
- csdlc-v2/src/store.rs
- csdlc-v2/src/cards.rs
- csdlc-v2/tests

## Non Goals

- ADL core rearchitecture
- runtime changes
- v1 wrapper restoration
