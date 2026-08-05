# Structured Intent Prompt

Template: 1.0.0

Issue: 5877

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Expose redacted versioned topology, certificate, failure, lease, placement, and migration state through the Runtime API contract.

## Required Outcome

Expose redacted versioned topology, certificate, failure, lease, placement, and migration state through the Runtime API contract.

## Scope

- adl-runtime/src/distributed/projection.rs
- adl-runtime/tests/distributed_projection.rs
- docs/api/runtime-v3/v1/distributed.openapi.json

## Authority

- Issue 5877 exclusively owns the declared paths
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
