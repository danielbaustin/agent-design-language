# Structured Intent Prompt

Template: 1.0.0

Issue: 5871

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Implement bounded signed capability advertisements that are evidence inputs, never direct authority grants.

## Required Outcome

Implement bounded signed capability advertisements that are evidence inputs, never direct authority grants.

## Scope

- adl-runtime/src/distributed/capability_advertisement.rs
- adl-runtime/tests/distributed_capability_advertisement.rs

## Authority

- Issue 5871 exclusively owns the declared paths
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
