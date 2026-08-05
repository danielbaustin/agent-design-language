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

- Profile the exact post-WP-21 active Rust products and rank ownership, duplication, dependency-direction, size, and test hotspots
- Refactor a small exact-file set under one declared owner without changing supported behavior or public contracts
- Retain characterization, negative, before/after LoC, dependency, lint, test, platform, and review proof

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
