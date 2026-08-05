# Structured Task Prompt

Template: 1.0.0

Issue: 5824

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Deliver historical-delivery audit and only the proven remaining enum/schema correction.

## Deliverables

- historical-delivery audit and only the proven remaining enum/schema correction
- schema inventory, typed round trips, negative cases, and no-duplicate-work disposition

## Acceptance

1. The declared required outcome is complete at the exact reviewed revision
2. Declared dependencies are verified from current evidence
3. The named proof surface is reproducible and retained
4. Applicable negative, failure, security, privacy, portability, and claim boundaries are tested or dispositioned
5. One bounded pre-PR review has no unresolved actionable findings

## Dependencies

- WP-01
- WP-05

## Inputs

- docs/milestones/v0.92/WP_ISSUE_WAVE_v0.92.yaml
- docs/milestones/v0.92/WBS_v0.92.md
- .adl/docs/TBD/workflow_tooling/planning/V0917_PROMPT_CARD_ENUM_TYPING_PLAN.md

## Non Goals

- Adjacent work packages
- Historical evidence rewriting
- Unsupported downstream milestone claims
