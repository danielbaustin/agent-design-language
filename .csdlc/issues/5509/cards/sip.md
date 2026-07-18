# Structured Intent Prompt

Template: 1.0.0

Issue: 5509

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Prove mixed Runtime v3 and CSM changes without executing unrelated Runtime v2 coverage.

## Required Outcome

A narrowly recognized mixed-crate family runs focused tests and composed coverage in each owning crate.

## Scope

- adl/tools/ci_path_policy.sh
- adl/tools/run_pr_fast_test_lane.sh
- adl/tools/run_pr_fast_coverage_lane.sh
- adl/tools/test_ci_path_policy.sh
- adl/tools/test_run_pr_fast_test_lane.sh
- adl/tools/test_run_pr_fast_coverage_lane.sh

## Authority

- Issue #5509 owns CI routing only
- Issue #5494 owns the Runtime v3 and CSM implementation
- Issue #5409 owns WP-07A acceptance truth

## Assumptions

- none

## Operator Constraints

- none
