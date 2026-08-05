# Structured Intent Prompt

Template: 1.0.0

Issue: 5842

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Complete WP-22: Quality gate.

## Required Outcome

quality gate that blocks internal review until every indexed v0.92 feature is landed with accepted exact-revision proof

## Scope

- Build the complete feature and critical-path matrix from the canonical v0.92 index
- Accept rows only with exact implementation, validation, negative, review, merge, integration, platform, and typed terminal evidence
- Emit a quality-gate record and owner-routed blocker report that fail closed before internal review

## Authority

- Issue 5842 owns only WP-22: Quality gate
- Adjacent v0.92 work packages retain their own implementation and proof authority

## Assumptions

- none

## Operator Constraints

- Prepare before execution
- Never edit tracked work on main
- Use one bounded pre-PR review
- Do not substitute fixtures, receipts, or prose for required working behavior
