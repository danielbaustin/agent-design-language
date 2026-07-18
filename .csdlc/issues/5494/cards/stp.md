# Structured Task Prompt

Template: 1.0.0

Issue: 5494

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Complete only the four remaining WP-07A findings and reconcile their proof truth.

## Deliverables

- Production supervised assembly
- Observed fail-closed readiness
- Real assembled-runtime soak
- Bounded credential overlap
- Updated WP-07A proof and register disposition

## Acceptance

1. AC1: Production executes declared required components through the supervisor
2. AC2: Readiness consumes observed health for every required component and typed channel
3. AC3: The deterministic soak runs supervised tasks and channels and proves failure and recovery readiness transitions
4. AC4: Credential renewal accepts old and new generations only during bounded overlap and revocation rejects both
5. AC5: #5409 and the canonical register close only after exact-revision review and proving validation

## Dependencies

- Tokio
- Existing Runtime v2 supervision and typed-channel primitives
- Existing Runtime API authentication store

## Inputs

- docs/reviews/v0.91.7/remaining-sprints-5403/WP07A_REARCHITECTURE_REVIEW_5121.md
- PR #5420
- Issue #5409

## Non Goals

- Runtime v3 changes
- AWS execution
- Unrelated ADL refactoring
