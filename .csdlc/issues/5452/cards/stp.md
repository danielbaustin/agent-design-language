# Structured Task Prompt

Template: 1.0.0

Issue: 5452

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Repair only builder-image wrapper status composition and its focused regression harness.

## Deliverables

- Fail-closed wrapper status composition
- Primary-success and summary-failure regression
- Primary-failure and summary-success regression

## Acceptance

1. AC1: Primary success plus summary failure returns non-zero
2. AC2: Primary failure plus summary success returns non-zero
3. AC3: Both stages succeeding retains the successful path
4. AC4: Existing wrapper artifacts and diagnostics remain available

## Dependencies

- Existing Spot builder-image validation wrapper and focused shell harness

## Inputs

- adl/tools/run_aws_spot_builder_image_validation.sh
- adl/tools/test_run_aws_spot_builder_image_validation.sh
- GitHub issue #5452

## Non Goals

- Runtime v2 or Runtime v3 changes
- AWS topology changes
- CI workflow changes
- WP-07 hardening changes
