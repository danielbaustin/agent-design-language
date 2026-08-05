# Structured Intent Prompt

Template: 1.0.0

Issue: 5875

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Implement prepare, quiesce, checkpoint, transfer, validate, fence, activate, and commit with source authority retained until validation and fencing succeed.

## Required Outcome

Implement prepare, quiesce, checkpoint, transfer, validate, fence, activate, and commit with source authority retained until validation and fencing succeed.

## Scope

- adl-runtime/src/distributed/migration.rs
- adl-runtime/tests/distributed_migration.rs

## Authority

- Issue 5875 exclusively owns the declared paths
- WP-04-IMP issue 5862 coordinates only
- WP-04.16 alone owns final module registration
- No sibling, Runtime v2, or v0.93 authority

## Assumptions

- none

## Operator Constraints

- Do not start before #5821 is terminal
- Bind only the exact exclusive paths
- Use nonzero exact test selection
- Fix all actionable pre-PR findings
