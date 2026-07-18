# Structured Intent Prompt

Template: 1.0.0

Issue: 5467

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Make the CI backend snapshot contract reachable and behaviorally prove its routing cases using local fixtures only.

## Required Outcome

The contract reaches every backend-snapshot assertion and locally proves hosted, Spot-selected, and invalid backend inputs without AWS access.

## Scope

- adl/tools/test_run_aws_spot_ci_profile.sh
- .github/workflows/ci.yaml

## Authority

- Local shell contract and GitHub-hosted CI only

## Assumptions

- none

## Operator Constraints

- none
