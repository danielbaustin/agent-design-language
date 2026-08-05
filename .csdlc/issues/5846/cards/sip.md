# Structured Intent Prompt

Template: 1.0.0

Issue: 5846

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Complete WP-25: Internal review.

## Required Outcome

internal review report and finding register

## Scope

- Freeze an exact-revision publication-safe packet after WP-23, WP-24, and WP-24A
- Run independent code, architecture, tests/PVF/CI, security, dependency, docs, lifecycle, demo/integration, and release/publication lanes
- Publish a findings-first internal report and complete finding register without remediation

## Authority

- Issue 5846 owns only WP-25: Internal review
- Adjacent v0.92 work packages retain their own implementation and proof authority

## Assumptions

- none

## Operator Constraints

- Prepare before execution
- Never edit tracked work on main
- Use one bounded pre-PR review
- Do not substitute fixtures, receipts, or prose for required working behavior
