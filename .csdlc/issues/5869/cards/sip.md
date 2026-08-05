# Structured Intent Prompt

Template: 1.0.0

Issue: 5869

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Implement monotonic epochs and bounded leases as prerequisites for distributed ownership decisions.

## Required Outcome

Implement monotonic epochs and bounded leases as prerequisites for distributed ownership decisions.

## Scope

- adl-runtime/src/distributed/lease.rs
- adl-runtime/tests/distributed_lease.rs

## Authority

- Issue 5869 exclusively owns the declared paths
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
