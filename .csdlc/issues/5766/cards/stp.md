# Structured Task Prompt

Template: 1.0.0

Issue: 5766

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Repair endpoint inventory/router truth and focused tests for #5766 only.

## Deliverables

- Reconciled endpoint inventory
- Focused route inventory tests
- Truthful readiness distinction
- Updated evidence/register wording if stale

## Acceptance

1. AC-1: CSM_RUNTIME_API_ENDPOINTS and runtime_api_router agree exactly, with a focused route inventory test.
2. AC-2: /v1/ready CSM semantics are truthful if mounted; otherwise it is not advertised as available.
3. AC-3: Existing Runtime v3 kernel /v1/ready work remains distinct from this CSM runtime API surface.
4. AC-4: Focused runtime API tests pass.
5. AC-5: No AWS work.

## Dependencies

- current Runtime v3 observatory/runtime API source
- #5764 overnight probe evidence

## Inputs

- adl-runtime/src/runtime_api.rs
- adl/src/csm_runtime_api.rs
- adl/src/csm_api_gateway_bridge.rs
- docs/reviews/v0.91.8/internal-review-5356/FINDINGS_REGISTER.md

## Non Goals

- Implement all planned CSM feature endpoints
- Change Runtime v3 kernel readiness semantics
- AWS usage
- Observatory Shepherd model integration
