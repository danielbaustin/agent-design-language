# Structured Intent Prompt

Template: 1.0.0

Issue: 5873

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Implement deterministic bounded placement from membership, fencing, capability, and resource-weather inputs.

## Required Outcome

Implement deterministic bounded placement from membership, fencing, capability, and resource-weather inputs.

## Scope

- adl-runtime/src/distributed/placement.rs
- adl-runtime/tests/distributed_placement.rs

## Authority

- Issue 5873 exclusively owns the declared paths
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
