# Structured Intent Prompt

Template: 1.0.0

Issue: 5843

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Complete WP-23: Docs and release-truth pass.

## Required Outcome

current canonical docs, release notes, feature list, ADR plan, skills, agent guidance, and milestone docs

## Scope

- Inventory and reconcile canonical root, milestone, feature, ADR, release, skill, and agent-guidance claims after WP-22 acceptance
- Map every updated release-facing statement to exact landed evidence or explicit planned/blocked/non-claim truth
- Retain a docs-review packet, release-truth diff, and ADR candidate packet only where a real unrecorded decision exists

## Authority

- Issue 5843 owns only WP-23: Docs and release-truth pass
- Adjacent v0.92 work packages retain their own implementation and proof authority

## Assumptions

- none

## Operator Constraints

- Prepare before execution
- Never edit tracked work on main
- Use one bounded pre-PR review
- Do not substitute fixtures, receipts, or prose for required working behavior
