# Structured Intent Prompt

Template: 1.0.0

Issue: 5874

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Implement authenticated snapshot catalog entries and content-bound transfer manifests without exposing private state.

## Required Outcome

Implement authenticated snapshot catalog entries and content-bound transfer manifests without exposing private state.

## Scope

- adl-runtime/src/distributed/snapshot_catalog.rs
- adl-runtime/tests/distributed_snapshot_catalog.rs

## Authority

- Issue 5874 exclusively owns the declared paths
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
