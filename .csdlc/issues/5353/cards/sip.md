# Structured Intent Prompt

Template: 1.0.0

Issue: 5353

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Make typed initialization and design approval atomic and digest-complete.

## Required Outcome

Issue-local design paths initialize without partial-record confusion, and approval refreshes both design and diagram digests.

## Scope

- csdlc-v2/src/lifecycle.rs
- csdlc-v2/src/store.rs
- csdlc-v2/src/cards.rs
- csdlc-v2/tests

## Authority

- typed Rust v2 only
- no v1 wrappers
- no ADL core rearchitecture

## Assumptions

- none

## Operator Constraints

- none
