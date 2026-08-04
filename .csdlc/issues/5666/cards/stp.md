# Structured Task Prompt

Template: 1.0.0

Issue: 5666

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Implement, validate, review, publish, merge, and close the bounded throughput fast-lane policy issue.

## Deliverables

- docs/tooling/DEVELOPER_THROUGHPUT_FAST_LANE.md
- link from validation routing docs and selector reference in the policy
- adl/tools/test_developer_throughput_fast_lane.sh

## Acceptance

1. AC-1: The policy names proportional issue classes, eligibility, escalation, stop conditions, and non-claims.
2. AC-2: Validation routing docs link the policy, and the policy references the validation selector without editing paths claimed by another issue.
3. AC-3: The policy requires FastWork or another declared external build root when the operator requires it, and forbids silent local-disk fallback.
4. AC-4: PR watching guidance is changed-state/blocker-only and says not to wait on GitHub when no action is possible.
5. AC-5: A focused contract test proves the required policy language and links exist.

## Dependencies

- GitHub issue #5666
- existing validation lane selector
- existing validation platform routing docs

## Inputs

- docs/architecture/VALIDATION_LANE_SELECTOR.md
- docs/tooling/VALIDATION_PLATFORM_ROUTING.md
- docs/milestones/v0.91.7/review/V0917_WP06_BUILD_THROUGHPUT_VALIDATION_COST_REDUCTION_4633.md

## Non Goals

- Runtime or product feature changes
- AWS execution
- broad CI workflow rewrite
- replacement of typed C-SDLC v2
- large test-classification migration
