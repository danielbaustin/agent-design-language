# Structured Intent Prompt

Template: 1.0.0

Issue: 5463

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Remove GitHub Actions Node 20 compatibility forcing while preserving immutable pinning.

## Required Outcome

Every annotated action pin uses a reviewed Node 24 revision and a hosted run contains no Node 20 deprecation annotation.

## Scope

- .github/workflows/aws-codefriend-build.yaml
- .github/workflows/aws-spot-remote-validation.yaml
- .github/workflows/ci.yaml
- .github/workflows/nightly-coverage-ratchet.yaml
- .github/workflows/v0871_milestone_closeout_gate.yaml
- adl/tools/test_ci_runtime_contracts.sh
- adl/tools/test_ci_path_policy.sh
- docs/tooling/GITHUB_ACTIONS_RUNTIME_PIN_INVENTORY.md

## Authority

- GitHub run 29632957768 is annotation evidence
- Official action repositories and immutable commit SHAs are release authority
- Repository runtime contracts preserve canonical pin policy

## Assumptions

- none

## Operator Constraints

- none
