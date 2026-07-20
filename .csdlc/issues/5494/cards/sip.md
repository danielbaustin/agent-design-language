# Structured Intent Prompt

Template: 1.0.0

Issue: 5494

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Make the Runtime v2 production topology, readiness, soak, and credential renewal satisfy #5409 truthfully.

## Required Outcome

Production uses supervised tasks, readiness consumes observed health, the soak exercises running assembly behavior, and credential rotation has bounded overlap.

## Scope

- adl-runtime/src/runtime_api_auth.rs
- adl-runtime/src/supervision.rs
- adl-runtime/src/topology.rs
- adl/src/csm_runtime_api.rs
- adl/src/long_lived_agent.rs
- docs/review-fixes/runtime/WP07A_REARCHITECTURE_REPAIR_5409.md
- docs/milestones/v0.91.7/review/V0917_SPRINT_REVIEW_REGISTER.md

## Authority

- GitHub #5409 remains the product acceptance issue
- Retained #5409 terminal evidence is immutable premature-closeout history
- Issue #5494 is the corrective execution authority

## Assumptions

- none

## Operator Constraints

- none
