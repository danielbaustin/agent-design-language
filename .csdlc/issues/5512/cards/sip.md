# Structured Intent Prompt

Template: 1.0.0

Issue: 5512

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Make the bounded Runtime v3/CSM coverage route execute valid owning-crate expressions.

## Required Outcome

The exact #5504 CI expression runs ADL-only and Runtime-v3-only coverage filters and composes both summaries.

## Scope

- adl/tools/run_pr_fast_coverage_lane.sh
- adl/tools/test_run_pr_fast_coverage_lane.sh

## Authority

- Issue #5512 owns only the focused coverage split and regression proof
- Issue #5494 owns Runtime v3 and CSM implementation
- Issue #5409 owns WP-07A acceptance truth

## Assumptions

- none

## Operator Constraints

- none
