# Structured Intent Prompt

Template: 1.0.0

Issue: 5563

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Restore a legal typed lifecycle path when approved authored design inputs change before readiness.

## Required Outcome

Initialized approved records can refresh stale design projections atomically without hand edits or gate bypass.

## Scope

- csdlc-v2/src/store.rs
- csdlc-v2/tests/gate2.rs

## Authority

- Typed csdlc-edit approve-design operation only

## Assumptions

- none

## Operator Constraints

- none
