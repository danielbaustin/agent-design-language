# Structured Intent Prompt

Template: 1.0.0

Issue: 5506

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Map Runtime v3 API-auth source to its focused independent-crate coverage tests.

## Required Outcome

Auth-only coverage runs the Runtime v3 auth tests, while mixed selections retain both workspace runs.

## Scope

- adl/tools/check_coverage_impact.sh
- adl/tools/run_pr_fast_coverage_lane.sh
- adl/tools/test_check_coverage_impact.sh
- adl/tools/test_run_pr_fast_coverage_lane.sh

## Authority

- Issue #5506 owns only coverage tooling
- Issue #5494 retains all Runtime v3 and CSM behavior
- The existing Runtime v3 weather service remains unchanged

## Assumptions

- none

## Operator Constraints

- none
