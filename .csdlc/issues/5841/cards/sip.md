# Structured Intent Prompt

Template: 1.0.0

Issue: 5841

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Complete WP-21A: Rust refactoring and maintainability pass.

## Required Outcome

behavior-preserving simplification of active Rust ownership boundaries, duplication, and maintainability hotspots before review

## Scope

- adl-runtime-kernel/src/control.rs
- adl-runtime-kernel/tests/control.rs
- adl-runtime-kernel/src/observability.rs
- adl-runtime-kernel/tests/observability.rs
- .csdlc/evidence/5841
- .csdlc/prepared/issues/5841/validate-refactor-selection.rb

## Authority

- Issue 5841 owns only WP-21A: Rust refactoring and maintainability pass
- Adjacent v0.92 work packages retain their own implementation and proof authority

## Assumptions

- none

## Operator Constraints

- Prepare before execution
- Never edit tracked work on main
- Use one bounded pre-PR review
- Do not substitute fixtures, receipts, or prose for required working behavior
