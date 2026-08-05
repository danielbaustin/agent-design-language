# Structured Intent Prompt

Template: 1.0.0

Issue: 5786

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Complete WP-21: Repository-wide code reduction cleanup.

## Required Outcome

behavior-preserving cleanup with exact deletion denominator

## Scope

- Inventory and disposition every remaining adl/src Rust file and every active Cargo, CI, install, docs, demo, and command reference
- Delete only exact ownership bands whose supported behavior is proven through ADL v2, Runtime v3, C-SDLC v2, or a named adapter/product owner
- Retain issue-local denominator, parity, rollback, clean-install, platform, and exact-head review evidence

## Authority

- Issue 5786 owns only WP-21: Repository-wide code reduction cleanup
- Adjacent v0.92 work packages retain their own implementation and proof authority

## Assumptions

- none

## Operator Constraints

- Prepare before execution
- Never edit tracked work on main
- Use one bounded pre-PR review
- Do not substitute fixtures, receipts, or prose for required working behavior
