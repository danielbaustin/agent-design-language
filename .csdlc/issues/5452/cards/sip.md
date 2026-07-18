# Structured Intent Prompt

Template: 1.0.0

Issue: 5452

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Make the Spot builder-image wrapper fail closed when either primary validation or retained-summary generation fails.

## Required Outcome

Both stage statuses remain observable and focused mixed-result regressions prove that neither failure can be masked.

## Scope

- adl/tools/run_aws_spot_builder_image_validation.sh
- adl/tools/test_run_aws_spot_builder_image_validation.sh

## Authority

- GitHub issue #5452 defines the behavioral defect and scope
- The wrapper must retain the existing builder validation contract
- No runtime or CI workflow changes are authorized

## Assumptions

- none

## Operator Constraints

- none
