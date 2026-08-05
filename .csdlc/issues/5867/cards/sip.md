# Structured Intent Prompt

Template: 1.0.0

Issue: 5867

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Implement deterministic membership epochs and bounded topology convergence from authenticated join events.

## Required Outcome

Implement deterministic membership epochs and bounded topology convergence from authenticated join events.

## Scope

- adl-runtime/src/distributed/membership.rs
- adl-runtime/tests/distributed_membership.rs

## Authority

- Issue 5867 exclusively owns the declared paths
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
