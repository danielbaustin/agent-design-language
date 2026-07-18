# Structured Intent Prompt

Template: 1.0.0

Issue: 5468

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Keep terminal SRP card status consistent with retained completed review evidence.

## Required Outcome

Typed terminal reconciliation projects SRP status complete when closed-out review truth is completed, with atomic receipt and projection validation.

## Scope

- csdlc-v2/src/store.rs
- csdlc-v2/tests/gate7_lifecycle.rs

## Authority

- GitHub issue #5468 defines the terminal status defect
- Gate 10D2 typed v2 reconciliation remains the sole mutation authority
- Arbitrary post-closeout card edits remain forbidden

## Assumptions

- none

## Operator Constraints

- none
