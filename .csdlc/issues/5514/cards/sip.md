# Structured Intent Prompt

Template: 1.0.0

Issue: 5514

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Make the bounded Runtime v3/CSM route pass the changed-source coverage gate without foreign selectors.

## Required Outcome

The exact mixed expression runs the complete valid ADL CSM selector family plus the owning Runtime v3 tests and produces an 80-percent-or-better changed-source result.

## Scope

- adl/tools/run_pr_fast_coverage_lane.sh
- adl/tools/test_run_pr_fast_coverage_lane.sh

## Authority

- Issue #5514 owns only completeness of the #5512 workspace partition
- Issue #5494 owns Runtime v3 and CSM implementation
- Issue #5409 owns WP-07A acceptance truth

## Assumptions

- none

## Operator Constraints

- none
