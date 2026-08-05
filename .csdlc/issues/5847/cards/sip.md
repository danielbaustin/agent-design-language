# Structured Intent Prompt

Template: 1.0.0

Issue: 5847

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Complete WP-26: External / third-party review.

## Required Outcome

external review handoff and received review

## Scope

- Prepare and freeze a publication-safe exact-revision third-party review handoff after WP-25
- Dispatch only through an operator-approved reviewer channel with read-only authority and current digest
- Retain the received report unchanged and create a separate complete findings index for WP-27

## Authority

- Issue 5847 owns only WP-26: External / third-party review
- Adjacent v0.92 work packages retain their own implementation and proof authority

## Assumptions

- none

## Operator Constraints

- Prepare before execution
- Never edit tracked work on main
- Use one bounded pre-PR review
- Do not substitute fixtures, receipts, or prose for required working behavior
