# Structured Intent Prompt

Template: 1.0.0

Issue: 5866

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Implement bounded seed discovery and authenticated join without making discovery an authority source.

## Required Outcome

Implement bounded seed discovery and authenticated join without making discovery an authority source.

## Scope

- adl-runtime/src/distributed/discovery.rs
- adl-runtime/tests/distributed_discovery.rs

## Authority

- Issue 5866 exclusively owns the declared paths
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
